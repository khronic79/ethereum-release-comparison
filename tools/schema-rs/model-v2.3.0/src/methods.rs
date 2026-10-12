use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
#[allow(unused_imports)]
use crate::model::*;

pub fn build() -> Value {
    let mut gen = SchemaGenerator::default();
    let mut methods = serde_json::Map::new();
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Vec<crate::prims::Hex20>>()).unwrap();
        methods.insert("eth_accounts".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Quantity>>()).unwrap();
        methods.insert("eth_baseFee".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("eth_blobBaseFee".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("eth_blockNumber".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<TransactionRequest>()).unwrap());
        props.insert("request".to_string(), prefix[0].clone());
        required.push(json!("request"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_id".to_string(), prefix[1].clone());
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<StateOverride>>()).unwrap());
        props.insert("state_overrides".to_string(), prefix[2].clone());
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<Box<BlockOverrides>>>()).unwrap());
        props.insert("block_overrides".to_string(), prefix[3].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":4,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap();
        methods.insert("eth_call".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<EthCallBundle>()).unwrap());
        props.insert("request".to_string(), prefix[0].clone());
        required.push(json!("request"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<EthCallBundleResponse>()).unwrap();
        methods.insert("eth_callBundle".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<Vec<Bundle>>()).unwrap());
        props.insert("bundles".to_string(), prefix[0].clone());
        required.push(json!("bundles"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<StateContext>>()).unwrap());
        props.insert("state_context".to_string(), prefix[1].clone());
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<StateOverride>>()).unwrap());
        props.insert("state_override".to_string(), prefix[2].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":3,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Vec<Vec<EthCallResponse>>>()).unwrap();
        methods.insert("eth_callMany".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<EthCancelBundle>()).unwrap());
        props.insert("request".to_string(), prefix[0].clone());
        required.push(json!("request"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<()>()).unwrap();
        methods.insert("eth_cancelBundle".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<EthCancelPrivateTransaction>()).unwrap());
        props.insert("request".to_string(), prefix[0].clone());
        required.push(json!("request"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<bool>()).unwrap();
        methods.insert("eth_cancelPrivateTransaction".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<EthCapabilities>()).unwrap();
        methods.insert("eth_capabilities".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Quantity>>()).unwrap();
        methods.insert("eth_chainId".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap();
        methods.insert("eth_coinbase".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<EthConfig>()).unwrap();
        methods.insert("eth_config".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<TransactionRequest>()).unwrap());
        props.insert("request".to_string(), prefix[0].clone());
        required.push(json!("request"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_number".to_string(), prefix[1].clone());
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<StateOverride>>()).unwrap());
        props.insert("state_override".to_string(), prefix[2].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":3,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Any>()).unwrap();
        methods.insert("eth_createAccessList".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<TransactionRequest>()).unwrap());
        props.insert("request".to_string(), prefix[0].clone());
        required.push(json!("request"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_number".to_string(), prefix[1].clone());
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<StateOverride>>()).unwrap());
        props.insert("state_override".to_string(), prefix[2].clone());
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<Box<BlockOverrides>>>()).unwrap());
        props.insert("block_overrides".to_string(), prefix[3].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":4,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("eth_estimateGas".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap());
        props.insert("block_count".to_string(), prefix[0].clone());
        required.push(json!("block_count"));
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockNumberOrTag>()).unwrap());
        props.insert("newest_block".to_string(), prefix[1].clone());
        required.push(json!("newest_block"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<Vec<f64>>>()).unwrap());
        props.insert("reward_percentiles".to_string(), prefix[2].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":3,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<FeeHistory>()).unwrap();
        methods.insert("eth_feeHistory".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<TransactionRequest>()).unwrap());
        props.insert("request".to_string(), prefix[0].clone());
        required.push(json!("request"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<FillTransaction>()).unwrap();
        methods.insert("eth_fillTransaction".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("eth_gasPrice".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockId>()).unwrap());
        props.insert("block".to_string(), prefix[1].clone());
        required.push(json!("block"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Account>>()).unwrap();
        methods.insert("eth_getAccount".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockId>()).unwrap());
        props.insert("block".to_string(), prefix[1].clone());
        required.push(json!("block"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<AccountInfo>()).unwrap();
        methods.insert("eth_getAccountInfo".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_number".to_string(), prefix[1].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("eth_getBalance".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockId>()).unwrap());
        props.insert("block_id".to_string(), prefix[0].clone());
        required.push(json!("block_id"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Any>>()).unwrap();
        methods.insert("eth_getBlockAccessList".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Any>>()).unwrap();
        methods.insert("eth_getBlockAccessListByBlockHash".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockNumberOrTag>()).unwrap());
        props.insert("number".to_string(), prefix[0].clone());
        required.push(json!("number"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Any>>()).unwrap();
        methods.insert("eth_getBlockAccessListByBlockNumber".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockId>()).unwrap());
        props.insert("block".to_string(), prefix[0].clone());
        required.push(json!("block"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Hex>>()).unwrap();
        methods.insert("eth_getBlockAccessListRaw".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        prefix.push(serde_json::to_value(gen.subschema_for::<bool>()).unwrap());
        props.insert("full".to_string(), prefix[1].clone());
        required.push(json!("full"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Block>>()).unwrap();
        methods.insert("eth_getBlockByHash".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockNumberOrTag>()).unwrap());
        props.insert("number".to_string(), prefix[0].clone());
        required.push(json!("number"));
        prefix.push(serde_json::to_value(gen.subschema_for::<bool>()).unwrap());
        props.insert("full".to_string(), prefix[1].clone());
        required.push(json!("full"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Block>>()).unwrap();
        methods.insert("eth_getBlockByNumber".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockId>()).unwrap());
        props.insert("block_id".to_string(), prefix[0].clone());
        required.push(json!("block_id"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Vec<TransactionReceipt>>>()).unwrap();
        methods.insert("eth_getBlockReceipts".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Quantity>>()).unwrap();
        methods.insert("eth_getBlockTransactionCountByHash".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockNumberOrTag>()).unwrap());
        props.insert("number".to_string(), prefix[0].clone());
        required.push(json!("number"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Quantity>>()).unwrap();
        methods.insert("eth_getBlockTransactionCountByNumber".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_id".to_string(), prefix[1].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap();
        methods.insert("eth_getCode".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<FilterId>()).unwrap());
        props.insert("id".to_string(), prefix[0].clone());
        required.push(json!("id"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<FilterChanges>()).unwrap();
        methods.insert("eth_getFilterChanges".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<FilterId>()).unwrap());
        props.insert("id".to_string(), prefix[0].clone());
        required.push(json!("id"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Vec<Log>>()).unwrap();
        methods.insert("eth_getFilterLogs".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Header>>()).unwrap();
        methods.insert("eth_getHeaderByHash".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockNumberOrTag>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Header>>()).unwrap();
        methods.insert("eth_getHeaderByNumber".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<Filter>()).unwrap());
        props.insert("filter".to_string(), prefix[0].clone());
        required.push(json!("filter"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Vec<Log>>()).unwrap();
        methods.insert("eth_getLogs".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Vec<JsonStorageKey>>()).unwrap());
        props.insert("keys".to_string(), prefix[1].clone());
        required.push(json!("keys"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_number".to_string(), prefix[2].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":3,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<EIP1186AccountProofResponse>()).unwrap();
        methods.insert("eth_getProof".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Index>()).unwrap());
        props.insert("index".to_string(), prefix[1].clone());
        required.push(json!("index"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Hex>>()).unwrap();
        methods.insert("eth_getRawTransactionByBlockHashAndIndex".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockNumberOrTag>()).unwrap());
        props.insert("number".to_string(), prefix[0].clone());
        required.push(json!("number"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Index>()).unwrap());
        props.insert("index".to_string(), prefix[1].clone());
        required.push(json!("index"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Hex>>()).unwrap();
        methods.insert("eth_getRawTransactionByBlockNumberAndIndex".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Hex>>()).unwrap();
        methods.insert("eth_getRawTransactionByHash".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<JsonStorageKey>()).unwrap());
        props.insert("index".to_string(), prefix[1].clone());
        required.push(json!("index"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_number".to_string(), prefix[2].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":3,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap();
        methods.insert("eth_getStorageAt".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<HashMap<crate::prims::Hex20, Vec<JsonStorageKey>>>()).unwrap());
        props.insert("requests".to_string(), prefix[0].clone());
        required.push(json!("requests"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_number".to_string(), prefix[1].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<HashMap<crate::prims::Hex20, Vec<crate::prims::Hex32>>>()).unwrap();
        methods.insert("eth_getStorageValues".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Index>()).unwrap());
        props.insert("index".to_string(), prefix[1].clone());
        required.push(json!("index"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Transaction>>()).unwrap();
        methods.insert("eth_getTransactionByBlockHashAndIndex".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockNumberOrTag>()).unwrap());
        props.insert("number".to_string(), prefix[0].clone());
        required.push(json!("number"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Index>()).unwrap());
        props.insert("index".to_string(), prefix[1].clone());
        required.push(json!("index"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Transaction>>()).unwrap();
        methods.insert("eth_getTransactionByBlockNumberAndIndex".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Transaction>>()).unwrap();
        methods.insert("eth_getTransactionByHash".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap());
        props.insert("nonce".to_string(), prefix[1].clone());
        required.push(json!("nonce"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Transaction>>()).unwrap();
        methods.insert("eth_getTransactionBySenderAndNonce".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_number".to_string(), prefix[1].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("eth_getTransactionCount".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<TransactionReceipt>>()).unwrap();
        methods.insert("eth_getTransactionReceipt".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Index>()).unwrap());
        props.insert("index".to_string(), prefix[1].clone());
        required.push(json!("index"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Block>>()).unwrap();
        methods.insert("eth_getUncleByBlockHashAndIndex".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockNumberOrTag>()).unwrap());
        props.insert("number".to_string(), prefix[0].clone());
        required.push(json!("number"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Index>()).unwrap());
        props.insert("index".to_string(), prefix[1].clone());
        required.push(json!("index"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<Block>>()).unwrap();
        methods.insert("eth_getUncleByBlockNumberAndIndex".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("hash".to_string(), prefix[0].clone());
        required.push(json!("hash"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Quantity>>()).unwrap();
        methods.insert("eth_getUncleCountByBlockHash".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<BlockNumberOrTag>()).unwrap());
        props.insert("number".to_string(), prefix[0].clone());
        required.push(json!("number"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Option<crate::prims::Quantity>>()).unwrap();
        methods.insert("eth_getUncleCountByBlockNumber".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Work>()).unwrap();
        methods.insert("eth_getWork".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("eth_hashrate".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("eth_maxPriorityFeePerGas".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<bool>()).unwrap();
        methods.insert("eth_mining".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<FilterId>()).unwrap();
        methods.insert("eth_newBlockFilter".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<Filter>()).unwrap());
        props.insert("filter".to_string(), prefix[0].clone());
        required.push(json!("filter"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<FilterId>()).unwrap();
        methods.insert("eth_newFilter".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<PendingTransactionFilterKind>>()).unwrap());
        props.insert("kind".to_string(), prefix[0].clone());
        required.push(json!("kind"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<FilterId>()).unwrap();
        methods.insert("eth_newPendingTransactionFilter".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Vec<Transaction>>()).unwrap();
        methods.insert("eth_pendingTransactions".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("eth_protocolVersion".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<EthSendBundle>()).unwrap());
        props.insert("bundle".to_string(), prefix[0].clone());
        required.push(json!("bundle"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<EthBundleHash>()).unwrap();
        methods.insert("eth_sendBundle".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap());
        props.insert("bytes".to_string(), prefix[0].clone());
        required.push(json!("bytes"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap();
        methods.insert("eth_sendPrivateRawTransaction".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<EthSendPrivateTransaction>()).unwrap());
        props.insert("request".to_string(), prefix[0].clone());
        required.push(json!("request"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap();
        methods.insert("eth_sendPrivateTransaction".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap());
        props.insert("bytes".to_string(), prefix[0].clone());
        required.push(json!("bytes"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap();
        methods.insert("eth_sendRawTransaction".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap());
        props.insert("bytes".to_string(), prefix[0].clone());
        required.push(json!("bytes"));
        prefix.push(serde_json::to_value(gen.subschema_for::<TransactionConditional>()).unwrap());
        props.insert("condition".to_string(), prefix[1].clone());
        required.push(json!("condition"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap();
        methods.insert("eth_sendRawTransactionConditional".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap());
        props.insert("bytes".to_string(), prefix[0].clone());
        required.push(json!("bytes"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<TransactionReceipt>()).unwrap();
        methods.insert("eth_sendRawTransactionSync".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<TransactionRequest>()).unwrap());
        props.insert("request".to_string(), prefix[0].clone());
        required.push(json!("request"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap();
        methods.insert("eth_sendTransaction".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap());
        props.insert("message".to_string(), prefix[1].clone());
        required.push(json!("message"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap();
        methods.insert("eth_sign".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<TransactionRequest>()).unwrap());
        props.insert("transaction".to_string(), prefix[0].clone());
        required.push(json!("transaction"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap();
        methods.insert("eth_signTransaction".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("address".to_string(), prefix[0].clone());
        required.push(json!("address"));
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Any>()).unwrap());
        props.insert("data".to_string(), prefix[1].clone());
        required.push(json!("data"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap();
        methods.insert("eth_signTypedData".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<SimulatePayload>()).unwrap());
        props.insert("opts".to_string(), prefix[0].clone());
        required.push(json!("opts"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<BlockId>>()).unwrap());
        props.insert("block_number".to_string(), prefix[1].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<Vec<SimulatedBlock>>()).unwrap();
        methods.insert("eth_simulateV1".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap());
        props.insert("hashrate".to_string(), prefix[0].clone());
        required.push(json!("hashrate"));
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("id".to_string(), prefix[1].clone());
        required.push(json!("id"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":2,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<bool>()).unwrap();
        methods.insert("eth_submitHashrate".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap());
        props.insert("nonce".to_string(), prefix[0].clone());
        required.push(json!("nonce"));
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("pow_hash".to_string(), prefix[1].clone());
        required.push(json!("pow_hash"));
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap());
        props.insert("mix_digest".to_string(), prefix[2].clone());
        required.push(json!("mix_digest"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":3,"maxItems":3,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<bool>()).unwrap();
        methods.insert("eth_submitWork".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<SubscriptionKind>()).unwrap());
        props.insert("kind".to_string(), prefix[0].clone());
        required.push(json!("kind"));
        prefix.push(serde_json::to_value(gen.subschema_for::<Option<Params>>()).unwrap());
        props.insert("params".to_string(), prefix[1].clone());
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":2,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<SubscriptionResult>()).unwrap();
        methods.insert("eth_subscribe".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<SyncStatus>()).unwrap();
        methods.insert("eth_syncing".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<FilterId>()).unwrap());
        props.insert("id".to_string(), prefix[0].clone());
        required.push(json!("id"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<bool>()).unwrap();
        methods.insert("eth_uninstallFilter".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<bool>()).unwrap();
        methods.insert("net_listening".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Quantity>()).unwrap();
        methods.insert("net_peerCount".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<String>()).unwrap();
        methods.insert("net_version".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<TxpoolContent>()).unwrap();
        methods.insert("txpool_content".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex20>()).unwrap());
        props.insert("from".to_string(), prefix[0].clone());
        required.push(json!("from"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<TxpoolContentFrom>()).unwrap();
        methods.insert("txpool_contentFrom".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<TxpoolInspect>()).unwrap();
        methods.insert("txpool_inspect".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<TxpoolStatus>()).unwrap();
        methods.insert("txpool_status".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":0,"maxItems":0,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<String>()).unwrap();
        methods.insert("web3_clientVersion".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    {
        let mut prefix: Vec<Value> = Vec::new();
        let mut props = serde_json::Map::new();
        let mut required: Vec<Value> = Vec::new();
        prefix.push(serde_json::to_value(gen.subschema_for::<crate::prims::Hex>()).unwrap());
        props.insert("input".to_string(), prefix[0].clone());
        required.push(json!("input"));
        let arr = json!({"type":"array","prefixItems":prefix,"minItems":1,"maxItems":1,"items":false});
        let obj = json!({"type":"object","properties":props,"required":required,"additionalProperties":false});
        let params_schema = json!({"anyOf":[arr,obj]});
        let result_schema = serde_json::to_value(gen.subschema_for::<crate::prims::Hex32>()).unwrap();
        methods.insert("web3_sha3".to_string(), json!({"params":params_schema,"result":result_schema}));
    }
    let defs = serde_json::to_value(gen.definitions()).unwrap();
    let mut doc = json!({"$schema":"https://json-schema.org/draft/2020-12/schema","methods":methods,"$defs":defs});
    let s = doc.to_string().replace("#/definitions/", "#/$defs/");
    serde_json::from_str(&s).unwrap()
}
