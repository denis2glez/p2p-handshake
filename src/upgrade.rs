//! This module implements the TLS handshake after a successful protocol negotiation.
//!
//! The `UpgradeInfo::protocol_info` method is called to determine which protocols are supported by
//! the trait implementation.
//! After a successful negotiation, `InboundConnectionUpgrade::secure_inbound` or `OutboundSecurityUpgrade::secure_outbound`
//! method is called. This method will return a `Future` that performs a handshake. This handshake
//! is considered mandatory, however in practice it is possible for the trait implementation to return
//! a dummy `Future`.

use crate::certificate::{self, P2pCertificate};
use crate::error::TlsUpgradeError;
use futures::{future::BoxFuture, AsyncRead, AsyncWrite, Future, FutureExt};
use futures_rustls::TlsStream;
use libp2p_core::upgrade::UpgradeInfo;
use libp2p_identity::PeerId;
use rustls::{ClientConfig, ServerConfig};
use rustls::{CommonState, ServerName};
use std::net::{IpAddr, Ipv4Addr};
use std::{
    iter::{once, Once},
    sync::Arc,
};

/// Possible security upgrade on an inbound connection
pub trait InboundSecurityUpgrade<T>: UpgradeInfo {
    /// Output after the upgrade has been successfully negotiated and the handshake performed.
    type Output;
    /// Possible error during the handshake.
    type Error;
    /// Future that performs the handshake with the remote.
    type Future: Future<Output = Result<(PeerId, Self::Output), Self::Error>>;

    /// After we have determined that the remote supports one of the protocols we support, this
    /// method is called to start the handshake.
    ///
    /// The `info` is the identifier of the protocol, as produced by `protocol_info`. Security
    /// transports use the optional `peer_id` parameter on outgoing upgrades to validate the
    /// expected `PeerId`.
    fn secure_inbound(self, socket: T, info: Self::Info, peer_id: Option<PeerId>) -> Self::Future;
}

/// Possible security upgrade on an outbound connection
pub trait OutboundSecurityUpgrade<T>: UpgradeInfo {
    /// Output after the upgrade has been successfully negotiated and the handshake performed.
    type Output;
    /// Possible error during the handshake.
    type Error;
    /// Future that performs the handshake with the remote.
    type Future: Future<Output = Result<(PeerId, Self::Output), Self::Error>>;

    /// After we have determined that the remote supports one of the protocols we support, this
    /// method is called to start the handshake.
    ///
    /// The `info` is the identifier of the protocol, as produced by `protocol_info`. Security
    /// transports use the optional `peer_id` parameter on outgoing upgrades to validate the
    /// expected `PeerId`.
    fn secure_outbound(self, socket: T, info: Self::Info, peer_id: Option<PeerId>) -> Self::Future;
}

#[derive(Clone)]
pub struct Config {
    server: ServerConfig,
    client: ClientConfig,
}

impl UpgradeInfo for Config {
    type Info = &'static str;
    type InfoIter = Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        once("/tls/1.0.0")
    }
}

fn extract_single_certificate(
    state: &CommonState,
) -> Result<P2pCertificate<'_>, certificate::ParseError> {
    let Some([cert]) = state.peer_certificates() else {
        panic!("config enforces exactly one certificate");
    };

    certificate::parse(cert)
}

impl<C> InboundSecurityUpgrade<C> for Config
where
    C: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = TlsStream<C>;
    type Error = TlsUpgradeError;
    type Future = BoxFuture<'static, Result<(PeerId, Self::Output), Self::Error>>;

    fn secure_inbound(self, socket: C, _: Self::Info, _: Option<PeerId>) -> Self::Future {
        async move {
            let stream = futures_rustls::TlsAcceptor::from(Arc::new(self.server))
                .accept(socket)
                .await
                .map_err(TlsUpgradeError::ServerUpgrade)?;

            let expected = extract_single_certificate(stream.get_ref().1)?.peer_id();

            Ok((expected, stream.into()))
        }
        .boxed()
    }
}

impl<C> OutboundSecurityUpgrade<C> for Config
where
    C: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = TlsStream<C>;
    type Error = TlsUpgradeError;
    type Future = BoxFuture<'static, Result<(PeerId, Self::Output), Self::Error>>;

    fn secure_outbound(self, socket: C, _: Self::Info, peer_id: Option<PeerId>) -> Self::Future {
        async move {
            let name = ServerName::IpAddress(IpAddr::V4(Ipv4Addr::UNSPECIFIED));

            let stream = futures_rustls::TlsConnector::from(Arc::new(self.client))
                .connect(name, socket)
                .await
                .map_err(TlsUpgradeError::ClientUpgrade)?;

            let expected = extract_single_certificate(stream.get_ref().1)?.peer_id();

            match peer_id {
                Some(found) if found != expected => {
                    Err(TlsUpgradeError::PeerIdMismatch { expected, found })
                }
                _ => Ok((expected, stream.into())),
            }
        }
        .boxed()
    }
}
