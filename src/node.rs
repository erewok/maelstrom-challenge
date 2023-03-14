use async_trait::async_trait;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::{self, Duration};

use crate::algorithms;
use crate::errors;
use crate::rpc;
use crate::workload;

/// A Node receives workload::Command(s) from the main thread and acts upon them.
/// It must be able to be launched from an init message, to handle a single message, and to shutdown
#[async_trait]
pub trait Node {
    // the following *will* be called by the runner
    fn new(starting_msg_id: u64, rx: Receiver<workload::Command>) -> Self
    where
        Self: Sized;
    async fn start(&mut self) -> Result<(), errors::ErrorMsg>;
    async fn stop(&mut self) -> Result<(), errors::ErrorMsg>;
    // Nodes must be able to handle a regular message but this won't be called directly
    async fn handle(&mut self, msg: String) -> Result<(), errors::ErrorMsg>;
    // Nodes must be able to handle an init message but this won't be called directly
    // Init behavior is controlled by main thread via Command::Init sent down the channel
    async fn on_init(&mut self, msg: rpc::InitMsgIn) -> Result<(), errors::ErrorMsg>;
}

async fn run_clock(tx: Sender<workload::Command>) {
    let mut interval = time::interval(Duration::from_millis(200));
    loop {
        interval.tick().await;
        tx.send(workload::Command::Tick).await.unwrap();
    }
}

pub async fn run(workload: workload::Workload) -> Result<(), errors::ErrorMsg> {
    let (tx, rx) = mpsc::channel(1000);

    let mut node = match workload {
        workload::Workload::Echo => {
            Box::new(algorithms::echo::EchoNode::new(1, rx)) as Box<dyn Node + Send>
        }
        workload::Workload::UniqueIds => {
            Box::new(algorithms::unique_ids::UniqueIdGenerator::new(1, rx)) as Box<dyn Node + Send>
        }
        workload::Workload::Broadcast => {
            Box::new(algorithms::broadcast::Broadcast::new(1, rx)) as Box<dyn Node + Send>
        }
        workload::Workload::GCounter => todo!(),
        workload::Workload::GSet => todo!(),
        workload::Workload::Kafka => todo!(),
        workload::Workload::LinKV => todo!(),
        workload::Workload::PNCounter => todo!(),
        workload::Workload::TxnListAppend => todo!(),
        workload::Workload::TxnRwRegister => todo!(),
    };

    let mut initialized = false;
    // Set up stdin listener loop
    let stdin = io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    // Launch our node
    let _handle = tokio::spawn(async move { node.start().await });

    // Launch our clock
    let _tx = tx.clone();
    tokio::spawn(async move { run_clock(_tx).await });
    // loop on stdin and send all messages down the channel
    while let Some(line) = lines
        .next_line()
        .await
        .map_err(errors::ErrorMsg::crash_error)?
    {
        if !initialized {
            let init_first = serde_json::from_str::<rpc::InitMsgIn>(&line)
                .map_err(errors::ErrorMsg::json_parse_error)?;
            tx.send(workload::Command::Init(init_first))
                .await
                .map_err(errors::ErrorMsg::crash_error)?;
            initialized = true;
        } else {
            tx.send(workload::Command::Msg(line))
                .await
                .map_err(errors::ErrorMsg::crash_error)?;
        }
    }
    tx.send(workload::Command::Shutdown)
        .await
        .map_err(errors::ErrorMsg::crash_error)?;
    Ok(())
}
