use serde::{Deserialize, Serialize};

use crate::errors;
use crate::rpc::{self, IntoReplyBody, MessageType};

#[derive(Deserialize)]
pub struct GenerateMsgIn {
    pub src: String,
    pub dest: String,
    pub body: GenerateRequestMsg,
}

impl GenerateMsgIn {
    pub fn into_response(&self, outbound_msg_id: u64) -> GenerateMsgOut {
        GenerateMsgOut {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body: self.body.into_reply(outbound_msg_id),
        }
    }

    pub fn parse_msg_to_str_response(
        msg: &str,
        value: String,
        outbound_msg_id: u64,
    ) -> Result<String, errors::ErrorMsg> {
        let msg_out = serde_json::from_str::<Self>(msg)
            .map(|m| m.into_response(outbound_msg_id))
            .map(|mut msg_out| msg_out.body.value = value)
            .map_err(|e| errors::ErrorMsg::json_parse_error())?;
        serde_json::to_string(&msg_out).map_err(|e| errors::ErrorMsg::json_dumps_error())
    }
}

#[derive(Serialize)]
pub struct GenerateMsgOut {
    pub src: String,
    pub dest: String,
    pub body: GenerateResponseMsg,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerateMsgType {
    GenerateOk,
}

/// The only allowed values for our messages below
/// are in these newtypes. These are not-public.
#[derive(Deserialize)]
struct Generate(MessageType);

#[derive(Serialize)]
struct GenerateOk(GenerateMsgType);

#[derive(Deserialize)]
pub struct GenerateRequestMsg {
    #[serde(rename = "type")]
    typ: Generate,
    pub msg_id: u64,
}

impl GenerateRequestMsg {
    pub fn new(msg_id: u64) -> Self {
        GenerateRequestMsg {
            typ: Generate(MessageType::Generate),
            msg_id,
        }
    }
}

impl rpc::IntoReplyBody for GenerateRequestMsg {
    type Item = GenerateResponseMsg;
    fn into_reply(&self, outbound_msg_id: u64) -> GenerateResponseMsg {
        GenerateResponseMsg {
            typ: GenerateOk(GenerateMsgType::GenerateOk),
            msg_id: outbound_msg_id,
            in_reply_to: Some(self.msg_id),
            value: "".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct GenerateResponseMsg {
    #[serde(rename = "type")]
    typ: GenerateOk,
    in_reply_to: Option<u64>,
    msg_id: u64,
    #[serde(rename = "id")]
    value: String,
}
impl rpc::Reply for GenerateResponseMsg {}
