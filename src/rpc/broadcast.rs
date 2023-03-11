use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors;
use crate::rpc::{self, IntoReplyBody, MessageType};

use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct BroadcastMsgIn {
    pub src: String,
    pub dest: String,
    pub body: BroadcastMsgRequestBody,
}

#[derive(Serialize, Debug)]
pub struct BroadcastMsgOut {
    pub src: String,
    pub dest: String,
    pub body: BroadcastMsgResponseBody,
}


impl BroadcastMsgIn {
    pub fn into_response(&self, outbound_msg_id: u64) -> BroadcastMsgOut {
        BroadcastMsgOut {
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
                msg_out
            })
            .map_err(|_e| {
                eprintln!("Failing to parse {:?}", _e);
                errors::ErrorMsg::json_parse_error()
            })?;
        serde_json::to_string(&msg_out).map_err(|_e| errors::ErrorMsg::json_dumps_error())
    }
}



#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum BroadcastMsgRequestBody {
    Toplogy(TopologyRequestMsg),
    Broadcast(BroadcastRequestMsg),
    Read(ReadRequestMsg)
}

impl rpc::IntoReplyBody for BroadcastMsgRequestBody {
    type Item = BroadcastMsgResponseBody;

    fn into_reply(&self, outbound_msg_id: u64) -> Self::Item {
        todo!()
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum BroadcastMsgResponseBody {
    Toplogy(TopologyResponseMsg),
    Broadcast(BroadcastResponseMsg),
    Read(ReadResponseMsg)
}

impl rpc::Reply for BroadcastMsgResponseBody {}


#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    TopologyOk,
    BroadcastOk,
    ReadOk,
}

/// The only allowed values for our messages below
/// are in these newtypes. These are not-public.
#[derive(Deserialize, Debug)]
struct Toplogy(MessageType);

/// Topology Request inbound
#[derive(Deserialize, Debug)]
pub struct TopologyRequestMsg {
    #[serde(rename = "type")]
    _typ: Toplogy,
    msg_id: u64,
    topology: HashMap<String, String>,
}

impl rpc::IntoReplyBody for TopologyRequestMsg {
    type Item = TopologyResponseMsg;
    fn into_reply(&self, outbound_msg_id: u64) -> TopologyResponseMsg {
        TopologyResponseMsg {
            typ: ToplogyOk(MsgType::TopologyOk),
            msg_id: outbound_msg_id,
            in_reply_to: Some(self.msg_id),
        }
    }
}


/// Topology Request inbound
#[derive(Serialize, Debug)]
struct ToplogyOk(MsgType);

#[derive(Serialize, Debug)]
pub struct TopologyResponseMsg {
    #[serde(rename = "type")]
    typ: ToplogyOk,
    in_reply_to: Option<u64>,
    msg_id: u64,
}

impl rpc::Reply for TopologyResponseMsg {}

/// Broadcast Request inbound
#[derive(Deserialize, Debug)]
struct Broadcast(MessageType);

#[derive(Deserialize, Debug)]
pub struct BroadcastRequestMsg {
    #[serde(rename = "type")]
    _typ: Broadcast,
    msg_id: u64,
    message: Value,
}


/// Broadcast Response
#[derive(Serialize, Debug)]
struct BroadcastOk(MsgType);


#[derive(Serialize, Debug)]
pub struct BroadcastResponseMsg {
    #[serde(rename = "type")]
    typ: BroadcastOk,
    in_reply_to: Option<u64>,
    msg_id: u64,
}

impl rpc::IntoReplyBody for BroadcastRequestMsg {
    type Item = BroadcastResponseMsg;
    fn into_reply(&self, outbound_msg_id: u64) -> BroadcastResponseMsg {
        BroadcastResponseMsg {
            typ: BroadcastOk(MsgType::BroadcastOk),
            msg_id: outbound_msg_id,
            in_reply_to: Some(self.msg_id),
        }
    }
}

impl rpc::Reply for BroadcastResponseMsg {}


/// Read Request inbound
#[derive(Deserialize, Debug)]
struct Read(MessageType);

#[derive(Deserialize, Debug)]
pub struct ReadRequestMsg {
    #[serde(rename = "type")]
    _typ: Read,
    msg_id: u64,
}

/// Read Response
#[derive(Serialize, Debug)]
struct ReadOk(MsgType);


#[derive(Serialize, Debug)]
pub struct ReadResponseMsg {
    #[serde(rename = "type")]
    typ: ReadOk,
    in_reply_to: Option<u64>,
    msg_id: u64,
    messages: Vec<Value>,
}

impl rpc::Reply for ReadResponseMsg {}