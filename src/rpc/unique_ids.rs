use serde::{Deserialize, Serialize};

use crate::errors;
use crate::rpc::{self, IntoReplyBody, MessageType};

#[derive(Deserialize, Debug)]
pub struct GenerateMsgIn {
    pub src: String,
    pub dest: String,
    pub body: GenerateRequestMsg,
}

impl GenerateMsgIn {
    pub fn into_response(self, outbound_msg_id: u64) -> GenerateMsgOut {
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
            .map(|mut msg_out| {
                msg_out.body.value = value;
                msg_out
            })
            .map_err(errors::ErrorMsg::json_parse_error)?;
        serde_json::to_string(&msg_out).map_err(errors::ErrorMsg::json_dumps_error)
    }
}

#[derive(Serialize, Debug)]
pub struct GenerateMsgOut {
    pub src: String,
    pub dest: String,
    pub body: GenerateResponseMsg,
}

/// Outbound message type strings
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GenerateMsgType {
    GenerateOk,
}

/// The only allowed values for our messages below
/// are in these newtypes. These are not-public.
#[derive(Deserialize, Debug)]
struct Generate(MessageType);

#[derive(Serialize, Debug)]
struct GenerateOk(GenerateMsgType);

#[derive(Deserialize, Debug)]
pub struct GenerateRequestMsg {
    #[serde(rename = "type")]
    _typ: Generate,
    pub msg_id: u64,
}

impl GenerateRequestMsg {
    pub fn new(msg_id: u64) -> Self {
        GenerateRequestMsg {
            _typ: Generate(MessageType::Generate),
            msg_id,
        }
    }
}

impl rpc::IntoReplyBody for GenerateRequestMsg {
    type Item = GenerateResponseMsg;
    fn into_reply(self, outbound_msg_id: u64) -> GenerateResponseMsg {
        GenerateResponseMsg {
            typ: GenerateOk(GenerateMsgType::GenerateOk),
            msg_id: outbound_msg_id,
            in_reply_to: Some(self.msg_id),
            value: "".to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct GenerateResponseMsg {
    #[serde(rename = "type")]
    typ: GenerateOk,
    in_reply_to: Option<u64>,
    msg_id: u64,
    #[serde(rename = "id")]
    pub value: String,
}
impl rpc::Reply for GenerateResponseMsg {}
