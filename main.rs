use std::sync::{Arc, Mutex};
use warp::Filter;

mod blockchain;
mod wallet;
mod api;
mod p2p;

use blockchain::{Blockchain, Transaction};

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // ✅ Start peer-to-peer thread
    p2p::start_p2p(Arc::clone(&blockchain));

    // 🌐 API: GET /chain
    let chain_route = {
        let blockchain = Arc::clone(&blockchain);
        warp::path("chain").map(move || {
            let blockchain = blockchain.lock().unwrap();
            warp::reply::json(&*blockchain)
        })
    };

    // ⚒️ API: GET /mine
    let mine_route = {
        let blockchain = Arc::clone(&blockchain);
        warp::path("mine").map(move || {
            let mut blockchain = blockchain.lock().unwrap();
            blockchain.mine_pending_transactions("Arya".to_string());
            warp::reply::json(&*blockchain)
        })
    };

    // 💸 API: POST /transaction
    let tx_route = {
        let blockchain = Arc::clone(&blockchain);
        warp::path("transaction")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |tx: Transaction| {
                let mut blockchain = blockchain.lock().unwrap();
                blockchain.add_transaction(tx);
                warp::reply::json(&"Transaction added")
            })
    };

    // ✅ Merge all routes
    let routes = chain_route.or(mine_route).or(tx_route);

    let (public_key, _) = wallet::generate_wallet();
    println!("🪪 Wallet Address: {}", hex::encode(public_key.to_bytes()));
    println!("🌐 Running at http://localhost:3030");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}


