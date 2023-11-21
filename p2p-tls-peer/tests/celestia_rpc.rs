//! This module is part of the RPC test suit in https://github.com/eigerco/celestia-node-rs.

use crate::utils::client::{new_test_client, AuthLevel};
use celestia_rpc::prelude::*;
use celestia_types::p2p;
use p2p_tls_peer::generic_p2p_node;
use tokio::time::{sleep, Duration};

pub mod utils;

#[tokio::test]
async fn add_remove_peer_test() {
    // add and then remove a peer, testing outputs from `p2p.Peers` and `p2p.Connectedness`
    let addr_info = generic_p2p_node()
        .await
        .expect("failed to spin up second node");
    let client = new_test_client(AuthLevel::Admin).await.unwrap();

    let initial_peers = client
        .p2p_peers()
        .await
        .expect("failed to get initial peer list");
    assert!(!initial_peers.contains(&addr_info.id));

    let connected_to_peer = client
        .p2p_connectedness(&addr_info.id)
        .await
        .expect("failed to check initial connection to peer");
    assert_eq!(connected_to_peer, p2p::Connectedness::NotConnected);

    client
        .p2p_connect(&addr_info)
        .await
        .expect("request to connect to second node failed");
    rpc_call_delay().await;

    let peers = client
        .p2p_peers()
        .await
        .expect("failed to get peer list after connect request");
    assert!(peers.contains(&addr_info.id));

    let connected_to_peer = client
        .p2p_connectedness(&addr_info.id)
        .await
        .expect("failed to check connection to peer after connect request");
    assert_eq!(connected_to_peer, p2p::Connectedness::Connected);

    client
        .p2p_close_peer(&addr_info.id)
        .await
        .expect("Failed to close peer");
    rpc_call_delay().await;

    let final_peers = client
        .p2p_peers()
        .await
        .expect("failed to get peer list after close peer request");
    assert!(!final_peers.contains(&addr_info.id));
}

#[tokio::test]
async fn protect_unprotect_test() {
    // check whether reported protect status reacts correctly to protect/unprotect requests and
    // whether node takes tag into the account

    const PROTECT_TAG: &str = "test-tag";
    const ANOTHER_PROTECT_TAG: &str = "test-tag-2";

    let addr_info = generic_p2p_node()
        .await
        .expect("failed to spin up second node");
    let client = new_test_client(AuthLevel::Admin).await.unwrap();

    client
        .p2p_connect(&addr_info)
        .await
        .expect("request to connect to second node failed");
    rpc_call_delay().await;

    let is_protected = client
        .p2p_is_protected(&addr_info.id, PROTECT_TAG)
        .await
        .expect("failed to check initial protect status");
    assert!(!is_protected);

    client
        .p2p_protect(&addr_info.id, PROTECT_TAG)
        .await
        .expect("protect request failed");
    rpc_call_delay().await;

    let is_protected = client
        .p2p_is_protected(&addr_info.id, PROTECT_TAG)
        .await
        .expect("failed to check protect status after protect request");
    assert!(is_protected);

    let is_protected_another_tag = client
        .p2p_is_protected(&addr_info.id, ANOTHER_PROTECT_TAG)
        .await
        .expect("failed to check protect status for another tag after protect request");
    assert!(!is_protected_another_tag);

    client
        .p2p_unprotect(&addr_info.id, PROTECT_TAG)
        .await
        .expect("unprotect request failed");
    rpc_call_delay().await;

    let is_protected = client
        .p2p_is_protected(&addr_info.id, PROTECT_TAG)
        .await
        .expect("failed to check protect status after unprotect reqest");
    assert!(!is_protected);
}

#[tokio::test]
async fn peer_block_unblock_test() {
    let addr_info = generic_p2p_node()
        .await
        .expect("failed to spin up second node");
    let client = new_test_client(AuthLevel::Admin).await.unwrap();

    let blocked_peers = client
        .p2p_list_blocked_peers()
        .await
        .expect("failed to get blocked peer list");
    assert!(!blocked_peers.contains(&addr_info.id));

    client
        .p2p_block_peer(&addr_info.id)
        .await
        .expect("failed to block peer");
    rpc_call_delay().await;

    let blocked_peers = client
        .p2p_list_blocked_peers()
        .await
        .expect("failed to get blocked peer list");
    assert!(blocked_peers.contains(&addr_info.id));

    client
        .p2p_unblock_peer(&addr_info.id)
        .await
        .expect("failed to block peer");
    rpc_call_delay().await;

    let blocked_peers = client
        .p2p_list_blocked_peers()
        .await
        .expect("failed to get blocked peer list");
    assert!(!blocked_peers.contains(&addr_info.id));
}

#[tokio::test]
async fn peer_info_test() {
    let addr_info = generic_p2p_node()
        .await
        .expect("failed to spin up second node");
    let client = new_test_client(AuthLevel::Admin).await.unwrap();

    client
        .p2p_connect(&addr_info)
        .await
        .expect("request to connect to second node failed");
    rpc_call_delay().await;

    let connectedness = client
        .p2p_connectedness(&addr_info.id)
        .await
        .expect("failed to check connection to peer after connect request");
    assert_eq!(connectedness, p2p::Connectedness::Connected);

    let peer_info = client
        .p2p_peer_info(&addr_info.id)
        .await
        .expect("failed to get peer info");

    assert_eq!(addr_info.id, peer_info.id);
}

async fn rpc_call_delay() {
    // delay for RPC calls like connect/close to let node finish the operation before we query it
    // again. Below 150 ms I start getting intermittent failures.
    sleep(Duration::from_millis(150)).await;
}
