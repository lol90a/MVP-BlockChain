use sha2::{Digest, Sha256};

pub fn proof_of_work(data: &str, difficulty: usize) -> (String, u64) {
    let mut nonce = 0;
    let difficulty_prefix = "0".repeat(difficulty);
    loop {
        let hash = calculate_hash(data, nonce);
        if hash.starts_with(&difficulty_prefix) {
            return (hash, nonce);
        }
        nonce += 1;
    }
}

fn calculate_hash(data: &str, nonce: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", data, nonce));
    format!("{:x}", hasher.finalize())
}
