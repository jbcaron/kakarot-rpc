use crate::providers::eth_provider::database::types::receipt::ExtendedTxReceipt;
use alloy_primitives::{Address, Bytes, B256, B64, U256, U64};
use alloy_rpc_types::{
    serde_helpers::JsonStorageKey, state::StateOverride, AccessListResult, Block, BlockOverrides,
    EIP1186AccountProofResponse, FeeHistory, Filter, FilterChanges, Index, SyncStatus, Transaction as EthTransaction,
    TransactionRequest, Work,
};
use alloy_serde::WithOtherFields;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use reth_primitives::{BlockId, BlockNumberOrTag};

/// Ethereum JSON-RPC API Trait
/// Mostly based on <https://github.com/paradigmxyz/reth/blob/559124ac5a0b25030250203babcd8a94693df648/crates/rpc/rpc-api/src/eth.rs#L15>
/// With some small modifications
#[rpc(server, namespace = "eth")]
#[async_trait]
pub trait EthApi {
    #[method(name = "blockNumber")]
    async fn block_number(&self) -> RpcResult<U64>;

    /// Returns an object with data about the sync status or false.
    #[method(name = "syncing")]
    async fn syncing(&self) -> RpcResult<SyncStatus>;

    /// Returns the client coinbase address.
    #[method(name = "coinbase")]
    async fn coinbase(&self) -> RpcResult<Address>;

    /// Returns a list of addresses owned by client.
    #[method(name = "accounts")]
    async fn accounts(&self) -> RpcResult<Vec<Address>>;

    /// Returns the chain ID of the current network.
    #[method(name = "chainId")]
    async fn chain_id(&self) -> RpcResult<Option<U64>>;

    /// Returns information about a block by hash.
    #[method(name = "getBlockByHash")]
    async fn block_by_hash(
        &self,
        hash: B256,
        full: bool,
    ) -> RpcResult<Option<WithOtherFields<Block<WithOtherFields<EthTransaction>>>>>;

    /// Returns information about a block by number.
    #[method(name = "getBlockByNumber")]
    async fn block_by_number(
        &self,
        number: BlockNumberOrTag,
        full: bool,
    ) -> RpcResult<Option<WithOtherFields<Block<WithOtherFields<EthTransaction>>>>>;

    /// Returns the number of transactions in a block from a block matching the given block hash.
    #[method(name = "getBlockTransactionCountByHash")]
    async fn block_transaction_count_by_hash(&self, hash: B256) -> RpcResult<Option<U256>>;

    /// Returns the number of transactions in a block matching the given block number.
    #[method(name = "getBlockTransactionCountByNumber")]
    async fn block_transaction_count_by_number(&self, number: BlockNumberOrTag) -> RpcResult<Option<U256>>;

    /// Returns the number of uncles in a block from a block matching the given block hash.
    #[method(name = "getUncleCountByBlockHash")]
    async fn block_uncles_count_by_block_hash(&self, hash: B256) -> RpcResult<U256>;

    /// Returns the number of uncles in a block with given block number.
    #[method(name = "getUncleCountByBlockNumber")]
    async fn block_uncles_count_by_block_number(&self, number: BlockNumberOrTag) -> RpcResult<U256>;

    /// Returns an uncle block of the given block and index.
    #[method(name = "getUncleByBlockHashAndIndex")]
    async fn uncle_by_block_hash_and_index(
        &self,
        hash: B256,
        index: Index,
    ) -> RpcResult<Option<WithOtherFields<Block<WithOtherFields<EthTransaction>>>>>;

    /// Returns an uncle block of the given block and index.
    #[method(name = "getUncleByBlockNumberAndIndex")]
    async fn uncle_by_block_number_and_index(
        &self,
        number: BlockNumberOrTag,
        index: Index,
    ) -> RpcResult<Option<WithOtherFields<Block<WithOtherFields<EthTransaction>>>>>;

    /// Returns the information about a transaction requested by transaction hash.
    #[method(name = "getTransactionByHash")]
    async fn transaction_by_hash(&self, hash: B256) -> RpcResult<Option<WithOtherFields<EthTransaction>>>;

    /// Returns information about a transaction by block hash and transaction index position.
    #[method(name = "getTransactionByBlockHashAndIndex")]
    async fn transaction_by_block_hash_and_index(
        &self,
        hash: B256,
        index: Index,
    ) -> RpcResult<Option<WithOtherFields<EthTransaction>>>;

    /// Returns information about a transaction by block number and transaction index position.
    #[method(name = "getTransactionByBlockNumberAndIndex")]
    async fn transaction_by_block_number_and_index(
        &self,
        number: BlockNumberOrTag,
        index: Index,
    ) -> RpcResult<Option<WithOtherFields<EthTransaction>>>;

    /// Returns the receipt of a transaction by transaction hash.
    #[method(name = "getTransactionReceipt")]
    async fn transaction_receipt(&self, hash: B256) -> RpcResult<Option<ExtendedTxReceipt>>;

    /// Returns the balance of the account of given address.
    #[method(name = "getBalance")]
    async fn balance(&self, address: Address, block_number: Option<BlockId>) -> RpcResult<U256>;

    /// Returns the value from a storage position at a given address
    #[method(name = "getStorageAt")]
    async fn storage_at(&self, address: Address, index: JsonStorageKey, block_id: Option<BlockId>) -> RpcResult<B256>;

    /// Returns the number of transactions sent from an address at given block number.
    #[method(name = "getTransactionCount")]
    async fn transaction_count(&self, address: Address, block_id: Option<BlockId>) -> RpcResult<U256>;

    /// Returns code at a given address at given block number.
    #[method(name = "getCode")]
    async fn get_code(&self, address: Address, block_id: Option<BlockId>) -> RpcResult<Bytes>;

    /// Returns the logs corresponding to the given filter object.
    #[method(name = "getLogs")]
    async fn get_logs(&self, filter: Filter) -> RpcResult<FilterChanges>;

    /// Executes a new message call immediately without creating a transaction on the block chain.
    #[method(name = "call")]
    async fn call(
        &self,
        request: TransactionRequest,
        block_id: Option<BlockId>,
        state_overrides: Option<StateOverride>,
        block_overrides: Option<Box<BlockOverrides>>,
    ) -> RpcResult<Bytes>;

    /// Generates an access list for a transaction.
    ///
    /// This method creates an [EIP2930](https://eips.ethereum.org/EIPS/eip-2930) type accessList based on a given Transaction.
    ///
    /// An access list contains all storage slots and addresses touched by the transaction, except
    /// for the sender account and the chain's precompiles.
    ///
    /// It returns list of addresses and storage keys used by the transaction, plus the gas
    /// consumed when the access list is added. That is, it gives you the list of addresses and
    /// storage keys that will be used by that transaction, plus the gas consumed if the access
    /// list is included. Like estimateGas, this is an estimation; the list could change
    /// when the transaction is actually mined. Adding an accessList to your transaction does
    /// not necessary result in lower gas usage compared to a transaction without an access
    /// list.
    #[method(name = "createAccessList")]
    async fn create_access_list(
        &self,
        request: TransactionRequest,
        block_id: Option<BlockId>,
    ) -> RpcResult<AccessListResult>;

    /// Generates and returns an estimate of how much gas is necessary to allow the transaction to
    /// complete.
    #[method(name = "estimateGas")]
    async fn estimate_gas(&self, request: TransactionRequest, block_id: Option<BlockId>) -> RpcResult<U256>;

    /// Returns the current price per gas in wei.
    #[method(name = "gasPrice")]
    async fn gas_price(&self) -> RpcResult<U256>;

    /// Returns the Transaction fee history
    ///
    /// Introduced in EIP-1159 for getting information on the appropriate priority fee to use.
    ///
    /// Returns transaction base fee per gas and effective priority fee per gas for the
    /// requested/supported block range. The returned Fee history for the returned block range
    /// can be a subsection of the requested range if not all blocks are available.
    #[method(name = "feeHistory")]
    async fn fee_history(
        &self,
        block_count: U64,
        newest_block: BlockNumberOrTag,
        reward_percentiles: Option<Vec<f64>>,
    ) -> RpcResult<FeeHistory>;

    /// Returns the current maxPriorityFeePerGas per gas in wei.
    #[method(name = "maxPriorityFeePerGas")]
    async fn max_priority_fee_per_gas(&self) -> RpcResult<U256>;

    /// Introduced in EIP-4844, returns the current blob base fee in wei.
    #[method(name = "blobBaseFee")]
    async fn blob_base_fee(&self) -> RpcResult<U256>;

    /// Returns whether the client is actively mining new blocks.
    #[method(name = "mining")]
    async fn mining(&self) -> RpcResult<bool>;

    /// Returns the number of hashes per second that the node is mining with.
    #[method(name = "hashrate")]
    async fn hashrate(&self) -> RpcResult<U256>;

    /// Returns the hash of the current block, the seedHash, and the boundary condition to be met
    /// (“target”)
    #[method(name = "getWork")]
    async fn get_work(&self) -> RpcResult<Work>;

    /// Used for submitting mining hashrate.
    #[method(name = "submitHashrate")]
    async fn submit_hashrate(&self, hashrate: U256, id: B256) -> RpcResult<bool>;

    /// Used for submitting a proof-of-work solution.
    #[method(name = "submitWork")]
    async fn submit_work(&self, nonce: B64, pow_hash: B256, mix_digest: B256) -> RpcResult<bool>;

    /// Sends transaction; will block waiting for signer to return the
    /// transaction hash.
    #[method(name = "sendTransaction")]
    async fn send_transaction(&self, request: TransactionRequest) -> RpcResult<B256>;

    /// Sends signed transaction, returning its hash.
    #[method(name = "sendRawTransaction")]
    async fn send_raw_transaction(&self, bytes: Bytes) -> RpcResult<B256>;

    /// Returns an Ethereum specific signature with: sign(keccak256("\x19Ethereum Signed Message:\n"
    /// + len(message) + message))).
    #[method(name = "sign")]
    async fn sign(&self, address: Address, message: Bytes) -> RpcResult<Bytes>;

    /// Signs a transaction that can be submitted to the network at a later time using with
    /// `sendRawTransaction.`
    #[method(name = "signTransaction")]
    async fn sign_transaction(&self, transaction: TransactionRequest) -> RpcResult<Bytes>;

    /// Signs data via [EIP-712](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-712.md).
    #[method(name = "signTypedData")]
    async fn sign_typed_data(&self, address: Address, data: serde_json::Value) -> RpcResult<Bytes>;

    /// Returns the account and storage values of the specified account including the Merkle-proof.
    /// This call can be used to verify that the data you are pulling from is not tampered with.
    #[method(name = "getProof")]
    async fn get_proof(
        &self,
        address: Address,
        keys: Vec<B256>,
        block_id: Option<BlockId>,
    ) -> RpcResult<EIP1186AccountProofResponse>;

    /// Creates a filter object, based on filter options, to notify when the state changes (logs).
    #[method(name = "newFilter")]
    async fn new_filter(&self, filter: Filter) -> RpcResult<U64>;

    /// Creates a filter in the node, to notify when a new block arrives.
    #[method(name = "newBlockFilter")]
    async fn new_block_filter(&self) -> RpcResult<U64>;

    /// Creates a filter in the node, to notify when new pending transactions arrive.
    #[method(name = "newPendingTransactionFilter")]
    async fn new_pending_transaction_filter(&self) -> RpcResult<U64>;

    /// Destroys a filter based on filter ID
    #[method(name = "uninstallFilter")]
    async fn uninstall_filter(&self, id: U64) -> RpcResult<bool>;

    /// Returns a list of all logs based on filter ID since the last log retrieval
    #[method(name = "getFilterChanges")]
    async fn get_filter_changes(&self, id: U64) -> RpcResult<FilterChanges>;

    /// Returns a list of all logs based on filter ID
    #[method(name = "getFilterLogs")]
    async fn get_filter_logs(&self, id: U64) -> RpcResult<FilterChanges>;

    /// Returns all transaction receipts for a given block.
    #[method(name = "getBlockReceipts")]
    async fn block_receipts(&self, block_id: Option<BlockId>) -> RpcResult<Option<Vec<ExtendedTxReceipt>>>;
}
