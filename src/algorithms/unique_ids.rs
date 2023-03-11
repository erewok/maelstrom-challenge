use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// This is inspired by twitter snowflake
///
/// Start our epoch on this date
/// id = 1387263000 <<(64-41)
///
/// Let’s say we’re sharding by user ID,
/// and there are 2000 logical shards;
/// if our user ID is 31341, then the shard ID is 31341 % 2000 -> 1341
/// We fill the next 13 bits with this value:
/// id |= 1341 <<(64-41-13)
///
/// Remaining bits are the counter
/// id |= (5001 % 1024)
///
///
///

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

pub struct UniqueIdGenerator {
    counter: u64,
    pub node_id: u64,
}

impl UniqueIdGenerator {
    pub fn new(node_id: u64) -> Self {
        Self { counter: 0, node_id}
    }

    pub fn generate(&mut self) -> u64 {
        self.counter += 1;
        let epoch_millis = get_milliseconds();
        let epoch = epoch_millis.parse::<u64>().expect("Failed to parse epoch time");
        let mut result = epoch << 23;
        result += self.node_id << 10;
        result += self.counter % 1024;
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let mut generator = UniqueIdGenerator::new(12);
        assert!(generator.generate() > 50463625189535744);
    }

}