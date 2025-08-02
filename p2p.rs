use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::Blockchain;

pub fn start_p2p(blockchain: Arc<Mutex<Blockchain>>) {
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:6000").expect("Failed to bind P2P port");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let blockchain = Arc::clone(&blockchain);
                    thread::spawn(move || handle_peer(stream, blockchain));
                }
                Err(e) => eprintln!("Connection error: {}", e),
            }
        }
    });

    println!("🕸️  P2P listening on port 6000");
}

fn handle_peer(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let chain_data = {
                let chain = blockchain.lock().unwrap();
                serde_json::to_string(&*chain).unwrap()
            };
            let _ = stream.write_all(chain_data.as_bytes());
        }
        Err(e) => {
            eprintln!("Read error: {}", e);
        }
    }
}

pub fn connect_to_peer(address: &str, blockchain: Arc<Mutex<Blockchain>>) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            let mut buffer = String::new();
            let _ = stream.read_to_string(&mut buffer);
            if let Ok(peer_chain) = serde_json::from_str::<Blockchain>(&buffer) {
                let mut chain = blockchain.lock().unwrap();
                if peer_chain.chain.len() > chain.chain.len() {
                    println!("🔗 Replacing local chain with peer's longer chain.");
                    *chain = peer_chain;
                } else {
                    println!("✅ Local chain is already up to date.");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", address, e);
        }
    }
}

