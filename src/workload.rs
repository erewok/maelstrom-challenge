#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Workload {
    Broadcast,
    Echo,
    GCounter, // growable eventually consistent counter
    GSet,     // growable eventually consistent set
    Kafka,
    LinKV,         // linearizable key-value store
    PNCounter,     // eventually consistent counter with inc and dec
    TxnListAppend, // transactional workload over a map of keys to lists of elements
    TxnRwRegister, // transactional workload over a map of keys to values.
    UniqueIds,     // simple workload for ID generation systems
}
