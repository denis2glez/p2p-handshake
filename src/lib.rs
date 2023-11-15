mod certificate;
mod error;
mod upgrade;
mod verifier;

use certificate::GenError;
use libp2p_identity::Keypair;
use libp2p_identity::PeerId;
use rustls::ClientConfig;
use rustls::ServerConfig;
use std::sync::Arc;
use verifier::Libp2pCertificateVerifier;

pub use futures_rustls::TlsStream;
pub use upgrade::Config;

const P2P_ALPN: &[u8] = b"libp2p";

/// Create a TLS client configuration for libp2p.
pub fn make_client_config(
    keypair: &Keypair,
    remote_peer_id: Option<PeerId>,
) -> Result<ClientConfig, GenError> {
    let (certificate, private_key) = certificate::generate(keypair)?;

    let mut crypto = ClientConfig::builder()
        .with_cipher_suites(verifier::CIPHERSUITES)
        .with_safe_default_kx_groups()
        .with_protocol_versions(verifier::PROTOCOL_VERSIONS)
        .expect("Cipher suites and kx groups are configured.")
        .with_custom_certificate_verifier(Arc::new(Libp2pCertificateVerifier::with_remote_peer_id(
            remote_peer_id,
        )))
        .with_client_auth_cert(vec![certificate], private_key)
        .expect("Client cert key DER is valid.");
    crypto.alpn_protocols = vec![P2P_ALPN.to_vec()];

    Ok(crypto)
}

/// Create a TLS server configuration for libp2p.
pub fn make_server_config(keypair: &Keypair) -> Result<ServerConfig, GenError> {
    let (certificate, private_key) = certificate::generate(keypair)?;

    let mut crypto = ServerConfig::builder()
        .with_cipher_suites(verifier::CIPHERSUITES)
        .with_safe_default_kx_groups()
        .with_protocol_versions(verifier::PROTOCOL_VERSIONS)
        .expect("Cipher suites and kx groups are configured.")
        .with_client_cert_verifier(Arc::new(Libp2pCertificateVerifier::new()))
        .with_single_cert(vec![certificate], private_key)
        .expect("Server cert key DER is valid.");
    crypto.alpn_protocols = vec![P2P_ALPN.to_vec()];

    Ok(crypto)
}
