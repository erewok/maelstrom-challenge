/// Key Value Service
/// Outputs KV instructions to stdout
/// Based on: https://github.com/jepsen-io/maelstrom/blob/main/demo/go/kv.go
/// 
use serde_json::Value;

use crate::errors;

/// KV Actions all get turned into RPC messages where the 
/// KV type is the `dest`
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KvType {
    LinKV,
    SeqKV,
    LwwKV,
}

#[derive(Clone, Debug)]
pub struct KV {
    _type: KvType
}

impl KV {

    pub fn send(msg: KvRpc) -> Result<(), errors::ErrMsg> {
        let msg_str = serde_json::to_string(msg).map_err(errors::ErrMsg::json_dumps_error)?;
        println!("{}", msg_str);
        Ok(())
    }

    pub fn read(key: String) -> Value {

    }
}


/// RPC Messages
/// These represent "actions" for KV
/// As with other RPCs; they will be printed to screen
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KvRpc {
    dest: KvType,
    body: KvRpcBody
}


/// KV Actions all get turned into RPC messages where the 
/// KV type is the `dest`
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum KvRpcBody {
    Read(ReadRequestBody),
    ReadOk(ReadResponseBody),
    Write(WriteRequestBody),
    Cas(CasRequestMsg),
}

pub struct ReadRequestBody {
    key: String,
}

pub struct ReadResponseBody {
    value: Value,
}

pub struct WriteRequestBody {
    key: String,
    value: Value,
}

pub struct CasRequestMsg {
    key: String,
    from: Value,
    to: Value,
    create_if_not_exists: Option<bool>,
}
