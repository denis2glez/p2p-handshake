//! This module implements the TLS handshake after a successful protocol negotiation.
//!
//! The `UpgradeInfo::protocol_info` method is called to determine which protocols are supported by
//! the trait implementation.
//! After a successful negotiation, the `SecurityUpgrade::upgrade_security` method is called. This
//! method will return a `Future` that performs a handshake. This handshake is considered mandatory,
//! however in practice it is possible for the trait implementation to return a dummy `Future`.

use futures::{future::BoxFuture, AsyncRead, AsyncWrite, Future};
use futures_rustls::TlsStream;
use libp2p_core::upgrade::UpgradeInfo;
use libp2p_identity::PeerId;
use rustls::{ClientConfig, ServerConfig};
use std::iter::{once, Once};

use crate::error::TlsUpgradeError;

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

impl<C> InboundSecurityUpgrade<C> for Config
where
    C: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = TlsStream<C>;
    type Error = TlsUpgradeError;
    type Future = BoxFuture<'static, Result<(PeerId, Self::Output), Self::Error>>;

    fn secure_inbound(self, socket: C, info: Self::Info, peer_id: Option<PeerId>) -> Self::Future {
        todo!()
    }
}

impl<C> OutboundSecurityUpgrade<C> for Config
where
    C: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = TlsStream<C>;
    type Error = TlsUpgradeError;
    type Future = BoxFuture<'static, Result<(PeerId, Self::Output), Self::Error>>;

    fn secure_outbound(self, socket: C, info: Self::Info, peer_id: Option<PeerId>) -> Self::Future {
        todo!()
    }
}
