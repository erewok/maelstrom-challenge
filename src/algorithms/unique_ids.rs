use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;

use crate::errors;
use crate::node::NodeHandler;
use crate::rpc::{self, unique_ids};

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

/// This is inspired by twitter snowflake
pub struct UniqueIdGenerator {
    counter: u64,
    pub node_id: u64,
}

impl Default for UniqueIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl UniqueIdGenerator {
    pub fn new() -> Self {
        Self {
            counter: 0,
            node_id: 0,
        }
    }

    pub fn generate(&mut self) -> Result<u64, errors::ErrorMsg> {
        self.counter += 1;
        let epoch_millis = get_milliseconds();
        let epoch = epoch_millis
            .parse::<u64>()
            .map_err(|_e| errors::ErrorMsg::crash_error())?;
        let mut result = epoch << 23;
        result += self.node_id << 10;
        result += self.counter % 1024;
        Ok(result)
    }
}

#[async_trait]
impl NodeHandler for UniqueIdGenerator {
    async fn handle(&mut self, msg: &str, next_msg_id: u64) -> Result<String, errors::ErrorMsg> {
        let generated_id = self.generate()?;
        unique_ids::GenerateMsgIn::parse_msg_to_str_response(
            msg,
            generated_id.to_string(),
            next_msg_id,
        )
    }
    async fn on_init(&mut self, msg: &rpc::InitMsgIn) -> Result<(), errors::ErrorMsg> {
        self.node_id = node_string_id_to_u64(msg.body.node_id.as_str());
        Ok(())
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let mut generator = UniqueIdGenerator::new();
        assert!(generator.generate().expect("Could not generate") > 50463625189535744);
    }

    #[test]
    fn test_generate_orderable() {
        let mut generator = UniqueIdGenerator::new();
        let one = generator.generate().expect("Could not generate");
        let two = generator.generate().expect("Could not generate");
        let three = generator.generate().expect("Could not generate");

        assert!(one < two);
        assert!(two < three);
    }
}
