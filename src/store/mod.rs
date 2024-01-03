use self::in_memory::Store as InMemoryStore;
use self::rocksdb::Store as RocksDBStore;
use self::sled::Store as SledStore;
use anyhow::Result;
use cairo_felt::Felt252;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::rpc::{MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, Transaction};

pub mod in_memory;
pub mod rocksdb;
pub mod sled;

pub(crate) type Key = Vec<u8>;
pub(crate) type Value = Vec<u8>;

const BLOCK_HEIGHT: &str = "height";
pub trait StoreEngine: Debug + Send {
    fn add_transaction(&mut self, transaction: Transaction) -> Result<()>;
    fn get_transaction(&self, tx_hash: Felt252) -> Result<Option<Transaction>>;
    fn add_block(&mut self, block: MaybePendingBlockWithTxs) -> Result<()>;
    fn get_block_by_hash(&self, block_hash: Felt252) -> Result<Option<MaybePendingBlockWithTxs>>;
    fn get_block_by_height(&self, block_height: u64) -> Result<Option<MaybePendingBlockWithTxs>>;
    fn set_value(&mut self, key: Key, value: Value) -> Result<()>;
    fn get_value(&self, key: Key) -> Result<Option<Value>>;
    fn add_transaction_receipt(
        &mut self,
        transaction_receipt: MaybePendingTransactionReceipt,
    ) -> Result<()>;
    fn get_transaction_receipt(
        &self,
        transaction_id: Felt252,
    ) -> Result<Option<MaybePendingTransactionReceipt>>;
}

#[derive(Debug, Clone)]
pub struct Store {
    engine: Arc<Mutex<dyn StoreEngine>>,
}

#[allow(dead_code)]
pub enum EngineType {
    RocksDB,
    Sled,
    InMemory,
}

impl Store {
    pub fn new(path: &str, engine_type: EngineType) -> Result<Self> {
        let mut store = match engine_type {
            EngineType::RocksDB => Self {
                engine: Arc::new(Mutex::new(
                    RocksDBStore::new(&format!("{path}.rocksdb"))
                        .expect("could not create rocksdb store"),
                )),
            },
            EngineType::Sled => Self {
                engine: Arc::new(Mutex::new(SledStore::new(&format!("{path}.sled"))?)),
            },
            EngineType::InMemory => Self {
                engine: Arc::new(Mutex::new(InMemoryStore::new()?)),
            },
        };
        store.init();
        Ok(store)
    }

    fn init(&mut self) {
        if self.get_height().is_none() {
            _ = self.set_height(0);
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<()> {
        self.engine
            .clone()
            .lock()
            .unwrap()
            .add_transaction(transaction)
    }

    pub fn get_transaction(&self, tx_hash: Felt252) -> Result<Option<Transaction>> {
        self.engine.clone().lock().unwrap().get_transaction(tx_hash)
    }

    pub fn add_block(&mut self, block: MaybePendingBlockWithTxs) -> Result<()> {
        self.engine.clone().lock().unwrap().add_block(block)
    }

    pub fn get_block_by_height(
        &self,
        block_height: u64,
    ) -> Result<Option<MaybePendingBlockWithTxs>> {
        self.engine
            .clone()
            .lock()
            .unwrap()
            .get_block_by_height(block_height)
    }

    pub fn get_block_by_hash(
        &self,
        block_hash: Felt252,
    ) -> Result<Option<MaybePendingBlockWithTxs>> {
        self.engine
            .clone()
            .lock()
            .unwrap()
            .get_block_by_hash(block_hash)
    }

    pub fn set_height(&mut self, value: u64) -> Result<()> {
        self.engine
            .clone()
            .lock()
            .unwrap()
            .set_value(BLOCK_HEIGHT.into(), value.to_be_bytes().to_vec())
    }

    pub fn get_height(&self) -> Option<u64> {
        self.engine
            .clone()
            .lock()
            .unwrap()
            .get_value(BLOCK_HEIGHT.into())
            .map_or(None, |result| {
                result.map(|value| u64::from_be_bytes(value.as_slice()[..8].try_into().unwrap()))
            })
    }

    pub fn add_transaction_receipt(
        &mut self,
        transaction_receipt: MaybePendingTransactionReceipt,
    ) -> Result<()> {
        self.engine
            .clone()
            .lock()
            .unwrap()
            .add_transaction_receipt(transaction_receipt)
    }

    pub fn get_transaction_receipt(
        &self,
        transaction_id: Felt252,
    ) -> Result<Option<MaybePendingTransactionReceipt>> {
        self.engine
            .clone()
            .lock()
            .unwrap()
            .get_transaction_receipt(transaction_id)
    }
}

#[cfg(test)]
mod tests {
    use starknet_core::types::FieldElement;
    use super::*;
    use std::{env, fs};
    use crate::rpc::{InvokeTransaction, InvokeTransactionV1,
        serializable_types::{to_felt252, to_field_element},
    };

    #[test]
    fn test_in_memory_store() {
        let store = Store::new("test", EngineType::InMemory).unwrap();
        test_store_tx(store.clone());
        test_store_height(store);
    }

    #[test]
    fn test_sled_store() {
        // Removing preexistent DBs in case of a failed previous test
        remove_test_dbs("test.sled.");
        let store = Store::new("test", EngineType::Sled).unwrap();
        test_store_tx(store.clone());
        test_store_height(store);
        remove_test_dbs("test.sled.");
    }

    #[test]
    fn test_rocksdb_store() {
        // Removing preexistent DBs in case of a failed previous test
        remove_test_dbs("test.rocksdb.");
        let store = Store::new("test", EngineType::RocksDB).unwrap();
        test_store_tx(store.clone());
        test_store_height(store.clone());

        // FIXME patching rocksdb weird behavior
        std::mem::forget(store);
        remove_test_dbs("test.rocksdb.");
    }

    fn test_store_height(mut store: Store) {
        // Test height starts in 0
        assert_eq!(Some(0u64), store.get_height());

        // Set height to an arbitrary number
        store.set_height(25u64).unwrap();

        // Test value has been persisted
        assert_eq!(Some(25u64), store.get_height());
    }

    fn test_store_tx(mut store: Store) {
        let tx_hash = Felt252::new(123123);
        let tx_fee = Felt252::new(89853483);
        let tx_signature = vec![Felt252::new(183728913)];
        let tx_nonce = Felt252::new(5);
        let tx_sender_address = Felt252::new(91232018);
        let tx_calldata = vec![Felt252::new(10), Felt252::new(0)];

        let tx = new_transaction(
            tx_hash.clone(),
            tx_fee.clone(),
            tx_signature.clone(),
            tx_nonce.clone(),
            tx_sender_address.clone(),
            tx_calldata.clone(),
        );
        let _ = store.add_transaction(tx);

        let stored_tx = store.get_transaction(tx_hash.clone()).unwrap().unwrap();
        let (
            stored_tx_hash,
            stored_tx_fee,
            stored_tx_signature,
            stored_tx_nonce,
            stored_tx_sender_address,
            stored_tx_calldata,
        ) = get_tx_data(stored_tx);
        assert_eq!(tx_hash, stored_tx_hash);
        assert_eq!(tx_fee, stored_tx_fee);
        assert_eq!(tx_signature, stored_tx_signature);
        assert_eq!(tx_nonce, stored_tx_nonce);
        assert_eq!(tx_sender_address, stored_tx_sender_address);
        assert_eq!(tx_calldata, stored_tx_calldata);
    }

    fn new_transaction(
        tx_hash: Felt252,
        tx_fee: Felt252,
        tx_signature: Vec<Felt252>,
        tx_nonce: Felt252,
        tx_sender_address: Felt252,
        tx_calldata: Vec<Felt252>,
    ) -> Transaction {
        let invoke_tx_v1 = InvokeTransactionV1 {
            transaction_hash: to_field_element(tx_hash),
            max_fee: to_field_element(tx_fee),
            signature: tx_signature.iter().map(|elem| to_field_element(elem.clone())).collect::<Vec<_>>(),
            nonce: FieldElement::from_bytes_be(&tx_nonce.to_be_bytes()).unwrap(),
            sender_address: FieldElement::from_bytes_be(&tx_sender_address.to_be_bytes()).unwrap(),
            calldata: tx_calldata.iter().map(|elem| to_field_element(elem.clone())).collect::<Vec<_>>(),
        };
        Transaction::Invoke(InvokeTransaction::V1(invoke_tx_v1))
    }

    fn get_tx_data(
        tx: Transaction,
    ) -> (
        Felt252,
        Felt252,
        Vec<Felt252>,
        Felt252,
        Felt252,
        Vec<Felt252>,
    ) {
        match tx {
            Transaction::Invoke(InvokeTransaction::V1(invoke_tx_v1)) => (
                to_felt252(invoke_tx_v1.transaction_hash),
                to_felt252(invoke_tx_v1.max_fee),
                invoke_tx_v1.signature.iter().map(|elem| to_felt252(*elem)).collect::<Vec<_>>(),
                to_felt252(invoke_tx_v1.nonce),
                to_felt252(invoke_tx_v1.sender_address),
                invoke_tx_v1.calldata.iter().map(|elem| to_felt252(*elem)).collect::<Vec<_>>(),
            ),
            _ => todo!(),
        }
    }

    fn remove_test_dbs(prefix: &str) {
        // Removes all test databases from filesystem
        for entry in fs::read_dir(env::current_dir().unwrap()).unwrap() {
            if entry
                .as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .starts_with(prefix)
            {
                fs::remove_dir_all(entry.unwrap().path()).unwrap();
            }
        }
    }
}
