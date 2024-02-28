use network::Network;
use node::node::Node;
use std::{env, io::Error as IoError};

#[tokio::main]
async fn main() -> Result<(), IoError> {
    dotenv::dotenv().ok();

    let public_key = env::var("KEY_PUB").expect("KEY_PUB must be set");
    let private_key = env::var("KEY_PRIV").expect("KEY_PRIV must be set");

    let node = Node::new(public_key, private_key);

    let network = Network::new(&node);
    let _ = network.run().await;

    Ok(())
}
