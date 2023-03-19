/// Broadcast node: see maelstrom broadcast docs
/// https://github.com/jepsen-io/maelstrom/blob/main/doc/03-broadcast/01-broadcast.md
///
///
use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::mpsc::Receiver;

use crate::errors;
use crate::kv;
use crate::node::Node;
use crate::rpc::{self, gcounter};
use crate::workload::Command;

/// In lieu of *sending* messages: we print them to screen
fn send_messages(messages: Vec<gcounter::GCounterMessage>) {
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

pub struct GCounter {
    kvstore: kv::KV,
    node_id: String,
    topology: HashMap<String, Vec<String>>,
    all_nodes: Vec<String>,
    notify_ticks: u64,
    last_msg_id: u64,
    // Count of all add instructions received
    // instruction_count: u64,
    // Current value from receiving add instructions
    internal_current: u64,
    // Current max from the whole cluster
    cluster_max: u64,
    rx: Receiver<Command>,
}

impl GCounter {
    async fn handle_tick(&mut self) -> Result<(), errors::ErrorMsg> {
        // In order to get over network partitions, we'd need to try this
        // more than once
        self.notify_ticks += 1;
        let msgs = self.build_read_all_messages();
        if !msgs.is_empty() {
            send_messages(msgs);
        }
        // race condition here?
        if self.notify_ticks > 40 {
            self.notify_ticks = 0;
        }
        Ok(())
    }

    async fn handle_add(
        &mut self,
        val: u64,
    ) -> Option<u64> {
        self.internal_current += val;
        self.kvstore.write(self.node_id.clone(), Value::Number(val.into()));
        None
    }

    async fn handle_add_ok(&mut self) -> Option<u64>{
        None
    }

    async fn handle_read(&mut self) -> Option<u64> {
        // self.kvstore.read(self.node_id.clone());
        if self.internal_current > self.cluster_max {
            Some(self.internal_current)
        } else {
            Some(self.cluster_max)
        }
    }

    async fn handle_read_ok(&mut self, val: u64) -> Option<u64> {
        if val > self.cluster_max {
            self.cluster_max = val;
        }
        None
    }

    /// Using a topology, we build a Vector of outbound messages
    /// We are asking for the values that all other nodes in the cluster have.
    fn build_read_all_messages(&mut self) -> Vec<gcounter::GCounterMessage> {
        let mut msgs: Vec<gcounter::GCounterMessage> = vec![];
        for dest in self.all_nodes.iter() {
            let msg = gcounter::GCounterMessage::new_read(
                self.node_id.clone(),
                    dest.clone(),
            );
            msgs.push(msg);
        } 
        msgs
    }
}

#[async_trait]
impl Node for GCounter {
    fn new(_starting_msg_id: u64, rx: Receiver<Command>) -> Self {
        Self {
            kvstore: kv::KV::new(kv::KvType::LinKV),
            rx,
            last_msg_id: 0,
            notify_ticks: 0,
            internal_current: 0,
            cluster_max: 0,
            node_id: "n0".to_string(),
            topology: HashMap::new(),
            all_nodes: vec![],
        }
    }

    async fn handle(&mut self, msg: String) -> Result<(), errors::ErrorMsg> {
        let msg_in = serde_json::from_str::<gcounter::GCounterMessage>(msg.as_str())
            .map_err(errors::ErrorMsg::json_parse_error)?;
        let value = match &msg_in.body {
            gcounter::GCounterMessageBody::Read => self.handle_read().await,
            gcounter::GCounterMessageBody::ReadOk(val) => self.handle_read_ok(val.value).await,
            gcounter::GCounterMessageBody::Add(val) => self.handle_add(val.delta).await,
            gcounter::GCounterMessageBody::AddOk => self.handle_add_ok().await,
            gcounter::GCounterMessageBody::NoOp =>  None,
        };
        let result = msg_in
            .into_str_response(value.unwrap_or(0))
            .map_err(|_e| {
                eprintln!("{:?}", _e);
                _e
            })?;
        if !result.is_empty() {
            println!("{}", result);
        }
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
