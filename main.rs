mod blockchain;
mod wallet;
mod api;
mod p2p;

use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;
use crate::wallet::generate_wallet;

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    let (public_key, _) = generate_wallet();
    println!("🪪 Wallet Address: {:?}", public_key);

    let p2p_blockchain = Arc::clone(&blockchain);
    tokio::spawn(async move {
        p2p::start_server(p2p_blockchain, 6000).await;
    });

    api::run_server(blockchain).await;
}
