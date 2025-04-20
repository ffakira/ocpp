#[derive(Debug)]
pub struct PayerLedger {
    pub balance: u64,
    pub transaction_log: Vec<String>,
}

impl PayerLedger {
    pub fn new(balance: u64) -> Self {
        Self {
            balance,
            transaction_log: Vec::new(),
        }
    }

    pub fn debit(&mut self, amount: u64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            self.transaction_log.push(format!("Debited: {}", amount));
            Ok(())
        } else {
            Err("Insufficient funds".into())
        }
    }

    pub fn log_transaction(&mut self, transaction_id: &str) {
        self.transaction_log
            .push(format!("Payer Transaction ID: {}", transaction_id));
    }
}
