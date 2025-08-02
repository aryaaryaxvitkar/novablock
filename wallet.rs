use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng; // ✅ FIX: Correct RNG from rand 0.7

pub fn generate_wallet() -> (PublicKey, Keypair) {
    let mut csprng = OsRng {}; // ✅ FIX: OsRng used as struct, not function
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public;

    (public_key, keypair)
}
