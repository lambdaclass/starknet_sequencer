use crate::rpc::{
    serializable_types::FeltParam, BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
    ContractClass, DeclareTransactionResult, DeployAccountTransactionResult, EventFilterWithPage,
    EventsPage, FeeEstimate, FunctionCall, InvokeTransactionResult, MaybePendingBlockWithTxHashes,
    MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, StateUpdate,
    SyncStatusType, Transaction,
};
use crate::store::{Store, EngineType};
use cairo_felt::Felt252;
use jsonrpsee::
    core::{async_trait, RpcResult};
use starknet_core::types::{TransactionStatus, SimulatedTransaction, SimulationFlag};

use super::StarknetRpcApiServer;

pub struct StarknetBackend {
    // mempool_handler: Mempool,
    store: Store,
}

impl StarknetBackend {
    pub fn new(store: &str) -> StarknetBackend {

        let store_path = format!("db_{}", store);
        StarknetBackend {
            store: Store::new(&store_path, EngineType::Sled).expect("Failed to create sequencer store"),
        }
    }
}

#[async_trait]
#[allow(unused_variables)]
impl StarknetRpcApiServer for StarknetBackend {
    fn spec_version(&self) -> RpcResult<String> {
        Ok("0.6.0".to_string())
    }

    fn get_transaction_status(&self, transaction_hash: FeltParam) -> RpcResult<TransactionStatus> {
        unimplemented!()
    }


    /// Returns the execution trace of a transaction by simulating it in the runtime.
    async fn simulate_transactions(
        &self,
        block_id: BlockId,
        transactions: Vec<BroadcastedTransaction>,
        simulation_flags: Vec<SimulationFlag>,
    ) -> RpcResult<Vec<SimulatedTransaction>> {
        unimplemented!()
    }

    async fn get_transaction_receipt(
        &self,
        transaction_hash: FeltParam,
    ) -> RpcResult<MaybePendingTransactionReceipt> {
        unimplemented!()
    }

    /// Returns the information about a transaction by transaction hash.
    fn get_transaction_by_hash(&self, transaction_hash: FeltParam) -> RpcResult<Transaction> {
        unimplemented!()
    }

    fn block_number(&self) -> RpcResult<u64> {
        Ok(self.store.get_height().expect("Heigh not found"))
    }

    fn block_hash_and_number(&self) -> RpcResult<BlockHashAndNumber> {
        unimplemented!();
    }

    fn get_block_transaction_count(&self, block_id: BlockId) -> RpcResult<u128> {
        unimplemented!();
    }

    /// get the storage at a given address and key and at a given block
    fn get_storage_at(
        &self,
        contract_address: FeltParam,
        key: FeltParam,
        block_id: BlockId,
    ) -> RpcResult<Felt252> {
        unimplemented!();
    }

    fn call(&self, request: FunctionCall, block_id: BlockId) -> RpcResult<Vec<String>> {
        unimplemented!();
    }

    /// Get the contract class at a given contract address for a given block id
    fn get_class_at(
        &self,
        block_id: BlockId,
        contract_address: FeltParam,
    ) -> RpcResult<ContractClass> {
        unimplemented!();
    }

    /// Get the contract class hash in the given block for the contract deployed at the given
    /// address
    ///
    /// # Arguments
    ///
    /// * `block_id` - The hash of the requested block, or number (height) of the requested block,
    ///   or a block tag
    /// * `contract_address` - The address of the contract whose class hash will be returned
    ///
    /// # Returns
    ///
    /// * `class_hash` - The class hash of the given contract
    fn get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: FeltParam,
    ) -> RpcResult<Felt252> {
        unimplemented!();
    }

    /// Implementation of the `syncing` RPC Endpoint.
    async fn syncing(&self) -> RpcResult<SyncStatusType> {
        unimplemented!();
    }

    /// Get the contract class definition in the given block associated with the given hash.
    fn get_class(&self, block_id: BlockId, class_hash: FeltParam) -> RpcResult<ContractClass> {
        unimplemented!();
    }

    /// Returns the specified block with transaction hashes.
    fn get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> RpcResult<MaybePendingBlockWithTxHashes> {
        unimplemented!();
    }

    /// Get the nonce associated with the given address at the given block
    fn get_nonce(&self, block_id: BlockId, contract_address: FeltParam) -> RpcResult<Felt252> {
        unimplemented!();
    }

    /// Get block information with full transactions given the block id
    fn get_block_with_txs(&self, block_id: BlockId) -> RpcResult<MaybePendingBlockWithTxs> {
        unimplemented!();
    }

    /// Returns the chain id.
    fn chain_id(&self) -> RpcResult<Felt252> {
        unimplemented!();
    }

    /// Estimate the fee associated with transaction
    ///
    /// # Arguments
    ///
    /// * `request` - starknet transaction request
    /// * `block_id` - hash of the requested block, number (height), or tag
    ///
    /// # Returns
    ///
    /// * `fee_estimate` - fee estimate in gwei
    async fn estimate_fee(
        &self,
        request: Vec<BroadcastedTransaction>,
        block_id: BlockId,
    ) -> RpcResult<Vec<FeeEstimate>> {
        unimplemented!();
    }

    // Returns the details of a transaction by a given block id and index
    fn get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: u64,
    ) -> RpcResult<Transaction> {
        unimplemented!();
    }

    /// Get the information about the result of executing the requested block
    fn get_state_update(&self, block_id: BlockId) -> RpcResult<StateUpdate> {
        unimplemented!();
    }

    /// Returns all events matching the given filter
    async fn get_events(&self, filter: EventFilterWithPage) -> RpcResult<EventsPage> {
        unimplemented!();
    }
/// Add an Invoke Transaction to invoke a contract function
    ///
    /// # Arguments
    ///
    /// * `invoke tx` - <https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/#invoke_transaction>
    ///
    /// # Returns
    ///
    /// * `transaction_hash` - transaction hash corresponding to the invocation
    async fn add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> RpcResult<InvokeTransactionResult> {
        unimplemented!();
    }

    /// Add an Deploy Account Transaction
    ///
    /// # Arguments
    ///
    /// * `deploy account transaction` - <https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/#deploy_account_transaction>
    ///
    /// # Returns
    ///
    /// * `transaction_hash` - transaction hash corresponding to the invocation
    /// * `contract_address` - address of the deployed contract account
    async fn add_deploy_account_transaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> RpcResult<DeployAccountTransactionResult> {
        unimplemented!();
    }

    async fn add_declare_transaction(
        &self,
        declare_transaction: BroadcastedDeclareTransaction,
    ) -> RpcResult<DeclareTransactionResult> {
        unimplemented!();
    }
}
