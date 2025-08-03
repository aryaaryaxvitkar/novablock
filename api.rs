use warp::Filter;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use ed25519_dalek::{Signature, PublicKey};
use hex;
use crate::blockchain::{Blockchain, Transaction};
use crate::wallet::verify_transaction;

#[derive(Deserialize)]
struct TransactionRequest {
    recipient: String,
    amount: u64,
    sender: String,     // hex encoded public key
    signature: String,  // hex encoded signature
}

pub fn get_routes(blockchain: Arc<Mutex<Blockchain>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let blockchain_filter = warp::any().map(move || blockchain.clone());

    let chain_route = warp::path("chain")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .map(|blockchain: Arc<Mutex<Blockchain>>| {
            let chain = blockchain.lock().unwrap().chain.clone();
            warp::reply::json(&chain)
        });

    let balance_route = warp::path("balance")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(blockchain_filter.clone())
        .map(|params: std::collections::HashMap<String, String>, blockchain: Arc<Mutex<Blockchain>>| {
            let addr = params.get("address").cloned().unwrap_or_default();
            let balance = blockchain.lock().unwrap().get_balance(&addr);
            warp::reply::json(&balance)
        });

    let tx_route = warp::path("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .map(|tx_req: TransactionRequest, blockchain: Arc<Mutex<Blockchain>>| {
            // Decode hex to raw bytes
            let signature_bytes = hex::decode(&tx_req.signature).expect("Invalid signature hex");
            let public_key_bytes = hex::decode(&tx_req.sender).expect("Invalid public key hex");

            // Convert to crypto objects
            let signature = Signature::from_bytes(&signature_bytes).expect("Invalid signature");
            let public_key = PublicKey::from_bytes(&public_key_bytes).expect("Invalid public key");

            // Build transaction
            let tx = Transaction {
                recipient: tx_req.recipient,
                amount: tx_req.amount,
                sender: public_key.clone(),
                signature: signature.clone(),
            };

            // Verify and add to blockchain
            if verify_transaction(&tx, &signature, &public_key) {
                blockchain.lock().unwrap().add_transaction(tx);
                warp::reply::json(&"Transaction added")
            } else {
                warp::reply::json(&"Invalid transaction signature")
            }
        });

    chain_route.or(balance_route).or(tx_route)
}

/// 🔥 This function starts the HTTP server at localhost:3030
pub async fn start_server(blockchain: Arc<Mutex<Blockchain>>) {
    let routes = get_routes(blockchain);
    println!("🌐 HTTP server running on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
