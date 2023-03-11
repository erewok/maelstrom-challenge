use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    Topology,
    TopologyOk,
    Broadcast,
    BroadcastOk,
    Read,
    ReadOk,
}

#[derive(Serialize, Deserialize)]
pub struct TopologyRequestMsg {
    #[serde(rename = "type")]
    typ: MsgType,
    msg_id: u64,
    topology: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct TopologyResponseMsg {
    #[serde(rename = "type")]
    typ: String,
    in_reply_to: Option<u64>,
    msg_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BroadcastMsg {
    #[serde(rename = "type")]
    typ: MsgType,
    msg_id: u64,
    message: Value,
}

#[derive(Serialize, Deserialize)]
pub struct BroadcastResponseMsg {
    #[serde(rename = "type")]
    typ: String,
    in_reply_to: Option<u64>,
    msg_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ReadAllMsg {
    #[serde(rename = "type")]
    typ: MsgType,
    msg_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ReadAllResponseMsg {
    #[serde(rename = "type")]
    typ: String,
    in_reply_to: Option<u64>,
    msg_id: Option<String>,
    messages: Vec<Value>,
}
