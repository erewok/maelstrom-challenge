use async_trait::async_trait;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::algorithms;
use crate::errors;
use crate::rpc;
use crate::workload;


#[async_trait]
pub trait NodeHandler {
    async fn handle(&mut self, msg: &str, next_msg_id: u64) -> Result<String, errors::ErrorMsg>;
    async fn on_init(&mut self, msg: &rpc::InitMsgIn) -> Result<(), errors::ErrorMsg>;
}


pub struct Node {
    pub node_id: String,
    pub node_ids: Vec<String>,
    pub next_msg_id: u64,
    handler: Box<dyn NodeHandler>
}

impl Node {
    pub fn new(handler: Box<dyn NodeHandler>) -> Self {
        Self {
            node_id: "".to_string(),
            node_ids: vec![],
            next_msg_id: 1,
            handler,
        }
    }
    pub async fn on_init(&mut self, msg: rpc::InitMsgIn) -> Result<String, errors::ErrorMsg> {
        self.handler.on_init(&msg).await?;
        let msg_out = msg.into_response(self.next_msg_id);
        self.node_id = msg.body.node_id;
        self.node_ids = msg.body.node_ids;
        serde_json::to_string(&msg_out).map_err(|_e| errors::ErrorMsg::json_dumps_error())
    }

    pub async fn handle(&mut self, msg: &str) -> Result<String, errors::ErrorMsg> {
        self.next_msg_id += 1;
        self.handler.handle(msg, self.next_msg_id).await
    }

    pub async fn run(workload: workload::Workload) -> Result<(), errors::ErrorMsg> {
        let stdin = io::stdin();
        let mut lines = BufReader::new(stdin).lines();

        let mut node  = match workload {
            workload::Workload::Echo => Node::new(Box::new(algorithms::echo::EchoNode {}) as Box<dyn NodeHandler>),
            workload::Workload::UniqueIds =>  Node::new(Box::new(algorithms::unique_ids::UniqueIdGenerator::new()) as Box<dyn NodeHandler>),
            workload::Workload::Broadcast => todo!(),
            workload::Workload::GCounter => todo!(),
            workload::Workload::GSet => todo!(),
            workload::Workload::Kafka => todo!(),
            workload::Workload::LinKV => todo!(),
            workload::Workload::PNCounter => todo!(),
            workload::Workload::TxnListAppend => todo!(),
            workload::Workload::TxnRwRegister => todo!(),
        };

        let mut initialized = false;

        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|_e| errors::ErrorMsg::crash_error())?
        {
            let result: String;

            if !initialized {
                let init_first = serde_json::from_str::<rpc::InitMsgIn>(&line).map_err(|_e| {
                    eprintln!("{:?}", _e);
                    errors::ErrorMsg::json_parse_error()
                })?;
                result = node.on_init(init_first).await?;
                initialized = true;
            } else {
                result = node.handle(&line).await.map_err(|_e| {
                    eprintln!("{:?}", _e);
                    _e
                })?;
            }
            println!("{}", result);
        }

        Ok(())
    }
}
