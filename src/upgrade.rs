//! This module implements the TLS handshake after a successful protocol negotiation.
//!
//! The `UpgradeInfo::protocol_info` method is called to determine which protocols are supported by
//! the trait implementation.
//! After a successful negotiation, the `InboundUpgrade::upgrade_inbound` or
//! `OutboundUpgrade::upgrade_outbound`method is called. This method will return a `Future` that
//! performs a handshake. This handshake is considered mandatory, however in practice it is possible
//! for the trait implementation to return a dummy `Future`.

use futures::{future::BoxFuture, AsyncRead, AsyncWrite};
use futures_rustls::TlsStream;
use libp2p_core::upgrade::{InboundConnectionUpgrade, OutboundConnectionUpgrade, UpgradeInfo};
use libp2p_identity::PeerId;
use std::iter::{once, Once};

#[derive(Clone)]
pub struct Config {}

#[derive(Debug)]
pub enum UpgradeError {}

impl UpgradeInfo for Config {
    type Info = &'static str;
    type InfoIter = Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        once("/tls/1.0.0")
    }
}

impl<C> InboundConnectionUpgrade<C> for Config
where
    C: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = (PeerId, TlsStream<C>);
    type Error = UpgradeError;
    type Future = BoxFuture<'static, Result<Self::Output, Self::Error>>;

    fn upgrade_inbound(self, _socket: C, _: Self::Info) -> Self::Future {
        unimplemented!()
    }
}

impl<C> OutboundConnectionUpgrade<C> for Config
where
    C: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = (PeerId, TlsStream<C>);
    type Error = UpgradeError;
    type Future = BoxFuture<'static, Result<Self::Output, Self::Error>>;

    fn upgrade_outbound(self, _socket: C, _: Self::Info) -> Self::Future {
        unimplemented!()
    }
}
