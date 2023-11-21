use clap::Parser;
use libp2p::Multiaddr;
use p2p_tls_cli::generic_p2p_node;
use tracing_subscriber::EnvFilter;

/// Simple program to perform a TLS handshake with a Celestia node.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Listening addresses.
    #[arg(short, long = "listen")]
    pub listen_addrs: Vec<Multiaddr>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let _ = dotenvy::dotenv();

    let _ = Args::parse();

    let _ = generic_p2p_node().await?;

    Ok(())
}
