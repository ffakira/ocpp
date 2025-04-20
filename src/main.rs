mod crypto;
mod ledger;
mod merkle;
mod storage;
mod transaction;

use chrono::Utc;
use hkdf::Hkdf;
use ledger::{PayerLedger, TerminalLedger};
use merkle::{MerkleTree, hash_leaf};
use sha2::Sha256;
use std::path::Path;
use storage::Storage;
use transaction::TransactionToken;
use uuid::Uuid;
use x25519_dalek::{EphemeralSecret, PublicKey};

fn derive_shared_secret(secret: EphemeralSecret, peer_public: &PublicKey) -> [u8; 32] {
    secret.diffie_hellman(peer_public).to_bytes()
}

fn derive_session_key(shared_secret: &[u8]) -> [u8; 32] {
    let hkdf = Hkdf::<Sha256>::new(None, shared_secret);
    let mut okm = [0u8; 32];
    hkdf.expand(&[], &mut okm).unwrap();
    okm
}

fn main() {
    let mut rng = rand::rngs::OsRng;

    let mobile_secret = EphemeralSecret::random_from_rng(&mut rng);
    let terminal_secret = EphemeralSecret::random_from_rng(&mut rng);
    let terminal_public = PublicKey::from(&terminal_secret);
    let shared_secret = derive_shared_secret(mobile_secret, &terminal_public);
    let session_key = derive_session_key(&shared_secret);

    let mut payer_ledger = PayerLedger::new(5000);
    let mut terminal_ledger = TerminalLedger::new();

    let storage = Storage::new(Path::new("./db"));

    let token = TransactionToken {
        transaction_id: Uuid::new_v4().to_string(),
        payer_id: "payer001".into(),
        merchant_id: "merchant001".into(),
        amount: 1000,
        timestamp: Utc::now().to_rfc3339(),
        nonce: Uuid::new_v4().to_string(),
    };

    if let Err(e) = payer_ledger.debit(token.amount) {
        println!("Transaction failed: {}", e);
        return;
    }

    terminal_ledger.credit(token.amount);

    payer_ledger.log_transaction(&token.transaction_id);
    terminal_ledger.log_transaction(&token.transaction_id);

    let leaf = hash_leaf(&format!("{}:{}", token.payer_id, token.amount));
    let leaves = vec![leaf.clone()]; // for now only one txn, could batch multiple

    let mut merkle_tree = MerkleTree::new(&leaves);
    let proof = merkle_tree.get_proof(0);

    // Verify the proof before tampering
    let is_valid_before = MerkleTree::verify_proof(&leaf, &proof, &merkle_tree.root);
    println!("Proof verified before tampering: {}", is_valid_before);

    // Tamper with the tree by modifying the root
    merkle_tree.root = vec![0u8; 32]; // Set the root to an invalid value

    // Verify the proof after tampering
    let is_valid_after = MerkleTree::verify_proof(&leaf, &proof, &merkle_tree.root);
    println!("Proof verified after tampering: {}", is_valid_after);

    let token_bytes = serde_json::to_vec(&token).unwrap();
    let (ciphertext, nonce) = crypto::encrypt(&token_bytes, &session_key);

    let mut value = Vec::from(nonce);
    value.extend_from_slice(&ciphertext);
    storage.store(token.transaction_id.as_bytes(), &value);

    let proof_key = format!("proof_{}", token.transaction_id);
    let proof_serialized = proof
        .iter()
        .map(|(hash, is_right)| format!("{}:{}", hex::encode(hash), is_right))
        .collect::<Vec<_>>()
        .join("|");
    storage.store(proof_key.as_bytes(), proof_serialized.as_bytes());

    // Done — later settlement server sync:
    println!("Stored transaction: {}", token.transaction_id);
    println!("Stored Merkle root: {}", hex::encode(merkle_tree.root));
}
