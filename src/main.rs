use clap::Parser;

use maelstrom_challenge::node;
use maelstrom_challenge::workload;

/// Run a Maelstrom Challenge from Fly.io
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the workload (challenge) to run
    #[arg(short, long)]
    #[arg(value_enum)]
    workload: workload::Workload,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    node::run(args.workload).await.unwrap_err();
}
