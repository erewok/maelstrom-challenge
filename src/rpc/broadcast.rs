use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::errors;
use crate::rpc::{self, IntoReplyBody, MessageType};

/// Our Broadcast node will *send* and *receive* these,
/// so need to be able to serialize them too.
#[derive(Serialize, Deserialize, Debug)]
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
    pub fn new_broadcast(
        src: String,
        dest: String,
        value: u64,
        msg_id: Option<u64>,
    ) -> BroadcastMsgIn {
        let body = BroadcastMsgRequestBody::Broadcast(BroadcastRequestMsg::new(value, msg_id));
        BroadcastMsgIn { src, dest, body }
    }

    pub fn into_response(
        self,
        value: Option<HashSet<u64>>,
        outbound_msg_id: u64,
    ) -> Option<BroadcastMsgOut> {
        match self.body.into_reply(outbound_msg_id) {
            BroadcastMsgResponseBody::NoOp => None,
            mut body => {
                let mut _value = value;
                if _value.is_some() {
                    body.set_value(_value.take());
                }
                Some(BroadcastMsgOut {
                    src: self.dest,
                    dest: self.src,
                    body,
                })
            }
        }
    }

    pub fn into_str_response(
        self,
        value: Option<HashSet<u64>>,
        outbound_msg_id: u64,
    ) -> Result<String, errors::ErrorMsg> {
        match self.into_response(value, outbound_msg_id) {
            None => Ok("".to_string()),
            Some(msg_out) => {
                serde_json::to_string(&msg_out).map_err(errors::ErrorMsg::json_dumps_error)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BroadcastMsgRequestBody {
    Topology(TopologyRequestMsg),
    Broadcast(BroadcastRequestMsg),
    BroadcastOk(BroadcastReceivedOkMsg),
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
            BroadcastMsgRequestBody::BroadcastOk(_) => BroadcastMsgResponseBody::NoOp,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastReceivedOkMsg {
    pub in_reply_to: Option<u64>,
    msg_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum BroadcastMsgResponseBody {
    Topology(TopologyResponseMsg),
    Broadcast(BroadcastResponseMsg),
    Read(ReadResponseMsg),
    NoOp,
}

impl BroadcastMsgResponseBody {
    pub fn set_value(&mut self, value: Option<HashSet<u64>>) {
        match self {
            BroadcastMsgResponseBody::Topology(_) => (),
            BroadcastMsgResponseBody::Broadcast(_) => (),
            BroadcastMsgResponseBody::Read(ref mut body) => {
                body.messages = value.unwrap_or_default()
            }
            BroadcastMsgResponseBody::NoOp => (),
        }
    }
}

impl rpc::Reply for BroadcastMsgResponseBody {}

/// Outbound message type strings
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
struct TopologyOk(MsgType);

#[derive(Serialize, Deserialize, Debug)]
pub struct TopologyResponseMsg {
    #[serde(rename = "type")]
    typ: TopologyOk,
    in_reply_to: Option<u64>,
    msg_id: u64,
}

impl rpc::Reply for TopologyResponseMsg {}

/// Broadcast Request inbound
#[derive(Serialize, Deserialize, Debug)]
struct Broadcast(MessageType);

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastRequestMsg {
    msg_id: Option<u64>,
    pub message: u64,
}

impl BroadcastRequestMsg {
    pub fn new(message: u64, msg_id: Option<u64>) -> Self {
        Self {
            msg_id,
            message,
        }
    }
}

/// Broadcast Response
#[derive(Serialize, Deserialize, Debug)]
struct BroadcastOk(MsgType);

#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
struct Read(MessageType);

#[derive(Serialize, Deserialize, Debug)]
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
            messages: HashSet::new(),
        }
    }
}

/// Read Response
#[derive(Serialize, Deserialize, Debug)]
struct ReadOk(MsgType);

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadResponseMsg {
    #[serde(rename = "type")]
    typ: ReadOk,
    in_reply_to: Option<u64>,
    msg_id: u64,
    messages: HashSet<u64>,
}

impl rpc::Reply for ReadResponseMsg {}
