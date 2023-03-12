/// https://github.com/jepsen-io/maelstrom/blob/main/doc/03-broadcast/01-broadcast.md
///
///
use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;

use crate::errors;
use crate::node::NodeHandler;
use crate::rpc::{self, broadcast};

pub struct Broadcast {
    pub node_id: String,
    topology: HashMap<String, Vec<String>>,
    values: Vec<Value>,
}

impl Default for Broadcast {
    fn default() -> Self {
        Self::new()
    }
}

impl Broadcast {
    pub fn new() -> Self {
        Self {
            node_id: "n0".to_string(),
            topology: HashMap::new(),
            values: vec![],
        }
    }

    pub fn handle_broadcast(&mut self, msg: &broadcast::BroadcastRequestMsg) -> Option<Vec<Value>> {
        self.values.push(msg.message.clone());
        None
    }

    pub fn handle_read(&mut self, _msg: &broadcast::ReadRequestMsg) -> Option<Vec<Value>> {
        Some(self.values.clone())
    }

    pub fn handle_topology(&mut self, msg: &broadcast::TopologyRequestMsg) -> Option<Vec<Value>> {
        self.topology = msg.topology.clone();
        None
    }
}

#[async_trait]
impl NodeHandler for Broadcast {
    async fn handle(&mut self, msg: &str, next_msg_id: u64) -> Result<String, errors::ErrorMsg> {
        let msg_in = serde_json::from_str::<broadcast::BroadcastMsgIn>(msg).map_err(|_e| {
            eprintln!("{}", _e);
            errors::ErrorMsg::json_parse_error()
        })?;
        let values = match &msg_in.body {
            broadcast::BroadcastMsgRequestBody::Topology(msg) => self.handle_topology(msg),
            broadcast::BroadcastMsgRequestBody::Broadcast(msg) => self.handle_broadcast(msg),
            broadcast::BroadcastMsgRequestBody::Read(msg) => self.handle_read(msg),
        };
        msg_in.into_str_response(values, next_msg_id)
    }
    async fn on_init(&mut self, msg: &rpc::InitMsgIn) -> Result<(), errors::ErrorMsg> {
        self.node_id = msg.body.node_id.clone();
        Ok(())
    }
}
