use schemars::JsonSchema;
#[allow(unused_imports)]
use crate::prims::*;

pub type Account = crate::prims::Any;


#[derive(JsonSchema)]
pub struct AccountInfo {
    
    pub balance: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub nonce: crate::prims::Quantity,
    
    pub code: crate::prims::Hex,
    #[serde(default, skip_serializing_if = "alloy_trie::AccountExtension::is_empty")]
    pub extension: crate::prims::Any,
}

#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
#[derive(JsonSchema)]
pub struct AccountOverride {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<crate::prims::Quantity>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub nonce: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<crate::prims::Hex>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::prims::Any>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_diff: Option<crate::prims::Any>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "movePrecompileToAddress",
            alias = "MovePrecompileToAddress"
        )]
    pub move_precompile_to: Option<crate::prims::Hex20>,
}

#[serde(untagged)]
#[derive(JsonSchema)]
pub enum AccountStorage {
    
    RootHash(crate::prims::Hex32),
    
    Slots(HashMap<crate::prims::Quantity, crate::prims::Hex32>),
}


#[derive(JsonSchema)]
pub enum Authorization {
    
    Basic(String),
    
    Bearer(String),
    
    Raw(String),
}

pub type Blob = crate::prims::Any;


#[derive(JsonSchema)]
pub struct BlobParams {
    
    pub target_blob_count: crate::prims::Quantity,
    
    pub max_blob_count: crate::prims::Quantity,
    
    pub update_fraction: crate::prims::Quantity,
    
    pub min_blob_fee: crate::prims::Quantity,
    
    pub max_blobs_per_tx: crate::prims::Quantity,
    
    pub blob_base_cost: crate::prims::Quantity,
}


#[derive(JsonSchema)]
pub struct BlobTransactionSidecar {
    
    pub blobs: Vec<crate::prims::Hex>,
    
    pub commitments: Vec<crate::prims::Hex>,
    
    pub proofs: Vec<crate::prims::Hex>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct BlobTransactionSidecarEip7594 {
    
    pub blobs: Vec<crate::prims::Hex>,
    
    pub commitments: Vec<crate::prims::Hex>,
    
    pub cell_proofs: Vec<crate::prims::Hex>,
}


#[derive(JsonSchema)]
pub enum BlobTransactionSidecarVariant {
    
    Eip4844(BlobTransactionSidecar),
    
    Eip7594(BlobTransactionSidecarEip7594),
}


#[derive(JsonSchema)]
pub struct Block<T, H>
where T: JsonSchema, H: JsonSchema {
    
    pub header: Header,
    
    pub body: BlockBody,
}


#[derive(JsonSchema)]
pub struct BlockBody<T, H>
where T: JsonSchema, H: JsonSchema {
    
    pub transactions: Vec<crate::prims::Any>,
    
    pub ommers: Vec<Header>,
    
    pub withdrawals: Option<Withdrawals>,
}


#[derive(JsonSchema)]
pub enum BlockId {
    
    Hash(RpcBlockHash),
    
    Number(BlockNumberOrTag),
}


#[derive(JsonSchema)]
pub enum BlockNumberOrTag {
    
    Latest,
    
    Finalized,
    
    Safe,
    
    Earliest,
    
    Pending,
    
    Number(crate::prims::Quantity),
}

#[serde(default, rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct BlockOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "blockNumber")]
    pub number: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<crate::prims::Quantity>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            alias = "timestamp",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub time: Option<crate::prims::Quantity>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub gas_limit: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "feeRecipient")]
    pub coinbase: Option<crate::prims::Hex20>,
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "prevRandao")]
    pub random: Option<crate::prims::Hex32>,
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "baseFeePerGas")]
    pub base_fee: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob_base_fee: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "parentBeaconBlockRoot")]
    pub beacon_root: Option<crate::prims::Hex32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<BTreeMap<crate::prims::Quantity, crate::prims::Hex32>>,
}

pub type BloomFilter = (Vec<crate::prims::Hex>);


#[derive(JsonSchema)]
pub struct Bundle<TxReq>
where TxReq: JsonSchema {
    
    pub transactions: Vec<TransactionRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_override: Option<BlockOverrides>,
}

pub type Bytes48 = crate::prims::Any;

#[serde(default, rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct ChainConfig {
    
    pub chain_id: crate::prims::Quantity,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub homestead_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub dao_fork_block: Option<crate::prims::Quantity>,
    
    pub dao_fork_support: bool,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub eip150_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub eip155_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub eip158_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub byzantium_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub constantinople_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub petersburg_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub istanbul_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub muir_glacier_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub berlin_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub london_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub arrow_glacier_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub gray_glacier_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub merge_netsplit_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub shanghai_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub cancun_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub prague_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub osaka_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub amsterdam_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub bogota_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub bpo1_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub bpo2_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub bpo3_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub bpo4_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_u64_opt")]
    pub bpo5_time: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none", with = "alloy_serde::ttd")]
    pub terminal_total_difficulty: Option<crate::prims::Quantity>,
    
    pub terminal_total_difficulty_passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethash: Option<EthashConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clique: Option<CliqueConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parlia: Option<ParliaConfig>,
    #[serde(flatten, default)]
    pub extra_fields: OtherFields,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_contract_address: Option<crate::prims::Hex20>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub blob_schedule: BTreeMap<String, BlobParams>,
    #[serde(skip)]
    pub _non_exhaustive: (),
}


#[derive(JsonSchema)]
pub struct CliqueConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epoch: Option<crate::prims::Quantity>,
}


#[derive(JsonSchema)]
pub struct DevOptions {
    
    pub block_time: Option<crate::prims::Quantity>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
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

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EIP1186StorageProof {
    
    pub key: JsonStorageKey,
    
    pub value: crate::prims::Quantity,
    
    pub proof: Vec<crate::prims::Hex>,
}


#[derive(JsonSchema)]
pub enum Eip658Value {
    
    Eip658(bool),
    
    PostState(crate::prims::Hex32),
}

pub type Empty = crate::prims::Any;


#[derive(JsonSchema)]
pub enum Error {
    
    UnknownFunction(String),
    
    UnknownSelector(crate::prims::Any),
    
    NotADeploymentTransaction,
    
    ContractNotDeployed,
    
    ZeroData(String, crate::prims::Any),
    
    AbiError(crate::prims::Any),
    
    TransportError(crate::prims::Any),
    
    PendingTransactionError(crate::prims::Any),
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthBundleHash {
    
    pub bundle_hash: crate::prims::Hex32,
}


#[derive(JsonSchema)]
pub struct EthCallBundle {
    
    pub txs: Vec<crate::prims::Hex>,
    #[schemars(with = "crate::prims::Quantity")]
    pub block_number: crate::prims::Quantity,
    
    pub state_block_number: BlockNumberOrTag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_index: Option<TransactionIndex>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<crate::prims::Hex20>,
    #[serde(default, with = "alloy_serde::quantity::opt", skip_serializing_if = "Option::is_none")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub timestamp: Option<crate::prims::Quantity>,
    #[serde(default, with = "alloy_serde::quantity::opt", skip_serializing_if = "Option::is_none")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub timeout: Option<crate::prims::Quantity>,
    #[serde(default, with = "alloy_serde::quantity::opt", skip_serializing_if = "Option::is_none")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub gas_limit: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<crate::prims::Quantity>,
    #[serde(default, with = "alloy_serde::quantity::opt", skip_serializing_if = "Option::is_none")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub base_fee: Option<crate::prims::Quantity>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthCallBundleResponse {
    
    pub bundle_hash: crate::prims::Hex32,
    
    pub bundle_gas_price: crate::prims::Quantity,
    
    pub coinbase_diff: crate::prims::Quantity,
    
    pub eth_sent_to_coinbase: crate::prims::Quantity,
    
    pub gas_fees: crate::prims::Quantity,
    
    pub results: Vec<EthCallBundleTransactionResult>,
    
    pub state_block_number: crate::prims::Quantity,
    
    pub total_gas_used: crate::prims::Quantity,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthCallBundleTransactionResult {
    
    pub coinbase_diff: crate::prims::Quantity,
    
    pub eth_sent_to_coinbase: crate::prims::Quantity,
    
    pub from_address: crate::prims::Hex20,
    
    pub gas_fees: crate::prims::Quantity,
    
    pub gas_price: crate::prims::Quantity,
    
    pub gas_used: crate::prims::Quantity,
    
    pub to_address: Option<crate::prims::Hex20>,
    
    pub tx_hash: crate::prims::Hex32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<crate::prims::Hex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert: Option<crate::prims::Hex>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthCallResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<crate::prims::Hex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthCancelBundle {
    
    pub replacement_uuid: String,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthCancelPrivateTransaction {
    
    pub tx_hash: crate::prims::Hex32,
}


#[derive(JsonSchema)]
pub struct EthCapabilities {
    
    pub head: EthCapabilitiesHead,
    
    pub state: EthCapabilitiesResource,
    
    pub tx: EthCapabilitiesResource,
    
    pub logs: EthCapabilitiesResource,
    
    pub receipts: EthCapabilitiesResource,
    
    pub blocks: EthCapabilitiesResource,
    #[serde(rename = "stateproofs")]
    pub state_proofs: EthCapabilitiesResource,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthCapabilitiesDeleteStrategy {
    #[serde(rename = "type")]
    pub strategy_type: EthCapabilitiesDeleteStrategyKind,
    #[schemars(with = "crate::prims::Quantity")]
    pub retention_blocks: crate::prims::Quantity,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub enum EthCapabilitiesDeleteStrategyKind {
    #[serde(rename = "window")]
    Window,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthCapabilitiesHead {
    #[schemars(with = "crate::prims::Quantity")]
    pub number: crate::prims::Quantity,
    
    pub hash: crate::prims::Hex32,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthCapabilitiesResource {
    
    pub disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_block: Option<crate::prims::Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_strategy: Option<EthCapabilitiesDeleteStrategy>,
}


#[derive(JsonSchema)]
pub struct EthConfig {
    
    pub cache: EthStateCacheConfig,
    
    pub gas_oracle: GasPriceOracleConfig,
    
    pub eth_proof_window: crate::prims::Quantity,
    
    pub max_tracing_requests: usize,
    
    pub max_blocking_io_requests: usize,
    
    pub max_trace_filter_blocks: crate::prims::Quantity,
    
    pub max_blocks_per_filter: crate::prims::Quantity,
    
    pub max_logs_per_response: usize,
    
    pub rpc_gas_cap: crate::prims::Quantity,
    
    pub rpc_max_simulate_blocks: crate::prims::Quantity,
    
    pub compute_state_root_for_eth_simulate: bool,
    
    pub stale_filter_ttl: crate::prims::Any,
    
    pub fee_history_cache: FeeHistoryCacheConfig,
    
    pub proof_permits: usize,
    
    pub max_batch_size: usize,
    
    pub pending_block_kind: PendingBlockKind,
    
    pub raw_tx_forwarder: ForwardConfig,
    
    pub send_raw_transaction_sync_timeout: crate::prims::Any,
    
    pub rpc_evm_memory_limit: crate::prims::Quantity,
    
    pub force_blob_sidecar_upcasting: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthSendBundle {
    
    pub txs: Vec<crate::prims::Hex>,
    #[schemars(with = "crate::prims::Quantity")]
    pub block_number: crate::prims::Quantity,
    #[serde(
        default,
        deserialize_with = "alloy_serde::quantity::opt::deserialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_timestamp: Option<crate::prims::Quantity>,
    #[serde(
        default,
        deserialize_with = "alloy_serde::quantity::opt::deserialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_timestamp: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reverting_tx_hashes: Vec<crate::prims::Hex32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dropping_tx_hashes: Vec<crate::prims::Hex32>,
    #[serde(
        default,
        deserialize_with = "alloy_serde::quantity::opt::deserialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub refund_percent: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_recipient: Option<crate::prims::Hex20>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refund_tx_hashes: Vec<crate::prims::Hex32>,
    #[serde(flatten, default)]
    pub extra_fields: OtherFields,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct EthSendPrivateTransaction {
    
    pub tx: crate::prims::Hex,
    #[serde(default, with = "alloy_serde::quantity::opt", skip_serializing_if = "Option::is_none")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_block_number: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "PrivateTransactionPreferences::is_empty")]
    pub preferences: PrivateTransactionPreferences,
}


#[derive(JsonSchema)]
pub struct EthStateCacheConfig {
    
    pub max_blocks: crate::prims::Quantity,
    
    pub max_receipts: crate::prims::Quantity,
    
    pub max_bals: crate::prims::Quantity,
    
    pub max_blocks_bytes: usize,
    
    pub max_receipts_bytes: usize,
    
    pub max_bals_bytes: usize,
    
    pub idle_timeout: crate::prims::Any,
    #[serde(default)]
    pub cache_computed_bals: bool,
    #[serde(default)]
    pub prewarm_bals: Option<usize>,
    
    pub max_concurrent_db_requests: usize,
    
    pub max_cached_tx_hashes: crate::prims::Quantity,
}


#[derive(JsonSchema)]
pub struct EthashConfig {
    ,
}


#[derive(JsonSchema)]
pub struct Ethereum {
    
    pub _private: (),
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct FeeHistory {
    #[serde(default, skip_serializing_if = "Vec::is_empty", with = "alloy_serde::quantity::vec")]#[schemars(with = "Vec<crate::prims::Quantity>")]
    pub base_fee_per_gas: Vec<crate::prims::Quantity>,
    #[serde(deserialize_with = "alloy_serde::null_as_default")]
    pub gas_used_ratio: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", with = "alloy_serde::quantity::vec")]#[schemars(with = "Vec<crate::prims::Quantity>")]
    pub base_fee_per_blob_gas: Vec<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blob_gas_used_ratio: Vec<f64>,
    #[serde(default, with = "alloy_serde::quantity")]#[schemars(with = "crate::prims::Quantity")]
    pub oldest_block: crate::prims::Quantity,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::u128_vec_vec_opt"
        )]#[schemars(with = "Option<Vec<Vec<crate::prims::Quantity>>>")]
    pub reward: Option<Vec<Vec<crate::prims::Quantity>>>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct FeeHistoryCacheConfig {
    
    pub max_blocks: crate::prims::Quantity,
    
    pub resolution: crate::prims::Quantity,
}


#[derive(JsonSchema)]
pub struct FillTransaction<T>
where T: JsonSchema {
    
    pub raw: crate::prims::Hex,
    
    pub tx: TypedTransaction,
}


#[derive(JsonSchema)]
pub struct Filter {
    
    pub block_option: FilterBlockOption,
    
    pub address: FilterSet,
    
    pub topics: Vec<Topic>,
}


#[derive(JsonSchema)]
pub enum FilterBlockOption {
    
    Range { pub from_block: Option<BlockNumberOrTag>, pub to_block: Option<BlockNumberOrTag> },
    
    AtBlockHash(crate::prims::Hex32),
}

#[serde(untagged)]
#[derive(JsonSchema)]
pub enum FilterChanges<T, L>
where T: JsonSchema, L: JsonSchema {
    
    Empty,
    
    Logs(Vec<RpcLog>),
    
    Hashes(Vec<crate::prims::Hex32>),
    
    Transactions(Vec<Transaction>),
}

#[serde(untagged)]
#[derive(JsonSchema)]
pub enum FilterId {
    
    Num(crate::prims::Quantity),
    
    Str(String),
}


#[derive(JsonSchema)]
pub struct FilterSet<T>
where T: JsonSchema {
    
    pub set: HashSet<crate::prims::Any>,
    #[serde(skip, default)]
    pub bloom_filter: crate::prims::Any,
}


#[derive(JsonSchema)]
pub struct ForwardConfig {
    
    pub tx_forwarder: Option<crate::prims::Any>,
}


#[derive(JsonSchema)]
pub struct GasPriceOracleConfig {
    
    pub blocks: crate::prims::Quantity,
    
    pub percentile: crate::prims::Quantity,
    
    pub max_header_history: crate::prims::Quantity,
    
    pub max_block_history: crate::prims::Quantity,
    
    pub max_reward_percentile_count: crate::prims::Quantity,
    
    pub default_suggested_fee: Option<crate::prims::Quantity>,
    
    pub max_price: Option<crate::prims::Quantity>,
    
    pub ignore_price: Option<crate::prims::Quantity>,
}


#[derive(JsonSchema)]
pub struct Genesis {
    #[serde(default)]
    pub config: ChainConfig,
    #[schemars(with = "crate::prims::Quantity")]
    pub nonce: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub timestamp: crate::prims::Quantity,
    
    pub extra_data: crate::prims::Hex,
    #[schemars(with = "crate::prims::Quantity")]
    pub gas_limit: crate::prims::Quantity,
    
    pub difficulty: crate::prims::Quantity,
    
    pub mix_hash: crate::prims::Hex32,
    
    pub coinbase: crate::prims::Hex20,
    
    pub alloc: BTreeMap<crate::prims::Hex20, GenesisAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub base_fee_per_gas: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub excess_blob_gas: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub blob_gas_used: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub slot_number: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub number: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_hash: Option<crate::prims::Hex32>,
}


#[derive(JsonSchema)]
pub struct GenesisAccount {
    #[serde(skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt", default)]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub nonce: Option<crate::prims::Quantity>,
    
    pub balance: crate::prims::Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<crate::prims::Hex>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_storage_map"
    )]
    pub storage: Option<BTreeMap<crate::prims::Hex32, crate::prims::Hex32>>,
    #[serde(
        rename = "secretKey",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_private_key"
    )]
    pub private_key: Option<crate::prims::Hex32>,
    #[serde(default, skip_serializing_if = "alloy_trie::AccountExtension::is_empty")]
    pub extension: crate::prims::Any,
}


#[derive(JsonSchema)]
pub struct Geth {
    
    pub program: Option<crate::prims::Any>,
    
    pub host: Option<String>,
    
    pub port: Option<crate::prims::Quantity>,
    
    pub authrpc_port: Option<crate::prims::Quantity>,
    
    pub ipc_path: Option<crate::prims::Any>,
    
    pub ipc_enabled: bool,
    
    pub data_dir: Option<crate::prims::Any>,
    
    pub chain_id: Option<crate::prims::Quantity>,
    
    pub insecure_unlock: bool,
    
    pub keep_err: bool,
    
    pub timeout: Option<crate::prims::Quantity>,
    
    pub genesis: Option<Genesis>,
    
    pub mode: NodeMode,
    
    pub clique_private_key: Option<crate::prims::Any>,
    
    pub args: Vec<crate::prims::Any>,
}


#[derive(JsonSchema)]
pub struct Header {
    
    pub parent_hash: crate::prims::Hex32,
    #[serde(rename = "sha3Uncles", alias = "ommersHash")]
    pub ommers_hash: crate::prims::Hex32,
    #[serde(rename = "miner", alias = "beneficiary")]
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
    #[serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub base_fee_per_gas: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawals_root: Option<crate::prims::Hex32>,
    #[serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub blob_gas_used: Option<crate::prims::Quantity>,
    #[serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub excess_blob_gas: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_beacon_block_root: Option<crate::prims::Hex32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests_hash: Option<crate::prims::Hex32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_access_list_hash: Option<crate::prims::Hex32>,
    #[serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub slot_number: Option<crate::prims::Quantity>,
}


#[derive(JsonSchema)]
pub struct HttpError {
    
    pub status: crate::prims::Quantity,
    
    pub body: String,
}


#[derive(JsonSchema)]
pub enum Id {
    
    Number(crate::prims::Quantity),
    
    String(String),
    
    None,
}

pub type Index = (crate::prims::Any);


#[derive(JsonSchema)]
pub enum JsonStorageKey {
    
    Hash(crate::prims::Hex32),
    
    Number(crate::prims::Quantity),
}


#[derive(JsonSchema)]
pub struct Log {
    
    pub address: crate::prims::Hex20,
    
    pub topics: Vec<crate::prims::Hex32>,
    
    pub data: crate::prims::Hex,
}


#[derive(JsonSchema)]
pub enum NodeMode {
    
    Dev(DevOptions),
    
    NonDev(PrivateNetOptions),
}

pub type Optional = (Option<crate::prims::Any>);


#[derive(JsonSchema)]
pub struct OtherFields {
    
    pub inner: BTreeMap<String, crate::prims::Any>,
}


#[derive(JsonSchema)]
pub enum Params {
    
    None,
    
    Logs(Box<Filter>),
    
    Bool(bool),
    
    TransactionReceipts(TransactionReceiptsParams),
}


#[derive(JsonSchema)]
pub struct ParliaConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epoch: Option<crate::prims::Quantity>,
}


#[derive(JsonSchema)]
pub enum PendingBlockKind {
    
    Empty,
    
    None,
    
    Full,
}


#[derive(JsonSchema)]
pub enum PendingTransactionError {
    
    FailedToRegister,
    
    TransportError(crate::prims::Any),
    
    Recv(crate::prims::Any),
    
    TxWatcher(crate::prims::Any),
}


#[derive(JsonSchema)]
pub enum PendingTransactionFilterKind {
    
    Hashes,
    
    Full,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct Privacy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<PrivacyHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builders: Option<Vec<String>>,
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
pub struct PrivateNetOptions {
    
    pub p2p_port: Option<crate::prims::Quantity>,
    
    pub discovery: bool,
}


#[derive(JsonSchema)]
pub struct PrivateTransactionPreferences {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fast: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity: Option<Validity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
}

#[serde(untagged)]
#[derive(JsonSchema)]
pub enum PubSubSyncStatus {
    
    Simple(bool),
    
    Detailed(SyncStatusMetadata),
}


#[derive(JsonSchema)]
pub struct Receipt<T>
where T: JsonSchema {
    #[serde(flatten)]
    pub status: Eip658Value,
    #[schemars(with = "crate::prims::Quantity")]
    pub cumulative_gas_used: crate::prims::Quantity,
    
    pub logs: Vec<Log>,
}


#[derive(JsonSchema)]
pub struct Receipts<T>
where T: JsonSchema {
    
    pub receipt_vec: Vec<Vec<crate::prims::Any>>,
}


#[derive(JsonSchema)]
pub struct Recovered<T>
where T: JsonSchema {
    
    pub signer: crate::prims::Hex20,
    #[serde(flatten)]
    pub inner: crate::prims::Any,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct Refund {
    #[schemars(with = "crate::prims::Quantity")]
    pub body_idx: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub percent: crate::prims::Quantity,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct RefundConfig {
    
    pub address: crate::prims::Hex20,
    #[schemars(with = "crate::prims::Quantity")]
    pub percent: crate::prims::Quantity,
}

pub type Requests = (Vec<crate::prims::Hex>);

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct RpcBlockHash {
    
    pub block_hash: crate::prims::Hex32,
    
    pub require_canonical: Option<bool>,
}

pub type RpcError = crate::prims::Any;


#[derive(JsonSchema)]
pub struct Signed<T, Sig>
where T: JsonSchema, Sig: JsonSchema {
    
    pub tx: crate::prims::Any,
    
    pub signature: Signature,
    
    pub hash: crate::prims::Any,
}


#[derive(JsonSchema)]
pub struct SimBlock<TxReq>
where TxReq: JsonSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_overrides: Option<BlockOverrides>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_overrides: Option<StateOverride>,
    #[serde(default = "Vec::new")]
    pub calls: Vec<TransactionRequest>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct SimCallResult {
    
    pub return_data: crate::prims::Hex,
    #[serde(default)]
    pub logs: Vec<Log>,
    #[schemars(with = "crate::prims::Quantity")]
    pub gas_used: crate::prims::Quantity,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_used_gas: Option<crate::prims::Quantity>,
    #[schemars(with = "crate::prims::Quantity")]
    pub status: crate::prims::Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<SimulateError>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct SimulateError {
    
    pub code: i32,
    
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::prims::Hex>,
}

#[serde(
        rename_all = "camelCase",
        bound(
            deserialize = "TxReq: serde::Deserialize<'de>",
            serialize = "TxReq: serde::Serialize"
        )
    )]
#[derive(JsonSchema)]
pub struct SimulatePayload<TxReq>
where TxReq: JsonSchema {
    #[serde(default)]
    pub block_state_calls: Vec<SimBlock>,
    #[serde(default)]
    pub trace_transfers: bool,
    #[serde(default)]
    pub validation: bool,
    #[serde(default)]
    pub return_full_transactions: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct SimulatedBlock<B>
where B: JsonSchema {
    #[serde(flatten)]
    pub inner: Block,
    
    pub calls: Vec<SimCallResult>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct Stage {
    
    pub name: String,
    #[schemars(with = "crate::prims::Quantity")]
    pub block: crate::prims::Quantity,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct StateContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<BlockId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_index: Option<TransactionIndex>,
}

pub type StateOverride = crate::prims::Any;

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub enum SubscriptionKind {
    
    NewHeads,
    
    Logs,
    
    NewPendingTransactions,
    
    Syncing,
    
    TransactionReceipts,
}


#[derive(JsonSchema)]
pub enum SubscriptionResult<T, R>
where T: JsonSchema, R: JsonSchema {
    
    Header(Box<WithOtherFields>),
    
    Log(Box<Log>),
    
    TransactionHash(crate::prims::Hex32),
    
    FullTransaction(Box<Transaction>),
    
    SyncState(PubSubSyncStatus),
    
    TransactionReceipts(Vec<TransactionReceipt>),
}


#[derive(JsonSchema)]
pub struct SyncInfo {
    
    pub starting_block: crate::prims::Quantity,
    
    pub current_block: crate::prims::Quantity,
    
    pub highest_block: crate::prims::Quantity,
    
    pub warp_chunks_amount: Option<crate::prims::Quantity>,
    
    pub warp_chunks_processed: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<Stage>>,
}


#[derive(JsonSchema)]
pub enum SyncStatus {
    
    Info(Box<SyncInfo>),
    
    None,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct SyncStatusMetadata {
    
    pub syncing: bool,
    #[schemars(with = "crate::prims::Quantity")]
    pub starting_block: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub current_block: crate::prims::Quantity,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub highest_block: Option<crate::prims::Quantity>,
}


#[derive(JsonSchema)]
pub struct Target {
    
    pub epoch: crate::prims::Quantity,
    
    pub root: crate::prims::Hex32,
}

pub type Topic = crate::prims::Any;


#[derive(JsonSchema)]
pub struct Transaction<T>
where T: JsonSchema {
    
    pub inner: Recovered,
    
    pub block_hash: Option<crate::prims::Hex32>,
    
    pub block_number: Option<crate::prims::Quantity>,
    
    pub transaction_index: Option<crate::prims::Quantity>,
    
    pub effective_gas_price: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<crate::prims::Quantity>,
}


#[derive(JsonSchema)]
pub struct TransactionConditional {
    #[serde(default)]
    pub known_accounts: crate::prims::Any,
    #[serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub block_number_min: Option<crate::prims::Quantity>,
    #[serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub block_number_max: Option<crate::prims::Quantity>,
    #[serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub timestamp_min: Option<crate::prims::Quantity>,
    #[serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub timestamp_max: Option<crate::prims::Quantity>,
}


#[derive(JsonSchema)]
pub enum TransactionIndex {
    
    All,
    
    Index(usize),
}


#[derive(JsonSchema)]
pub struct TransactionInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<crate::prims::Hex>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::prims::Hex>,
}


#[derive(JsonSchema)]
pub struct TransactionReceipt<T>
where T: JsonSchema {
    #[serde(flatten)]
    pub inner: ReceiptEnvelope<Log>,
    
    pub transaction_hash: crate::prims::Hex32,
    #[serde(default, with = "alloy_serde::quantity::opt")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub transaction_index: Option<crate::prims::Quantity>,
    #[serde(default)]
    pub block_hash: Option<crate::prims::Hex32>,
    #[serde(default, with = "alloy_serde::quantity::opt")]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub block_number: Option<crate::prims::Quantity>,
    #[schemars(with = "crate::prims::Quantity")]
    pub gas_used: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub effective_gas_price: crate::prims::Quantity,
    #[serde(
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt",
            default
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub blob_gas_used: Option<crate::prims::Quantity>,
    #[serde(
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt",
            default
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub blob_gas_price: Option<crate::prims::Quantity>,
    
    pub from: crate::prims::Hex20,
    
    pub to: Option<crate::prims::Hex20>,
    
    pub contract_address: Option<crate::prims::Hex20>,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct TransactionReceiptsParams {
    #[serde(default)]
    pub transaction_hashes: Option<Vec<crate::prims::Hex32>>,
}


#[derive(JsonSchema)]
pub struct TransactionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<crate::prims::Hex20>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::prims::Any>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub gas_price: Option<crate::prims::Quantity>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_fee_per_gas: Option<crate::prims::Quantity>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_priority_fee_per_gas: Option<crate::prims::Quantity>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub max_fee_per_blob_gas: Option<crate::prims::Quantity>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub gas: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<crate::prims::Quantity>,
    #[serde(default, flatten)]
    pub input: TransactionInput,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub nonce: Option<crate::prims::Quantity>,
    #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub chain_id: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_list: Option<crate::prims::Any>,
    #[serde(
            default,
            rename = "type",
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]#[schemars(with = "Option<crate::prims::Quantity>")]
    pub transaction_type: Option<crate::prims::Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob_versioned_hashes: Option<Vec<crate::prims::Hex32>>,
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub sidecar: Option<BlobTransactionSidecarVariant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_list: Option<Vec<crate::prims::Any>>,
}

pub type TransportError = crate::prims::Any;


#[derive(JsonSchema)]
pub enum TransportErrorKind {
    
    MissingBatchResponse(Id),
    
    BackendGone,
    
    PubsubUnavailable,
    
    HttpError(crate::prims::Any),
    
    HttpErrorWithRetryAfter { pub error: HttpError, pub retry_after: crate::prims::Any },
    
    Custom(crate::prims::Any),
    
    NonRetryable(crate::prims::Any),
}


#[derive(JsonSchema)]
pub struct TxpoolContent<T>
where T: JsonSchema {
    
    pub pending: BTreeMap<crate::prims::Hex20, BTreeMap<String, Transaction>>,
    
    pub queued: BTreeMap<crate::prims::Hex20, BTreeMap<String, Transaction>>,
}


#[derive(JsonSchema)]
pub struct TxpoolContentFrom<T>
where T: JsonSchema {
    
    pub pending: BTreeMap<String, Transaction>,
    
    pub queued: BTreeMap<String, Transaction>,
}


#[derive(JsonSchema)]
pub struct TxpoolInspect {
    
    pub pending: BTreeMap<crate::prims::Hex20, BTreeMap<String, TxpoolInspectSummary>>,
    
    pub queued: BTreeMap<crate::prims::Hex20, BTreeMap<String, TxpoolInspectSummary>>,
}


#[derive(JsonSchema)]
pub struct TxpoolInspectSummary {
    
    pub to: Option<crate::prims::Hex20>,
    
    pub value: crate::prims::Quantity,
    
    pub gas: crate::prims::Quantity,
    
    pub gas_price: crate::prims::Quantity,
}


#[derive(JsonSchema)]
pub struct TxpoolStatus {
    #[schemars(with = "crate::prims::Quantity")]
    pub pending: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub queued: crate::prims::Quantity,
}

#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
pub struct Validity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<Vec<Refund>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_config: Option<Vec<RefundConfig>>,
}


#[derive(JsonSchema)]
pub enum WatchTxError {
    
    Timeout,
}


#[derive(JsonSchema)]
pub struct WithOtherFields<T>
where T: JsonSchema {
    #[serde(flatten)]
    pub inner: crate::prims::Any,
    #[serde(flatten)]
    pub other: OtherFields,
}


#[derive(JsonSchema)]
pub struct Withdrawal {
    #[schemars(with = "crate::prims::Quantity")]
    pub index: crate::prims::Quantity,
    #[schemars(with = "crate::prims::Quantity")]
    pub validator_index: crate::prims::Quantity,
    
    pub address: crate::prims::Hex20,
    #[schemars(with = "crate::prims::Quantity")]
    pub amount: crate::prims::Quantity,
}

pub type Withdrawals = (crate::prims::Any);


#[derive(JsonSchema)]
pub struct Work {
    
    pub pow_hash: crate::prims::Hex32,
    
    pub seed_hash: crate::prims::Hex32,
    
    pub target: crate::prims::Hex32,
    
    pub number: Option<crate::prims::Quantity>,
}