use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::errors;
use crate::node::Node;
use crate::rpc::{self, echo};
use crate::workload::Command;

pub struct EchoNode {
    last_msg_id: u64,
    rx: Receiver<Command>,
}

#[async_trait]
impl Node for EchoNode {
    fn new(starting_msg_id: u64, rx: Receiver<Command>) -> Self {
        Self {
            last_msg_id: starting_msg_id,
            rx,
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
        let result = echo::EchoMsgIn::parse_msg_to_str_response(msg.as_str(), self.last_msg_id)
            .map_err(|_e| {
                eprintln!("{:?}", _e);
                _e
            })?;
        println!("{}", result);
        Ok(())
    }

    async fn on_init(&mut self, msg: rpc::InitMsgIn) -> Result<(), errors::ErrorMsg> {
        let msg_out = msg.into_response(self.last_msg_id);
        let result = serde_json::to_string(&msg_out).map_err(errors::ErrorMsg::json_dumps_error)?;
        println!("{}", result);
        Ok(())
    }
    async fn stop(&mut self) -> Result<(), errors::ErrorMsg> {
        Ok(())
    }
}
