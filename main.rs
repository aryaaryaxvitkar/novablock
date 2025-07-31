mod blockchain;

use blockchain::{Blockchain, Transaction};
use std::sync::{Arc, Mutex};
use warp::Filter;

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    let get_chain = {
        let blockchain = blockchain.clone();
        warp::path("chain")
            .and(warp::get())
            .map(move || {
                let bc = blockchain.lock().unwrap();
                warp::reply::json(&*bc)
            })
    };

    let post_tx = {
        let blockchain = blockchain.clone();
        warp::path("tx")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |tx: Transaction| {
                let mut bc = blockchain.lock().unwrap();
                match bc.add_transaction(tx) {
                    Ok(_) => warp::reply::json(&"Transaction added."),
                    Err(e) => warp::reply::json(&e),
                }
            })
    };

    let mine_block = {
        let blockchain = blockchain.clone();
        warp::path("mine")
            .and(warp::post())
            .map(move || {
                let mut bc = blockchain.lock().unwrap();
                bc.mine_block();
                warp::reply::json(&"Block mined.")
            })
    };

    let get_balance = {
        let blockchain = blockchain.clone();
        warp::path!("balance" / String)
            .map(move |user: String| {
                let bc = blockchain.lock().unwrap();
                let balance = bc.get_balance(&user);
                warp::reply::json(&format!("Balance of {}: {} ARX", user, balance))
            })
    };

    let routes = get_chain.or(post_tx).or(mine_block).or(get_balance);

    println!("🌐 Running at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
