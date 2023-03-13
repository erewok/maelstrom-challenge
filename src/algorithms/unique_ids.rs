use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::errors;
use crate::node::Node;
use crate::rpc::{self, unique_ids};
use crate::workload::Command;

/// This is inspired by twitter snowflake:
/// 64 bits in three chunks:
///   timestamp (first 23 bits)
///   node id (middle 10 bits)
///   (counter % 1024) remaining bits

/// We're going to use the integer identifier for this node as part of our unique id
fn node_string_id_to_u64(node_id: &str) -> u64 {
    // node_ids are strings like "n2". We want to turn this into an int.
    let node_id_int: String = node_id.chars().filter(|c| c.is_ascii_digit()).collect();
    if node_id_int.is_empty() {
        0
    } else {
        node_id_int.parse().unwrap_or(0)
    }
}

// Milliseconds since epoch are the leftmost bits in our unique id
pub fn get_milliseconds() -> String {
    // our epoch begins Sunday, January 1, 2023 1:01:01 AM
    let our_epoch_start: SystemTime = UNIX_EPOCH + Duration::from_secs(1672534861);
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(our_epoch_start)
        .expect("Time went backwards");
    let epoch_milli_string = since_the_epoch.as_millis().to_string();
    let (head, _) = epoch_milli_string.split_at(10);
    head.to_owned()
}

/// This is our Node implementation
pub struct UniqueIdGenerator {
    counter: u64,
    node_id: u64,
    last_msg_id: u64,
    rx: Receiver<Command>,
}

impl UniqueIdGenerator {
    pub fn generate(&mut self) -> Result<u64, errors::ErrorMsg> {
        self.counter += 1;
        let epoch_millis = get_milliseconds();
        let epoch = epoch_millis
            .parse::<u64>()
            .map_err(errors::ErrorMsg::crash_error)?;
        let mut result = epoch << 23;
        result += self.node_id << 10;
        result += self.counter % 1024;
        Ok(result)
    }
}

#[async_trait]
impl Node for UniqueIdGenerator {
    fn new(starting_msg_id: u64, rx: Receiver<Command>) -> Self {
        Self {
            rx,
            last_msg_id: starting_msg_id,
            counter: 1,
            node_id: 0,
        }
    }

    // the following *will* be called by the runner
    async fn start(&mut self) -> Result<(), errors::ErrorMsg> {
        while let Some(cmd) = self.rx.recv().await {
            match cmd {
                Command::Init(init_msg) => self.on_init(init_msg).await?,
                Command::Msg(msg) => self.handle(msg).await?,
                Command::Shutdown => self.stop().await?,
                _ => (),
            }
        }
        Ok(())
    }

    async fn handle(&mut self, msg: String) -> Result<(), errors::ErrorMsg> {
        self.last_msg_id += 1;
        let generated_id = self.generate()?;
        let result = unique_ids::GenerateMsgIn::parse_msg_to_str_response(
            msg.as_str(),
            generated_id.to_string(),
            self.last_msg_id,
        )
        .map_err(|_e| {
            eprintln!("{:?}", _e);
            _e
        })?;
        println!("{}", result);
        Ok(())
    }

    async fn on_init(&mut self, msg: rpc::InitMsgIn) -> Result<(), errors::ErrorMsg> {
        self.node_id = node_string_id_to_u64(msg.body.node_id.as_str());
        let msg_out = msg.into_response(self.last_msg_id);
        let result = serde_json::to_string(&msg_out).map_err(errors::ErrorMsg::json_dumps_error)?;
        println!("{}", result);
        Ok(())
    }
    async fn stop(&mut self) -> Result<(), errors::ErrorMsg> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_generate() {
    //     let mut generator = UniqueIdGenerator::new();
    //     assert!(generator.generate().expect("Could not generate") > 50463625189535744);
    // }

    // #[test]
    // fn test_generate_orderable() {
    //     let mut generator = UniqueIdGenerator::new();
    //     let one = generator.generate().expect("Could not generate");
    //     let two = generator.generate().expect("Could not generate");
    //     let three = generator.generate().expect("Could not generate");

    //     assert!(one < two);
    //     assert!(two < three);
    // }
}
