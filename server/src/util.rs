use rand::{distributions::Alphanumeric, Rng}; // 0.8

pub fn new_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
