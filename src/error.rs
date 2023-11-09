use crate::certificate::{GenError, ParseError};

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
}
