#[derive(Debug)]
pub struct TerminalLedger {
    pub received_amount: u64,
    pub transaction_log: Vec<String>,
}

impl TerminalLedger {
    pub fn new() -> Self {
        Self {
            received_amount: 0,
            transaction_log: Vec::new(),
        }
    }

    pub fn credit(&mut self, amount: u64) {
        self.received_amount += amount;
    }

    pub fn log_transaction(&mut self, transaction_id: &str) {
        self.transaction_log
            .push(format!("Transaction ID: {}", transaction_id));
    }
}
