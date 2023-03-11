use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    Add,
    AddOk,
    Read,
    ReadOk,
}

#[derive(Serialize, Deserialize)]
pub struct AddRequestMsg {
    #[serde(rename = "type")]
    typ: MsgType,
    msg_id: usize,
    element: Value,
}

#[derive(Serialize, Deserialize)]
pub struct AddResponseMsg {
    #[serde(rename = "type")]
    typ: MsgType,
    in_reply_to: Option<usize>,
    msg_id: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ReadMsg {
    #[serde(rename = "type")]
    typ: MsgType,
    msg_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ReadResponseMsg {
    #[serde(rename = "type")]
    typ: MsgType,
    in_reply_to: Option<usize>,
    msg_id: Option<String>,
    value: Vec<Value>
}