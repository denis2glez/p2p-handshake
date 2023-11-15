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

/// Possible security upgrade on a connection
pub trait SecurityUpgrade<T>: UpgradeInfo {
    /// Output after the upgrade has been successfully negotiated and the handshake performed.
    type Output;
    /// Possible error during the handshake.
    type Error;
    /// Future that performs the handshake with the remote.
    type Future: Future<Output = Result<Self::Output, Self::Error>>;

    /// After we have determined that the remote supports one of the protocols we support, this
    /// method is called to start the handshake.
    ///
    /// The `info` is the identifier of the protocol, as produced by `protocol_info`. Security
    /// transports use the optional `peer_id` parameter on outgoing upgrades to validate the
    /// expected `PeerId`.
    fn upgrade_security(self, socket: T, info: Self::Info, peer_id: Option<PeerId>)
        -> Self::Future;
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

impl<C> SecurityUpgrade<C> for Config
where
    C: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = (PeerId, TlsStream<C>);
    type Error = TlsUpgradeError;
    type Future = BoxFuture<'static, Result<Self::Output, Self::Error>>;

    fn upgrade_security(
        self,
        socket: C,
        info: Self::Info,
        peer_id: Option<PeerId>,
    ) -> Self::Future {
        unimplemented!()
    }
}
