use warp::Filter;
use std::sync::{Arc, Mutex};
use crate::blockchain::{Blockchain, Transaction};
use serde::{Deserialize};
use crate::wallet::verify_transaction;

#[derive(Debug, Deserialize)]
struct TransactionRequest {
    sender: String,
    receiver: String,
    amount: u64,
    signature: String,
}

pub async fn run_server(blockchain: Arc<Mutex<Blockchain>>) {
    let post_tx = warp::path("transaction")
        .and(warp::post())
        .and(with_blockchain(blockchain.clone()))
        .and(warp::body::json())
        .map(|blockchain: Arc<Mutex<Blockchain>>, tx_req: TransactionRequest| {
            let tx = Transaction {
                sender: tx_req.sender,
                receiver: tx_req.receiver,
                amount: tx_req.amount,
                signature: Some(tx_req.signature),
            };

            if verify_transaction(&tx) {
                blockchain.lock().unwrap().add_transaction(tx);
                warp::reply::json(&"Transaction added")
            } else {
                warp::reply::json(&"Invalid transaction")
            }
        });

    let get_chain = warp::path("chain")
        .and(warp::get())
        .and(with_blockchain(blockchain))
        .map(|blockchain: Arc<Mutex<Blockchain>>| {
            let chain = blockchain.lock().unwrap();
            warp::reply::json(&chain.chain)
        });

    let routes = post_tx.or(get_chain);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_blockchain(
    blockchain: Arc<Mutex<Blockchain>>,
) -> impl Filter<Extract = (Arc<Mutex<Blockchain>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || blockchain.clone())
}

