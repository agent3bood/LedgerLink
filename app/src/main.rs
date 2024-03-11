use ledger::chain::Chain;
use network::Network;
use node::node::Node;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let public_key = env::var("KEY_PUB").expect("KEY_PUB must be set");
    let private_key = env::var("KEY_PRIV").expect("KEY_PRIV must be set");

    let (network_tx, mut network_rx) = tokio::sync::mpsc::channel(100);
    let (node_tx, mut node_rx) = tokio::sync::mpsc::channel(100);

    let chain = Chain::new();
    let node = Node::new(public_key, private_key, chain, node_rx, network_tx)?;
    let network = Network::new(node.id.clone(), network_rx, node_tx);

    let _ = tokio::join!(node.run(), network.run());

    Ok(())
}
