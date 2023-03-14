/// Broadcast node: see maelstrom broadcast docs
/// https://github.com/jepsen-io/maelstrom/blob/main/doc/03-broadcast/01-broadcast.md
///
///
use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use rand::{seq::IteratorRandom, thread_rng};
use tokio::sync::mpsc::Receiver;

use crate::errors;
use crate::node::Node;
use crate::rpc::{self, broadcast};
use crate::workload::Command;

/// In lieu of *sending* messages: we print them to screen
fn send_messages(messages: Vec<rpc::broadcast::BroadcastMsgIn>) {
    messages
        .iter()
        .map(|m| {
            let serialized = serde_json::to_string(&m).map_err(|_e| {
                eprintln!("Serialization failed: {}", _e);
            });
            // simulate "send" by printing to screen
            println!("{}", serialized.unwrap_or_default());
        })
        .reduce(|_a, _e| ());
}

pub struct Broadcast {
    node_id: String,
    topology: HashMap<String, Vec<String>>,
    all_nodes: Vec<String>,
    // we will periodically broadcast messages to all other nodes
    notify_vals: Vec<u64>,
    // maelstrom broadcast values are unique and results do not need to be ordered
    values: HashSet<u64>,
    last_msg_id: u64,
    rx: Receiver<Command>,
}

impl Broadcast {
    async fn handle_tick(&mut self) -> Result<(), errors::ErrorMsg> {
        let msgs = self.build_broadcast_messages();
        if !msgs.is_empty() {
            send_messages(msgs);
        }
        // race condition here?
        self.notify_vals = vec![];
        Ok(())
    }

    async fn handle_broadcast(
        &mut self,
        msg: &broadcast::BroadcastRequestMsg,
    ) -> Option<HashSet<u64>> {
        if !self.values.contains(&msg.message) {
            self.values.insert(msg.message);
        }
        if !self.notify_vals.contains(&msg.message) {
            self.notify_vals.push(msg.message);
        }
        None
    }

    async fn handle_broadcast_ok(
        &mut self,
        _msg: &broadcast::BroadcastReceivedOkMsg,
    ) -> Option<HashSet<u64>> {
        None
    }

    async fn handle_read(&mut self, _msg: &broadcast::ReadRequestMsg) -> Option<HashSet<u64>> {
        Some(self.values.clone())
    }

    async fn handle_topology(
        &mut self,
        msg: &broadcast::TopologyRequestMsg,
    ) -> Option<HashSet<u64>> {
        self.topology = msg.topology.clone();
        self.all_nodes = vec![];
        for node_list in msg.topology.values() {
            for node in node_list.into_iter().filter(|nid| **nid != self.node_id) {
                self.all_nodes.push(node.clone());
            }
        }
        None
    }

    /// Using a topology, we build a Vector of outbound messages
    /// All messages have same content for now.
    fn build_broadcast_messages(&mut self) -> Vec<rpc::broadcast::BroadcastMsgIn> {
        let mut rng = thread_rng();
        let mut msgs = vec![];
        let sample_count = self.all_nodes.len() / 2 + 1;
        let sample = self.all_nodes.iter().choose_multiple(&mut rng, sample_count);
        for dest in sample {
            for msg in self.notify_vals.iter().map(|notify_val| {
                rpc::broadcast::BroadcastMsgIn::new_broadcast(
                    self.node_id.clone(),
                    dest.clone(),
                    *notify_val,
                    None,
                )
            }) {
                msgs.push(msg);
            }
        }

        msgs
    }
}

#[async_trait]
impl Node for Broadcast {
    fn new(starting_msg_id: u64, rx: Receiver<Command>) -> Self {
        Self {
            rx,
            last_msg_id: starting_msg_id,
            notify_vals: vec![],
            node_id: "n0".to_string(),
            topology: HashMap::new(),
            all_nodes: vec![],
            values: HashSet::new(),
        }
    }

    async fn handle(&mut self, msg: String) -> Result<(), errors::ErrorMsg> {
        self.last_msg_id += 1;
        let msg_in = serde_json::from_str::<broadcast::BroadcastMsgIn>(msg.as_str())
            .map_err(errors::ErrorMsg::json_parse_error)?;
        let values = match &msg_in.body {
            broadcast::BroadcastMsgRequestBody::Topology(msg) => self.handle_topology(msg).await,
            broadcast::BroadcastMsgRequestBody::Broadcast(msg) => self.handle_broadcast(msg).await,
            broadcast::BroadcastMsgRequestBody::Read(msg) => self.handle_read(msg).await,
            broadcast::BroadcastMsgRequestBody::BroadcastOk(msg) => {
                self.handle_broadcast_ok(msg).await
            }
        };
        let result = msg_in
            .into_str_response(values, self.last_msg_id)
            .map_err(|_e| {
                eprintln!("{:?}", _e);
                _e
            })?;
        println!("{}", result);
        Ok(())
    }

    async fn on_init(&mut self, msg: rpc::InitMsgIn) -> Result<(), errors::ErrorMsg> {
        self.node_id = msg.body.node_id.clone();
        let msg_out = msg.into_response(self.last_msg_id);
        let result = serde_json::to_string(&msg_out).map_err(errors::ErrorMsg::json_dumps_error)?;
        println!("{}", result);
        Ok(())
    }

    async fn start(&mut self) -> Result<(), errors::ErrorMsg> {
        while let Some(cmd) = self.rx.recv().await {
            match cmd {
                Command::Init(init_msg) => self.on_init(init_msg).await?,
                Command::Msg(msg) => self.handle(msg).await?,
                Command::Tick => self.handle_tick().await?,
                Command::Toplogy(new_topology) => self.topology = new_topology,
                Command::Shutdown => self.stop().await?,
            }
        }
        Ok(())
    }
    async fn stop(&mut self) -> Result<(), errors::ErrorMsg> {
        Ok(())
    }
}
