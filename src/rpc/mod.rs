pub mod broadcast;
pub mod echo;
pub mod gcounter;
pub mod gset;
pub mod unique_ids;

use serde::{Deserialize, Serialize};

use crate::errors;

/// These strings alone don't give much information
/// because they're often repeated for different workloads
/// see: https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md
#[derive(Serialize, Clone, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Init,
    Broadcast,
    BroadcastOk,
    Topology,
    Read,
    Echo,
    Add,
    Send,
    Poll,
    CommitOffsets,
    ListCommittedOffsets,
    Write,
    Cas,
    Txn,
    Generate,
}

/// Traits for turning requests into responses
pub trait Reply {}
pub trait IntoReplyBody {
    type Item: Reply;
    fn into_reply(self, outbound_msg_id: u64) -> Self::Item;
}

///

#[derive(Clone, Debug, Deserialize)]
pub struct InitMsgIn {
    pub src: String,
    pub dest: String,
    pub body: InitRequestMsg,
}

impl InitMsgIn {
    pub fn into_response(self, outbound_msg_id: u64) -> InitMsgOut {
        InitMsgOut {
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
            .map_err(errors::ErrorMsg::json_parse_error)?;
        serde_json::to_string(&msg_out).map_err(errors::ErrorMsg::json_dumps_error)
    }
}

#[derive(Serialize, Debug)]
pub struct InitMsgOut {
    pub src: String,
    pub dest: String,
    pub body: InitResponseMsg,
}

#[derive(Clone, Deserialize, Debug)]
struct Init(MessageType);

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
enum InitMessageResp {
    InitOk,
}

#[derive(Serialize, Debug)]
struct InitOk(InitMessageResp);

#[derive(Clone, Deserialize, Debug)]
pub struct InitRequestMsg {
    #[serde(rename = "type")]
    _typ: Init,
    pub msg_id: u64,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

impl InitRequestMsg {
    pub fn new(msg_id: u64, node_id: String, node_ids: Vec<String>) -> Self {
        InitRequestMsg {
            _typ: Init(MessageType::Init),
            msg_id,
            node_id,
            node_ids,
        }
    }
}
impl IntoReplyBody for InitRequestMsg {
    type Item = InitResponseMsg;
    fn into_reply(self, _: u64) -> InitResponseMsg {
        InitResponseMsg {
            typ: InitOk(InitMessageResp::InitOk),
            in_reply_to: Some(self.msg_id),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct InitResponseMsg {
    #[serde(rename = "type")]
    typ: InitOk,
    pub in_reply_to: Option<u64>,
}

impl Reply for InitResponseMsg {}
