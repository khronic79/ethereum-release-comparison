use schemars::JsonSchema;
use schemars::gen::SchemaGenerator;
use schemars::schema::Schema;
#[allow(unused_imports)]
use std::collections::{BTreeMap, HashMap, HashSet};
#[allow(unused_imports)]
use crate::prims::*;

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCancelBundle {
    pub replacement_uuid: String,
}

#[derive(JsonSchema)]
pub struct Receipts {
    pub receipt_vec: Vec<Vec<crate::prims::Any>>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct GasPriceOracleConfig {
    pub blocks: u32,
    pub percentile: u32,
    pub max_header_history: u64,
    pub max_block_history: u64,
    pub max_reward_percentile_count: u64,
    pub default_suggested_fee: Option<crate::prims::Quantity>,
    pub max_price: Option<crate::prims::Quantity>,
    pub ignore_price: Option<crate::prims::Quantity>,
}

#[derive(JsonSchema)]
pub struct BlockBody {
    pub transactions: Vec<crate::prims::Any>,
    pub ommers: Vec<Header>,
    pub withdrawals: Option<Withdrawals>,
}

pub type Bytes48 = crate::prims::Any;

#[derive(JsonSchema)]
pub enum TransactionIndex {
    All,
    Index(usize),
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCallBundleResponse {
    pub bundle_hash: crate::prims::Hex32,
    pub bundle_gas_price: crate::prims::Quantity,
    pub coinbase_diff: crate::prims::Quantity,
    pub eth_sent_to_coinbase: crate::prims::Quantity,
    pub gas_fees: crate::prims::Quantity,
    pub results: Vec<EthCallBundleTransactionResult>,
    pub state_block_number: u64,
    pub total_gas_used: u64,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct StateContext {
    pub block_number: Option<BlockId>,
    pub transaction_index: Option<TransactionIndex>,
}

pub type Blob = crate::prims::Any;

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct Receipt {
    #[schemars(flatten)]
    pub status: Eip658Value,
    #[schemars(with = "crate::prims::Quantity")]
    pub cumulative_gas_used: crate::prims::Quantity,
    pub logs: Vec<Log>,
}

#[derive(JsonSchema)]
pub struct Withdrawals {
}

pub type RpcError = crate::prims::Any;

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EIP1186StorageProof {
    pub key: JsonStorageKey,
    pub value: crate::prims::Quantity,
    pub proof: Vec<crate::prims::Hex>,
}

#[derive(JsonSchema)]
#[schemars(untagged)]
pub enum BlobTransactionSidecarVariant {
    Eip4844(BlobTransactionSidecar),
    Eip7594(BlobTransactionSidecarEip7594),
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCapabilitiesHead {
    #[schemars(with = "crate::prims::Quantity")]
    pub number: crate::prims::Quantity,
    pub hash: crate::prims::Hex32,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct SimBlock {
    pub block_overrides: Option<BlockOverrides>,
    pub state_overrides: Option<StateOverride>,
    pub calls: Vec<TransactionRequest>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct AccountOverride {
    pub balance: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub nonce: Option<crate::prims::Quantity>,
    pub code: Option<crate::prims::Hex>,
    pub state: Option<crate::prims::Any>,
    pub state_diff: Option<crate::prims::Any>,
    #[schemars(rename = "movePrecompileToAddress")]
    pub move_precompile_to: Option<crate::prims::Hex20>,
}

#[derive(JsonSchema)]
pub enum FilterBlockOption {
    Range { from_block: Option<BlockNumberOrTag>, to_block: Option<BlockNumberOrTag> },
    AtBlockHash(crate::prims::Hex32),
}

#[derive(JsonSchema)]
pub struct ForwardConfig {
    pub tx_forwarder: Option<crate::prims::Any>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct Stage {
    pub name: String,
    #[schemars(with = "crate::prims::Quantity")]
    pub block: crate::prims::Quantity,
}

#[derive(JsonSchema)]
pub struct TxpoolContentFrom {
    pub pending: BTreeMap<String, Transaction>,
    pub queued: BTreeMap<String, Transaction>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EIP1186AccountProofResponse {
    pub address: crate::prims::Hex20,
    pub balance: crate::prims::Quantity,
    pub code_hash: crate::prims::Hex32,
    #[schemars(with = "crate::prims::Quantity")]
    pub nonce: crate::prims::Quantity,
    pub storage_hash: crate::prims::Hex32,
    pub account_proof: Vec<crate::prims::Hex>,
    pub storage_proof: Vec<EIP1186StorageProof>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCallResponse {
    pub value: Option<crate::prims::Hex>,
    pub error: Option<String>,
}

pub type Topic = FilterSet;

#[derive(JsonSchema)]
#[schemars(untagged)]
pub enum JsonStorageKey {
    Hash(crate::prims::Hex32),
    Number(crate::prims::Quantity),
}

#[derive(JsonSchema)]
pub struct Work {
    pub pow_hash: crate::prims::Hex32,
    pub seed_hash: crate::prims::Hex32,
    pub target: crate::prims::Hex32,
    pub number: Option<u64>,
}

#[derive(JsonSchema)]
pub struct Requests {
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct Validity {
    pub refund: Option<Vec<Refund>>,
    pub refund_config: Option<Vec<RefundConfig>>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct Refund {
    #[schemars(with = "crate::prims::Quantity")]
    pub body_idx: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub percent: crate::prims::Quantity,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct Bundle {
    pub transactions: Vec<TransactionRequest>,
    pub block_override: Option<BlockOverrides>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct RpcBlockHash {
    pub block_hash: crate::prims::Hex32,
    pub require_canonical: Option<bool>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthSendPrivateTransaction {
    pub tx: crate::prims::Hex,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_block_number: Option<crate::prims::Quantity>,
    pub preferences: PrivateTransactionPreferences,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCallBundle {
    pub txs: Vec<crate::prims::Hex>,
    #[schemars(with = "crate::prims::Quantity")]
    pub block_number: crate::prims::Quantity,
    pub state_block_number: BlockNumberOrTag,
    pub transaction_index: Option<TransactionIndex>,
    pub coinbase: Option<crate::prims::Hex20>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub timestamp: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub timeout: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub gas_limit: Option<crate::prims::Quantity>,
    pub difficulty: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub base_fee: Option<crate::prims::Quantity>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct FeeHistoryCacheConfig {
    pub max_blocks: u64,
    pub resolution: u64,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct SimulatePayload {
    pub block_state_calls: Vec<SimBlock>,
    pub trace_transfers: bool,
    pub validation: bool,
    pub return_full_transactions: bool,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthStateCacheConfig {
    pub max_blocks: u32,
    pub max_receipts: u32,
    pub max_headers: u32,
    pub max_bals: u32,
    pub max_concurrent_db_requests: usize,
    pub max_cached_tx_hashes: u32,
}

#[derive(JsonSchema)]
pub struct Log {
    pub address: crate::prims::Hex20,
    pub topics: Vec<crate::prims::Hex32>,
    pub data: crate::prims::Hex,
}

#[derive(JsonSchema)]
pub struct Block {
    pub header: Header,
    pub body: BlockBody,
}

#[derive(JsonSchema)]
pub struct OtherFields {
    pub inner: BTreeMap<String, crate::prims::Any>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct SimulatedBlock {
    #[schemars(flatten)]
    pub inner: Block,
    pub calls: Vec<SimCallResult>,
}

#[derive(JsonSchema)]
pub struct TxpoolInspect {
    pub pending: BTreeMap<crate::prims::Hex20, BTreeMap<String, TxpoolInspectSummary>>,
    pub queued: BTreeMap<crate::prims::Hex20, BTreeMap<String, TxpoolInspectSummary>>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct BlobTransactionSidecarEip7594 {
    pub blobs: Vec<crate::prims::Hex>,
    pub commitments: Vec<crate::prims::Hex>,
    pub cell_proofs: Vec<crate::prims::Hex>,
}

#[derive(JsonSchema)]
pub enum Eip658Value {
    Eip658(bool),
    PostState(crate::prims::Hex32),
}

pub type Account = crate::prims::Any;

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCancelPrivateTransaction {
    pub tx_hash: crate::prims::Hex32,
}

#[derive(JsonSchema)]
#[schemars(untagged)]
pub enum AccountStorage {
    RootHash(crate::prims::Hex32),
    Slots(HashMap<crate::prims::Quantity, crate::prims::Hex32>),
}

#[derive(JsonSchema)]
pub struct EthConfig {
    pub cache: EthStateCacheConfig,
    pub gas_oracle: GasPriceOracleConfig,
    pub eth_proof_window: u64,
    pub max_tracing_requests: usize,
    pub max_blocking_io_requests: usize,
    pub max_trace_filter_blocks: u64,
    pub max_blocks_per_filter: u64,
    pub max_logs_per_response: usize,
    pub rpc_gas_cap: u64,
    pub rpc_max_simulate_blocks: u64,
    pub compute_state_root_for_eth_simulate: bool,
    pub stale_filter_ttl: crate::prims::Any,
    pub fee_history_cache: FeeHistoryCacheConfig,
    pub proof_permits: usize,
    pub max_batch_size: usize,
    pub pending_block_kind: PendingBlockKind,
    pub raw_tx_forwarder: ForwardConfig,
    pub send_raw_transaction_sync_timeout: crate::prims::Any,
    pub rpc_evm_memory_limit: u64,
    pub force_blob_sidecar_upcasting: bool,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct FeeHistory {
    #[schemars(with = "Vec<crate::prims::Quantity>")]
    pub base_fee_per_gas: Vec<crate::prims::Quantity>,
    pub gas_used_ratio: Vec<f64>,
    #[schemars(with = "Vec<crate::prims::Quantity>")]
    pub base_fee_per_blob_gas: Vec<crate::prims::Quantity>,
    pub blob_gas_used_ratio: Vec<f64>,
    #[schemars(with = "crate::prims::Quantity")]
    pub oldest_block: crate::prims::Quantity,
    #[schemars(with = "Option<Vec<Vec<crate::prims::Quantity>>>")]
    pub reward: Option<Vec<Vec<crate::prims::Quantity>>>,
}

#[derive(JsonSchema)]
pub enum PendingTransactionFilterKind {
    Hashes,
    Full,
}

#[derive(JsonSchema)]
pub struct PrivacyHint {
    pub calldata: bool,
    pub contract_address: bool,
    pub logs: bool,
    pub function_selector: bool,
    pub hash: bool,
    pub tx_hash: bool,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct AccountInfo {
    pub balance: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub nonce: crate::prims::Quantity,
    pub code: crate::prims::Hex,
}

#[derive(JsonSchema)]
#[schemars(untagged)]
pub enum SubscriptionResult {
    Header(Box<WithOtherFields>),
    Log(Box<Log>),
    TransactionHash(crate::prims::Hex32),
    FullTransaction(Box<Transaction>),
    SyncState(PubSubSyncStatus),
    TransactionReceipts(Vec<TransactionReceipt>),
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct TransactionConditional {
    pub known_accounts: crate::prims::Any,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub block_number_min: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub block_number_max: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub timestamp_min: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub timestamp_max: Option<crate::prims::Quantity>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub from: Option<crate::prims::Hex20>,
    pub to: Option<crate::prims::Any>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub gas_price: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_fee_per_gas: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_priority_fee_per_gas: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_fee_per_blob_gas: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub gas: Option<crate::prims::Quantity>,
    pub value: Option<crate::prims::Quantity>,
    #[schemars(flatten)]
    pub input: TransactionInput,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub nonce: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub chain_id: Option<crate::prims::Quantity>,
    pub access_list: Option<crate::prims::Any>,
    #[schemars(rename = "type")]
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub transaction_type: Option<crate::prims::Quantity>,
    pub blob_versioned_hashes: Option<Vec<crate::prims::Hex32>>,
    #[schemars(flatten)]
    pub sidecar: Option<BlobTransactionSidecarVariant>,
    pub authorization_list: Option<Vec<crate::prims::Any>>,
}

#[derive(JsonSchema)]
pub enum BlockNumberOrTag {
    Latest,
    Finalized,
    Safe,
    Earliest,
    Pending,
    Number(u64),
}

#[derive(JsonSchema)]
pub struct TxpoolStatus {
    #[schemars(with = "crate::prims::Quantity")]
    pub pending: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub queued: crate::prims::Quantity,
}

#[derive(JsonSchema)]
pub enum PendingBlockKind {
    Empty,
    None,
    Full,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct Header {
    pub parent_hash: crate::prims::Hex32,
    #[schemars(rename = "sha3Uncles")]
    pub ommers_hash: crate::prims::Hex32,
    #[schemars(rename = "miner")]
    pub beneficiary: crate::prims::Hex20,
    pub state_root: crate::prims::Hex32,
    pub transactions_root: crate::prims::Hex32,
    pub receipts_root: crate::prims::Hex32,
    pub logs_bloom: crate::prims::Hex,
    pub difficulty: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub number: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub gas_limit: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub gas_used: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub timestamp: crate::prims::Quantity,
    pub extra_data: crate::prims::Hex,
    pub mix_hash: crate::prims::Hex32,
    pub nonce: crate::prims::Hex,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub base_fee_per_gas: Option<crate::prims::Quantity>,
    pub withdrawals_root: Option<crate::prims::Hex32>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub blob_gas_used: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub excess_blob_gas: Option<crate::prims::Quantity>,
    pub parent_beacon_block_root: Option<crate::prims::Hex32>,
    pub requests_hash: Option<crate::prims::Hex32>,
    pub block_access_list_hash: Option<crate::prims::Hex32>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub slot_number: Option<crate::prims::Quantity>,
}

#[derive(JsonSchema)]
pub struct FillTransaction {
    pub raw: crate::prims::Hex,
    pub tx: crate::prims::Any,
}

#[derive(JsonSchema)]
pub struct Index {
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct RefundConfig {
    pub address: crate::prims::Hex20,
    #[schemars(with = "crate::prims::Quantity")]
    pub percent: crate::prims::Quantity,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct SyncStatusMetadata {
    pub syncing: bool,
    #[schemars(with = "crate::prims::Quantity")]
    pub starting_block: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub current_block: crate::prims::Quantity,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub highest_block: Option<crate::prims::Quantity>,
}

pub struct TransactionReceipt;
impl JsonSchema for TransactionReceipt {
    fn schema_name() -> String { "TransactionReceipt".to_string() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        serde_json::from_str(r##"{"type":"object","additionalProperties":true,"properties":{"transactionHash":{"$ref":"#/definitions/Hex32"},"transactionIndex":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"blockHash":{"anyOf":[{"$ref":"#/definitions/Hex32"},{"type":"null"}]},"blockNumber":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"from":{"$ref":"#/definitions/Hex20"},"to":{"anyOf":[{"$ref":"#/definitions/Hex20"},{"type":"null"}]},"cumulativeGasUsed":{"$ref":"#/definitions/Quantity"},"gasUsed":{"$ref":"#/definitions/Quantity"},"contractAddress":{"anyOf":[{"$ref":"#/definitions/Hex20"},{"type":"null"}]},"logs":{"type":"array","items":{"$ref":"#/definitions/Log"}},"logsBloom":{"$ref":"#/definitions/Hex"},"status":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"root":{"anyOf":[{"$ref":"#/definitions/Hex32"},{"type":"null"}]},"type":{"$ref":"#/definitions/Quantity"},"effectiveGasPrice":{"$ref":"#/definitions/Quantity"},"blobGasUsed":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"blobGasPrice":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]}}}"##).expect("override schema")
    }
}

pub type StateOverride = crate::prims::Any;

#[derive(JsonSchema)]
pub struct BlobParams {
    pub target_blob_count: u64,
    pub max_blob_count: u64,
    pub update_fraction: u128,
    pub min_blob_fee: u128,
    pub max_blobs_per_tx: u64,
    pub blob_base_cost: u64,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCapabilitiesDeleteStrategy {
    #[schemars(rename = "type")]
    pub strategy_type: EthCapabilitiesDeleteStrategyKind,
    pub retention_blocks: u64,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCapabilitiesResource {
    pub disabled: bool,
    pub oldest_block: Option<crate::prims::Quantity>,
    pub delete_strategy: Option<EthCapabilitiesDeleteStrategy>,
}

#[derive(JsonSchema)]
pub struct BlobTransactionSidecar {
    pub blobs: Vec<crate::prims::Hex>,
    pub commitments: Vec<crate::prims::Hex>,
    pub proofs: Vec<crate::prims::Hex>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCallBundleTransactionResult {
    pub coinbase_diff: crate::prims::Quantity,
    pub eth_sent_to_coinbase: crate::prims::Quantity,
    pub from_address: crate::prims::Hex20,
    pub gas_fees: crate::prims::Quantity,
    pub gas_price: crate::prims::Quantity,
    pub gas_used: u64,
    pub to_address: Option<crate::prims::Hex20>,
    pub tx_hash: crate::prims::Hex32,
    pub value: Option<crate::prims::Hex>,
    pub revert: Option<crate::prims::Hex>,
}

#[derive(JsonSchema)]
pub struct BloomFilter {
}

#[derive(JsonSchema)]
#[schemars(untagged)]
pub enum PubSubSyncStatus {
    Simple(bool),
    Detailed(SyncStatusMetadata),
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct SimCallResult {
    pub return_data: crate::prims::Hex,
    pub logs: Vec<Log>,
    #[schemars(with = "crate::prims::Quantity")]
    pub gas_used: crate::prims::Quantity,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_used_gas: Option<crate::prims::Quantity>,
    #[schemars(with = "crate::prims::Quantity")]
    pub status: crate::prims::Quantity,
    pub error: Option<SimulateError>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthBundleHash {
    pub bundle_hash: crate::prims::Hex32,
}

#[derive(JsonSchema)]
pub enum SubscriptionKind {
    NewHeads,
    Logs,
    NewPendingTransactions,
    Syncing,
    TransactionReceipts,
}

#[derive(JsonSchema)]
pub enum Params {
    None,
    Logs(Box<Filter>),
    Bool(bool),
    TransactionReceipts(TransactionReceiptsParams),
}

#[derive(JsonSchema)]
pub struct Filter {
    pub block_option: FilterBlockOption,
    pub address: FilterSet,
    pub topics: Vec<Topic>,
}

pub struct Recovered;
impl JsonSchema for Recovered {
    fn schema_name() -> String { "Recovered".to_string() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        serde_json::from_str(r##"{"type":"object","additionalProperties":true}"##).expect("override schema")
    }
}

#[derive(JsonSchema)]
pub struct PrivateTransactionPreferences {
    pub fast: Option<bool>,
    pub validity: Option<Validity>,
    pub privacy: Option<Privacy>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthCapabilities {
    pub head: EthCapabilitiesHead,
    pub state: EthCapabilitiesResource,
    pub tx: EthCapabilitiesResource,
    pub logs: EthCapabilitiesResource,
    pub receipts: EthCapabilitiesResource,
    pub blocks: EthCapabilitiesResource,
    #[schemars(rename = "stateproofs")]
    pub state_proofs: EthCapabilitiesResource,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct EthSendBundle {
    pub txs: Vec<crate::prims::Hex>,
    #[schemars(with = "crate::prims::Quantity")]
    pub block_number: crate::prims::Quantity,
    pub min_timestamp: Option<u64>,
    pub max_timestamp: Option<u64>,
    pub reverting_tx_hashes: Vec<crate::prims::Hex32>,
    pub replacement_uuid: Option<String>,
    pub dropping_tx_hashes: Vec<crate::prims::Hex32>,
    pub refund_percent: Option<u8>,
    pub refund_recipient: Option<crate::prims::Hex20>,
    pub refund_tx_hashes: Vec<crate::prims::Hex32>,
    #[schemars(flatten)]
    pub extra_fields: OtherFields,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct SyncInfo {
    pub starting_block: crate::prims::Quantity,
    pub current_block: crate::prims::Quantity,
    pub highest_block: crate::prims::Quantity,
    pub warp_chunks_amount: Option<crate::prims::Quantity>,
    pub warp_chunks_processed: Option<crate::prims::Quantity>,
    pub stages: Option<Vec<Stage>>,
}

#[derive(JsonSchema)]
pub enum SyncStatus {
    Info(Box<SyncInfo>),
    None,
}

#[derive(JsonSchema)]
pub enum BlockId {
    Hash(RpcBlockHash),
    Number(BlockNumberOrTag),
}

#[derive(JsonSchema)]
pub enum EthCapabilitiesDeleteStrategyKind {
    #[schemars(rename = "window")]
    Window,
}

#[derive(JsonSchema)]
#[schemars(untagged)]
pub enum FilterChanges {
    Empty,
    Logs(Vec<crate::prims::Any>),
    Hashes(Vec<crate::prims::Hex32>),
    Transactions(Vec<Transaction>),
}

#[derive(JsonSchema)]
pub struct TransactionInput {
    pub input: Option<crate::prims::Hex>,
    pub data: Option<crate::prims::Hex>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct TransactionReceiptsParams {
    pub transaction_hashes: Option<Vec<crate::prims::Hex32>>,
}

#[derive(JsonSchema)]
pub struct TxpoolContent {
    pub pending: BTreeMap<crate::prims::Hex20, BTreeMap<String, Transaction>>,
    pub queued: BTreeMap<crate::prims::Hex20, BTreeMap<String, Transaction>>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct SimulateError {
    pub code: i32,
    pub message: String,
    pub data: Option<crate::prims::Hex>,
}

#[derive(JsonSchema)]
pub struct WithOtherFields {
    #[schemars(flatten)]
    pub inner: crate::prims::Any,
    #[schemars(flatten)]
    pub other: OtherFields,
}

#[derive(JsonSchema)]
#[schemars(untagged)]
pub enum FilterId {
    Num(u64),
    Str(String),
}

#[derive(JsonSchema)]
pub struct FilterSet {
    pub set: HashSet<crate::prims::Any>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct BlockOverrides {
    pub number: Option<crate::prims::Quantity>,
    pub difficulty: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub time: Option<crate::prims::Quantity>,
    #[schemars(with = "Option<crate::prims::Quantity>")]
    pub gas_limit: Option<crate::prims::Quantity>,
    pub coinbase: Option<crate::prims::Hex20>,
    pub random: Option<crate::prims::Hex32>,
    pub base_fee: Option<crate::prims::Quantity>,
    pub blob_base_fee: Option<crate::prims::Quantity>,
    pub beacon_root: Option<crate::prims::Hex32>,
    pub block_hash: Option<BTreeMap<u64, crate::prims::Hex32>>,
}

#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
pub struct Privacy {
    pub hints: Option<PrivacyHint>,
    pub builders: Option<Vec<String>>,
}

#[derive(JsonSchema)]
pub struct Signed {
    pub tx: crate::prims::Any,
    pub signature: crate::prims::Any,
    pub hash: crate::prims::Any,
}

pub struct Transaction;
impl JsonSchema for Transaction {
    fn schema_name() -> String { "Transaction".to_string() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        serde_json::from_str(r##"{"type":"object","additionalProperties":true,"properties":{"hash":{"$ref":"#/definitions/Hex32"},"type":{"$ref":"#/definitions/Quantity"},"from":{"$ref":"#/definitions/Hex20"},"to":{"anyOf":[{"$ref":"#/definitions/Hex20"},{"type":"null"}]},"nonce":{"$ref":"#/definitions/Quantity"},"gas":{"$ref":"#/definitions/Quantity"},"gasPrice":{"$ref":"#/definitions/Quantity"},"maxFeePerGas":{"$ref":"#/definitions/Quantity"},"maxPriorityFeePerGas":{"$ref":"#/definitions/Quantity"},"maxFeePerBlobGas":{"$ref":"#/definitions/Quantity"},"value":{"$ref":"#/definitions/Quantity"},"input":{"$ref":"#/definitions/Hex"},"accessList":{"type":"array"},"blobVersionedHashes":{"type":"array"},"authorizationList":{"type":"array"},"v":{"$ref":"#/definitions/Quantity"},"r":{"$ref":"#/definitions/Quantity"},"s":{"$ref":"#/definitions/Quantity"},"yParity":{"$ref":"#/definitions/Quantity"},"chainId":{"$ref":"#/definitions/Quantity"},"blockHash":{"anyOf":[{"$ref":"#/definitions/Hex32"},{"type":"null"}]},"blockNumber":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"transactionIndex":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"blockTimestamp":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]}}}"##).expect("override schema")
    }
}

#[derive(JsonSchema)]
pub struct TxpoolInspectSummary {
    pub to: Option<crate::prims::Hex20>,
    pub value: crate::prims::Quantity,
    pub gas: u64,
    pub gas_price: u128,
}

#[derive(JsonSchema)]
pub struct Withdrawal {
    #[schemars(with = "crate::prims::Quantity")]
    pub index: crate::prims::Quantity,
    #[schemars(rename = "validatorIndex")]
    #[schemars(with = "crate::prims::Quantity")]
    pub validator_index: crate::prims::Quantity,
    pub address: crate::prims::Hex20,
    #[schemars(with = "crate::prims::Quantity")]
    pub amount: crate::prims::Quantity,
}

