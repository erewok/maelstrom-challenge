use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::errors;
use crate::rpc::{self, echo};
use crate::workload;

pub struct Node {
    pub node_id: String,
    pub node_ids: Vec<String>,
    pub next_msg_id: u64,
    workload: workload::Workload,
}

impl Node {
    pub fn new(workload: workload::Workload) -> Self {
        Self {
            node_id: "".to_string(),
            node_ids: vec![],
            next_msg_id: 1,
            workload,
        }
    }
    pub fn on_init(&mut self, msg: rpc::InitMsgIn) -> Result<String, errors::ErrorMsg> {
        let msg_out = msg.into_response(self.next_msg_id);
        self.node_id = msg.body.node_id;
        self.node_ids = msg.body.node_ids;
        serde_json::to_string(&msg_out).map_err(|_e| errors::ErrorMsg::json_dumps_error())
    }

    pub fn handle(&mut self, msg: &str) -> Result<String, errors::ErrorMsg> {
        self.next_msg_id += 1;
        match self.workload {
            workload::Workload::Echo => {
                echo::EchoMsgIn::parse_msg_to_str_response(msg, self.next_msg_id)
            }
            workload::Workload::UniqueIDs => todo!(),
            workload::Workload::Broadcast => todo!(),
            workload::Workload::GCounter => todo!(),
            workload::Workload::GSet => todo!(),
            workload::Workload::Kafka => todo!(),
            workload::Workload::LinKV => todo!(),
            workload::Workload::PNCounter => todo!(),
            workload::Workload::TxnListAppend => todo!(),
            workload::Workload::TxnRwRegister => todo!(),
        }
    }

    pub async fn run(workload: workload::Workload) -> Result<(), errors::ErrorMsg> {
        let stdin = io::stdin();
        let mut lines = BufReader::new(stdin).lines();
        let mut node: Self = Node::new(workload);
        let mut initialized = false;

        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|e| errors::ErrorMsg::crash_error())?
        {
            let result: String;

            if !initialized {
                let init_first = serde_json::from_str::<rpc::InitMsgIn>(&line).map_err(|_e| {
                    eprintln!("{:?}", _e);
                    errors::ErrorMsg::json_parse_error()
                })?;
                result = node.on_init(init_first)?;
                initialized = true;
            } else {
                result = node.handle(&line).map_err(|_e| {
                    eprintln!("{:?}", _e);
                    _e
                })?;
            }
            println!("{}", result);
        }

        Ok(())
    }
}
