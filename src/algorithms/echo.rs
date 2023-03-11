use async_trait::async_trait;

use crate::errors;
use crate::node::NodeHandler;
use crate::rpc::{self, echo};

pub struct EchoNode {}

#[async_trait]
impl NodeHandler for EchoNode {

    async fn handle(&mut self, msg: &str, next_msg_id: u64) -> Result<String, errors::ErrorMsg> {
        echo::EchoMsgIn::parse_msg_to_str_response(msg, next_msg_id)
    }
    async fn on_init(&mut self, _msg: &rpc::InitMsgIn) -> Result<(), errors::ErrorMsg> {
        Ok(())
    }
}