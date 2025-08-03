use tokio_tungstenite::accept_async;
use warp::Filter;
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;

pub async fn start_server(blockchain: Arc<Mutex<Blockchain>>, port: u16) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await.expect("P2P Server bind failed");

    println!("🔗 P2P Node listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let blockchain = Arc::clone(&blockchain);
        tokio::spawn(async move {
            let _ws_stream = accept_async(stream).await.expect("WebSocket error");
            // TODO: Handle P2P messaging
        });
    }
}
