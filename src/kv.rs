/// Key Value Service
/// Outputs KV instructions to stdout
/// Based on: https://github.com/jepsen-io/maelstrom/blob/main/demo/go/kv.go
/// 
use serde::{Deserialize, Serialize};
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

    pub fn new(_type: KvType) -> Self {
        Self { _type }
    }

    pub fn send(&self, msg: KvRpc) -> Result<(), errors::ErrorMsg> {
        let msg_str = serde_json::to_string(&msg).map_err(errors::ErrorMsg::json_dumps_error)?;
        println!("{}", msg_str);
        Ok(())
    }

    pub fn write(&self, key: String, value: Value) -> Result<(), errors::ErrorMsg> {
        self.send(KvRpc {
            dest: self._type.clone(),
            body: KvRpcBody::Write(WriteRequestBody { key, value})
        })
    }

    pub fn read(&self, key: String) -> Result<(), errors::ErrorMsg> {
        self.send(KvRpc {
            dest: self._type.clone(),
            body: KvRpcBody::Read(ReadRequestBody { key })
        })
    }

    pub fn cas(  
        &self,  
        key: String,
        from: Value,
        to: Value,
        create_if_not_exists: Option<bool>
    ) ->  Result<(), errors::ErrorMsg> {
        self.send(KvRpc {
            dest: self._type.clone(),
            body: KvRpcBody::Cas(CasRequestMsg { key, from, to, create_if_not_exists })
        })
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadRequestBody {
    key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadResponseBody {
    value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WriteRequestBody {
    key: String,
    value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CasRequestMsg {
    key: String,
    from: Value,
    to: Value,
    create_if_not_exists: Option<bool>,
}
