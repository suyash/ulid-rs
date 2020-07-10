use std::time::{SystemTime, UNIX_EPOCH};

use rand::random;
use ulid_rs::Ulid;

fn main() {
    // create one with the current timestamp and a random value
    let ulid = Ulid::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        random,
    );
    println!("{}", ulid.to_string());
}
