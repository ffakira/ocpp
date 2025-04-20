use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionToken {
    pub transaction_id: String,
    pub payer_id: String,
    pub merchant_id: String,
    pub amount: u64,
    pub timestamp: String,
    pub nonce: String,
}
