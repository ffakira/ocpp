use lmdb::{Database, DatabaseFlags, Environment, Transaction, WriteFlags};
use std::path::Path;

pub struct Storage {
    env: Environment,
    db: Database,
}

impl Storage {
    pub fn new(path: &Path) -> Self {
        std::fs::create_dir_all(path).unwrap();
        let env = Environment::new().set_max_dbs(1).open(path).unwrap();
        let db = env
            .create_db(Some("transactions"), DatabaseFlags::empty())
            .unwrap();

        Self { env, db }
    }

    pub fn store(&self, key: &[u8], value: &[u8]) {
        let mut txn = self.env.begin_rw_txn().unwrap();
        txn.put(self.db, &key, &value, WriteFlags::empty()).unwrap();
        txn.commit().unwrap();
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let txn = self.env.begin_ro_txn().unwrap();
        txn.get(self.db, &key).ok().map(|v| v.to_vec())
    }
}
