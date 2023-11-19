use crate::certificate::{GenError, ParseError};
use libp2p_identity::PeerId;
use multistream_select::NegotiationError;
use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum TlsUpgradeError {
    #[error("Failed to generate certificate")]
    CertificateGeneration(#[from] GenError),
    #[error("Failed to upgrade server connection")]
    ServerUpgrade(std::io::Error),
    #[error("Failed to upgrade client connection")]
    ClientUpgrade(std::io::Error),
    #[error("Failed to parse certificate")]
    BadCertificate(#[from] ParseError),
    #[error("Invalid peer ID (expected {expected:?}, found {found:?})")]
    PeerIdMismatch { expected: PeerId, found: PeerId },
}

/// Error that can happen when upgrading a connection or substream to use a protocol.
#[derive(Debug)]
pub enum UpgradeError<E> {
    /// Error during the negotiation process.
    Select(NegotiationError),
    /// Error during the post-negotiation handshake.
    Apply(E),
}

impl<E> UpgradeError<E> {
    pub fn map_err<F, T>(self, f: F) -> UpgradeError<T>
    where
        F: FnOnce(E) -> T,
    {
        match self {
            UpgradeError::Select(e) => UpgradeError::Select(e),
            UpgradeError::Apply(e) => UpgradeError::Apply(f(e)),
        }
    }

    pub fn into_err<T>(self) -> UpgradeError<T>
    where
        T: From<E>,
    {
        self.map_err(Into::into)
    }
}

impl<E> fmt::Display for UpgradeError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpgradeError::Select(_) => write!(f, "Multistream select failed"),
            UpgradeError::Apply(_) => write!(f, "Handshake failed"),
        }
    }
}

impl<E> std::error::Error for UpgradeError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UpgradeError::Select(e) => Some(e),
            UpgradeError::Apply(e) => Some(e),
        }
    }
}

impl<E> From<NegotiationError> for UpgradeError<E> {
    fn from(e: NegotiationError) -> Self {
        UpgradeError::Select(e)
    }
}
