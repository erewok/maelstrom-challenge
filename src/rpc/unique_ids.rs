use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    Generate,
    GenerateOk,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateRequestMsg {
    #[serde(rename = "type")]
    typ: MsgType,
    msg_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateResponseMsg {
    #[serde(rename = "type")]
    typ: String,
    in_reply_to: Option<usize>,
    msg_id: Option<String>,
    #[serde(rename = "id")]
    value: Value
}
