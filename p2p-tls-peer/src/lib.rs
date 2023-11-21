use celestia_types::p2p;
use futures::StreamExt;

use libp2p::{
    swarm::{dummy, SwarmEvent},
    tcp, tls, yamux, SwarmBuilder,
};
use tokio::{
    sync::mpsc,
    task,
    time::{sleep, Duration},
};

/// Creates a generic P2P node that uses the TLS handshake implemented in the `p2p-tls-handshake` crate.
pub async fn generic_p2p_node() -> anyhow::Result<p2p::AddrInfo> {
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            // TODO: The goal is to replace the following with p2p_tls_handshake::Config::new
            // The latter is being merged upstream in https://github.com/libp2p/rust-libp2p/pull/4864
            // where at the moment there are only some details to complete.
            // Initially, the implementation of the TLS handshake was mainly reduced to defining the
            // Config representation (using rustls) and implementing the InboundConnectionUpgrade
            // and OutboundConnectionUpgrade traits for this struct (see p2p_tls_handshake::Config).
            tls::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| dummy::Behaviour)?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_millis(500)))
        .build();

    let local_peer_id = *swarm.local_peer_id();
    tracing::info!("→ Local peer id: {local_peer_id:?}");

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let (addr_tx, mut addr_rx) = mpsc::channel(32);

    task::spawn(async move {
        loop {
            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    tracing::info!("→ Listening on {address:?}");
                    if addr_tx.send(address).await.is_err() {
                        tracing::warn!("→ Received new address after set startup time");
                    }
                }
                SwarmEvent::Behaviour(event) => tracing::info!("→ {event:?}"),
                other => {
                    tracing::debug!("→ Unhandled {:?}", other);
                }
            }
        }
    });

    sleep(Duration::from_millis(500)).await;
    addr_rx.close();

    let mut addrs = vec![];
    while let Some(addr) = addr_rx.recv().await {
        addrs.push(addr);
    }

    let addr = p2p::AddrInfo {
        id: p2p::PeerId(local_peer_id),
        addrs,
    };

    Ok(addr)
}
