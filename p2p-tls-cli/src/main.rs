use clap::Parser;
use libp2p::Multiaddr;

/// Simple program to perform a TLS handshake with a Celestia node.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Listening addresses.
    #[arg(short, long = "listen")]
    pub listen_addrs: Vec<Multiaddr>,
}

fn main() {
    let args = Args::parse();

    println!("{args:?}");
}
