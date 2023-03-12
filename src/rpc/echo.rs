use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors;
use crate::rpc::{self, MessageType};

use super::IntoReplyBody;

/// The only allowed values for our messages below
/// are in these newtypes. These are not-public.
#[derive(Deserialize)]
struct Echo(MessageType);

/// Outbound message type strings
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum EchoMsgResp {
    EchoOk,
}

#[derive(Serialize)]
struct EchoOk(EchoMsgResp);

#[derive(Deserialize)]
pub struct EchoMsgIn {
    pub src: String,
    pub dest: String,
    pub body: EchoRequestMsg,
}

impl EchoMsgIn {
    pub fn into_response(self, outbound_msg_id: u64) -> EchoMsgOut {
        EchoMsgOut {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body: self.body.into_reply(outbound_msg_id),
        }
    }

    pub fn parse_msg_to_str_response(
        msg: &str,
        outbound_msg_id: u64,
    ) -> Result<String, errors::ErrorMsg> {
        let msg_out = serde_json::from_str::<Self>(msg)
            .map(|m| m.into_response(outbound_msg_id))
            .map_err(|_e| errors::ErrorMsg::json_parse_error())?;
        serde_json::to_string(&msg_out).map_err(|_e| errors::ErrorMsg::json_dumps_error())
    }
}

#[derive(Serialize)]
pub struct EchoMsgOut {
    pub src: String,
    pub dest: String,
    pub body: EchoResponseMsg,
}

#[derive(Deserialize)]
pub struct EchoRequestMsg {
    #[serde(rename = "type")]
    _typ: Echo,
    pub msg_id: u64,
    pub echo: Value,
}

impl EchoRequestMsg {
    pub fn new(msg_id: u64, echo: Value) -> Self {
        EchoRequestMsg {
            _typ: Echo(MessageType::Echo),
            msg_id,
            echo,
        }
    }
}

impl rpc::IntoReplyBody for EchoRequestMsg {
    type Item = EchoResponseMsg;
    fn into_reply(self, outbound_msg_id: u64) -> EchoResponseMsg {
        EchoResponseMsg {
            typ: EchoOk(EchoMsgResp::EchoOk),
            msg_id: outbound_msg_id,
            in_reply_to: Some(self.msg_id),
            echo: self.echo,
        }
    }
}

#[derive(Serialize)]
pub struct EchoResponseMsg {
    #[serde(rename = "type")]
    typ: EchoOk,
    pub msg_id: u64,
    pub echo: Value,
    pub in_reply_to: Option<u64>,
}

impl rpc::Reply for EchoResponseMsg {}
