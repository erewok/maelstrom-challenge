use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::errors;
use crate::rpc::{self, IntoReplyBody, MessageType};

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
    pub fn into_response(self, value: Option<Vec<Value>>, outbound_msg_id: u64) -> BroadcastMsgOut {
        let mut body = self.body.into_reply(outbound_msg_id);
        let mut _value = value;
        if _value.is_some() {
            body.set_value(_value.take());
        }
        BroadcastMsgOut {
            src: self.dest,
            dest: self.src,
            body,
        }
    }

    pub fn into_str_response(
        self,
        value: Option<Vec<Value>>,
        outbound_msg_id: u64,
    ) -> Result<String, errors::ErrorMsg> {
        let msg_out = self.into_response(value, outbound_msg_id);
        serde_json::to_string(&msg_out).map_err(|_e| errors::ErrorMsg::json_dumps_error())
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BroadcastMsgRequestBody {
    Topology(TopologyRequestMsg),
    Broadcast(BroadcastRequestMsg),
    Read(ReadRequestMsg),
}

impl rpc::IntoReplyBody for BroadcastMsgRequestBody {
    type Item = BroadcastMsgResponseBody;

    fn into_reply(self, outbound_msg_id: u64) -> Self::Item {
        match self {
            BroadcastMsgRequestBody::Topology(resp) => {
                BroadcastMsgResponseBody::Topology(resp.into_reply(outbound_msg_id))
            }
            BroadcastMsgRequestBody::Broadcast(resp) => {
                BroadcastMsgResponseBody::Broadcast(resp.into_reply(outbound_msg_id))
            }
            BroadcastMsgRequestBody::Read(resp) => {
                BroadcastMsgResponseBody::Read(resp.into_reply(outbound_msg_id))
            }
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BroadcastMsgResponseBody {
    Topology(TopologyResponseMsg),
    Broadcast(BroadcastResponseMsg),
    Read(ReadResponseMsg),
}

impl BroadcastMsgResponseBody {
    pub fn set_value(&mut self, value: Option<Vec<Value>>) {
        match self {
            BroadcastMsgResponseBody::Topology(_) => (),
            BroadcastMsgResponseBody::Broadcast(_) => (),
            BroadcastMsgResponseBody::Read(ref mut body) => {
                body.messages = value.unwrap_or_default()
            }
        }
    }
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
struct Topology(MessageType);

/// Topology Request inbound
#[derive(Deserialize, Debug)]
pub struct TopologyRequestMsg {
    msg_id: Option<u64>,
    pub topology: HashMap<String, Vec<String>>,
}

impl rpc::IntoReplyBody for TopologyRequestMsg {
    type Item = TopologyResponseMsg;
    fn into_reply(self, outbound_msg_id: u64) -> TopologyResponseMsg {
        TopologyResponseMsg {
            typ: TopologyOk(MsgType::TopologyOk),
            msg_id: outbound_msg_id,
            in_reply_to: self.msg_id,
        }
    }
}

/// Topology Request inbound
#[derive(Serialize, Debug)]
struct TopologyOk(MsgType);

#[derive(Serialize, Debug)]
pub struct TopologyResponseMsg {
    #[serde(rename = "type")]
    typ: TopologyOk,
    in_reply_to: Option<u64>,
    msg_id: u64,
}

impl rpc::Reply for TopologyResponseMsg {}

/// Broadcast Request inbound
#[derive(Deserialize, Debug)]
struct Broadcast(MessageType);

#[derive(Deserialize, Debug)]
pub struct BroadcastRequestMsg {
    msg_id: Option<u64>,
    pub message: Value,
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
    fn into_reply(self, outbound_msg_id: u64) -> BroadcastResponseMsg {
        BroadcastResponseMsg {
            typ: BroadcastOk(MsgType::BroadcastOk),
            msg_id: outbound_msg_id,
            in_reply_to: self.msg_id,
        }
    }
}

impl rpc::Reply for BroadcastResponseMsg {}

/// Read Request inbound
#[derive(Deserialize, Debug)]
struct Read(MessageType);

#[derive(Deserialize, Debug)]
pub struct ReadRequestMsg {
    msg_id: Option<u64>,
}
impl rpc::IntoReplyBody for ReadRequestMsg {
    type Item = ReadResponseMsg;
    fn into_reply(self, outbound_msg_id: u64) -> ReadResponseMsg {
        ReadResponseMsg {
            typ: ReadOk(MsgType::ReadOk),
            msg_id: outbound_msg_id,
            in_reply_to: self.msg_id,
            messages: vec![],
        }
    }
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
