use ledger::chain::Chain;
use network::Network;
use node::node::Node;
use std::env;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let public_key = env::var("KEY_PUB").expect("KEY_PUB must be set");
    let private_key = env::var("KEY_PRIV").expect("KEY_PRIV must be set");

    let chain: Chain = Chain::new();
    let node = Node::new(public_key, private_key, Arc::new(Mutex::new(chain)))?;
    let network = Network::new(Arc::new(Mutex::new(node)));
    let _ = network.run().await;

    Ok(())
}
