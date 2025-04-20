pub mod merkle;

use std::{fs, path::Path};

// use chrono::Utc;
// use hex;
// use hkdf::{Hkdf, hmac};
// use hmac::{Hmac, Mac};
use lmdb::{DatabaseFlags, Environment, Transaction, WriteFlags};
// use merkle::{MerkleTree, hash_leaf};
// use rand::rngs::OsRng;
// use serde::{Deserialize, Serialize};
// use sha2::Sha256;
// use uuid::Uuid;
// use x25519_dalek::{EphemeralSecret, PublicKey};

// type HmacSha256 = Hmac<Sha256>;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct TransactionToken {
//     transaction_id: String,
//     payer_id: String,
//     merchant_id: String,
//     amount: u64,
//     timestamp: String,
//     nonce: String,
// }

// #[derive(Debug)]
// struct LocalLedger {
//     balance: u64,
//     transaction_log: Vec<String>,
// }

// impl LocalLedger {
//     fn new(balance: u64) -> Self {
//         LocalLedger {
//             balance,
//             transaction_log: Vec::new(),
//         }
//     }

//     fn debit(&mut self, amount: u64) -> Result<(), String> {
//         if self.balance >= amount {
//             self.balance -= amount;
//             self.transaction_log.push(format!("Debited: {}", amount));
//             Ok(())
//         } else {
//             Err("Insufficient funds".to_string())
//         }
//     }

//     fn log_transaction(&mut self, transaction_id: &str) {
//         self.transaction_log
//             .push(format!("Transaction ID: {}", transaction_id));
//     }
// }

// #[derive(Debug)]
// struct TerminalLedger {
//     received_amount: u64,
//     transaction_log: Vec<String>,
// }

// impl TerminalLedger {
//     fn new() -> Self {
//         TerminalLedger {
//             received_amount: 0,
//             transaction_log: Vec::new(),
//         }
//     }

//     fn credit(&mut self, amount: u64) {
//         self.received_amount += amount;
//     }

//     fn log_transaction(&mut self, transaction_id: &str) {
//         self.transaction_log
//             .push(format!("Transaction ID: {}", transaction_id));
//     }
// }

// // Derive shared secret via X25519 Diffie-Hellman
// fn derive_shared_secret(own_secret: EphemeralSecret, peer_public: &PublicKey) -> [u8; 32] {
//     own_secret.diffie_hellman(peer_public).to_bytes()
// }

// // Derive session key from shared secret using HKDF
// fn derive_session_key(shared_secret: &[u8]) -> [u8; 32] {
//     let hkdf = Hkdf::<Sha256>::new(None, shared_secret);
//     let mut session_key = [0u8; 32];
//     hkdf.expand(&[], &mut session_key)
//         .expect("Failed to derive session key");
//     session_key
// }

// fn sign_transaction_token(session_key: &[u8; 32], token: &TransactionToken) -> Vec<u8> {
//     let token_bytes = serde_json::to_vec(token).expect("Failed to serialize token");
//     let mut mac = HmacSha256::new_from_slice(session_key).expect("HMAC can take key of any size");
//     mac.update(&token_bytes);
//     mac.finalize().into_bytes().to_vec()
// }

// fn verify_transaction_token(
//     session_key: &[u8; 32],
//     token: &TransactionToken,
//     signature: &[u8],
// ) -> bool {
//     let token_bytes = serde_json::to_vec(token).expect("Failed to serialize token");
//     let mut mac = HmacSha256::new_from_slice(session_key).expect("HMAC can take key of any size");
//     mac.update(&token_bytes);
//     mac.verify_slice(signature).is_ok()
// }

// fn process_transaction(
//     token: &TransactionToken,
//     signature: &[u8],
//     session_key: &[u8; 32],
//     terminal_ledger: &mut TerminalLedger,
// ) {
//     if verify_transaction_token(session_key, token, signature) {
//         terminal_ledger.credit(token.amount);
//         terminal_ledger.log_transaction(&token.transaction_id);
//         println!("Transaction verified and processed by terminal.");
//     } else {
//         println!("Transaction verification failed.");
//     }
// }

// fn main() {
//     println!("=== mobile + terminal ephemeral key exchange ===");

//     // Mobile generates ephemeral key pair
//     let mobile_secret = EphemeralSecret::random_from_rng(&mut OsRng);
//     let mobile_public = PublicKey::from(&mobile_secret);

//     // Terminal generates ephemeral key pair
//     let terminal_secret = EphemeralSecret::random_from_rng(&mut OsRng);
//     let terminal_public = PublicKey::from(&terminal_secret);

//     // Derive shared secrets
//     let mobile_shared_secret = derive_shared_secret(mobile_secret, &terminal_public);
//     let terminal_shared_secret = derive_shared_secret(terminal_secret, &mobile_public);

//     assert_eq!(mobile_shared_secret, terminal_shared_secret);
//     println!("Shared secret successfully established.");

//     // Derive session key from shared secret
//     let session_key = derive_session_key(&mobile_shared_secret);
//     println!("Session key derived: {:?}", session_key);

//     // Create transaction token
//     let token = TransactionToken {
//         transaction_id: Uuid::new_v4().to_string(),
//         payer_id: "payer001".to_string(),
//         merchant_id: "merchant001".to_string(),
//         amount: 1000,
//         timestamp: Utc::now().to_rfc3339(),
//         nonce: Uuid::new_v4().to_string(),
//     };

//     // Mobile signs token
//     let signature = sign_transaction_token(&session_key, &token);
//     println!("Transaction token signed.");

//     // Mobile debits amount
//     let mut mobile_ledger = LocalLedger::new(10000);
//     if let Err(err) = mobile_ledger.debit(token.amount) {
//         println!("Error: {}", err);
//     } else {
//         println!("Mobile debited successfully.");
//     }
//     mobile_ledger.log_transaction(&token.transaction_id);

//     // Terminal processes transaction
//     let mut terminal_ledger = TerminalLedger::new();
//     process_transaction(&token, &signature, &session_key, &mut terminal_ledger);

//     // Final ledgers
//     println!("\n=== Final Ledgers ===");
//     println!("Mobile Ledger: {:?}", mobile_ledger);
//     println!("Terminal Ledger: {:?}", terminal_ledger);
// }

fn main() {
    let path = Path::new("./db");
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }

    // Create and open a new LMDB environment
    let env = Environment::new()
        .set_flags(lmdb::EnvironmentFlags::NO_SYNC) // Optional: Disable syncing for performance
        .set_max_dbs(1)
        .open(Path::new("./db")) // Open a database folder called "db"
        .unwrap();

    // Create a new database inside the environment (if it doesn't exist)
    let db = env
        .create_db(Some("transactions"), DatabaseFlags::empty())
        .unwrap();

    // Example data to store: Merkle proof or balance
    let transaction_id = "5e99d625-ea55-4377-86b8-d6540c41b362".to_string();
    let balance: u64 = 10000;

    // Open a write transaction
    let mut txn = env.begin_rw_txn().unwrap();

    // Put some data in the database (store transaction data)
    txn.put(
        db,
        &transaction_id,
        &(balance as u64).to_le_bytes(),
        WriteFlags::empty(),
    )
    .unwrap();

    // Commit the transaction
    txn.commit().unwrap();

    // Retrieve the data from the database to verify it
    let txn = env.begin_ro_txn().unwrap();
    let retrieved_balance: Option<Vec<u8>> = txn.get(db, &transaction_id).ok().map(|v| v.to_vec());

    match retrieved_balance {
        Some(balance) => {
            let balance = u64::from_le_bytes(balance.try_into().unwrap());
            println!("Retrieved Balance: {}", balance);
        }
        None => println!("Transaction ID not found in database!"),
    }

    // Clean up (Environment will be automatically cleaned up when it goes out of scope)
}
