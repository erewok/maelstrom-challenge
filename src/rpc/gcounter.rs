use serde::{Deserialize, Serialize};

use crate::errors;
use crate::rpc;

/// Our Broadcast node will *send* and *receive* these,
/// so need to be able to serialize them too.
#[derive(Serialize, Deserialize, Debug)]
pub struct GCounterMessage {
    pub src: String,
    pub dest: String,
    pub body: GCounterMessageBody,
}

impl GCounterMessage {
    pub fn new_read(
        src: String,
        dest: String,
    ) -> GCounterMessage {
        GCounterMessage { src, dest, body: GCounterMessageBody::Read }
    }

    pub fn into_response(
        self,
        value: u64,
    ) -> Option<GCounterMessage> {
        match self.body.into_reply(value) {
            GCounterMessageBody::NoOp => None,
            body => {
                Some(GCounterMessage {
                    src: self.dest,
                    dest: self.src,
                    body,
                })
            }
        }
    }

    pub fn into_str_response(
        self,
        value: u64,
    ) -> Result<String, errors::ErrorMsg> {
        match self.into_response(value) {
            None => Ok("".to_string()),
            Some(msg_out) => {
                serde_json::to_string(&msg_out).map_err(errors::ErrorMsg::json_dumps_error)
            }
        }
    }
}

impl GCounterMessageBody {
    fn into_reply(self, value: u64) -> Self {
        match self {
            GCounterMessageBody::Read => {
                GCounterMessageBody::ReadOk(ReadResponseMsg {value})
            },
            GCounterMessageBody::Add(_req) => GCounterMessageBody::AddOk,
            GCounterMessageBody::ReadOk(_req)  => GCounterMessageBody::NoOp,
            GCounterMessageBody::AddOk => GCounterMessageBody::NoOp,
            GCounterMessageBody::NoOp => GCounterMessageBody::NoOp,
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum GCounterMessageBody {
    Read,
    ReadOk(ReadResponseMsg),
    Add(AddRequestMsg),
    AddOk,
    NoOp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddRequestMsg {
    pub delta: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadResponseMsg {
    pub value: u64,
}

impl rpc::Reply for GCounterMessageBody {}
impl rpc::Reply for ReadResponseMsg {}