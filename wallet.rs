use ed25519_dalek::{Keypair, Signature, Signer, Verifier, PublicKey};
use rand::rngs::OsRng;
use crate::blockchain::Transaction;
use hex;

pub fn generate_wallet() -> (String, Keypair) {
    let keypair: Keypair = Keypair::generate(&mut OsRng);
    let public_key = hex::encode(keypair.public.to_bytes());
    (public_key, keypair)
}

pub fn sign_transaction(tx: &Transaction, sender_keypair: &Keypair) -> Signature {
    let message = format!("{}{}{}", tx.sender, tx.receiver, tx.amount);
    sender_keypair.sign(message.as_bytes())
}

pub fn verify_transaction(tx: &Transaction) -> bool {
    let message = format!("{}{}{}", tx.sender, tx.receiver, tx.amount);
    let public_key_bytes = match hex::decode(&tx.sender) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let public_key = match PublicKey::from_bytes(&public_key_bytes) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    let signature_bytes = match &tx.signature {
        Some(sig) => match hex::decode(sig) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        },
        None => return false,
    };

    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    public_key.verify(message.as_bytes(), &signature).is_ok()
}
