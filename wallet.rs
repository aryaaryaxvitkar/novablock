use ed25519_dalek::{Keypair, Signature, Signer, Verifier, PublicKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use hex;

use crate::blockchain::Transaction;

pub fn generate_wallet() -> (Keypair, String) {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let address = hex::encode(keypair.public.as_bytes());
    (keypair, address)
}

pub fn sign_transaction(tx: &Transaction, sender_keypair: &Keypair) -> Signature {
    let tx_data = serde_json::to_vec(tx).expect("Failed to serialize transaction");
    sender_keypair.sign(&tx_data)
}

pub fn verify_transaction(tx: &Transaction, signature: &Signature, public_key: &PublicKey) -> bool {
    let tx_data = serde_json::to_vec(tx).expect("Failed to serialize transaction");
    public_key.verify(&tx_data, signature).is_ok()
}
