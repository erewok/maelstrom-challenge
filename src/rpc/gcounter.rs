use serde::{Deserialize, Serialize};

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
    msg_id: u64,
    delta: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AddResponseMsg {
    #[serde(rename = "type")]
    typ: String,
    in_reply_to: Option<u64>,
    msg_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ReadMsg {
    #[serde(rename = "type")]
    typ: MsgType,
    msg_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ReadResponseMsg {
    #[serde(rename = "type")]
    typ: String,
    in_reply_to: Option<u64>,
    msg_id: Option<String>,
    value: u64,
}
