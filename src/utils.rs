use sha2::{Sha256, Digest};

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn proof_of_work(data: &str, difficulty: usize) -> (String, u64) {
    let mut nonce = 0;
    loop {
        let attempt = format!("{}{}", data, nonce);
        let hash_value = hash(&attempt);
        if hash_value.chars().take(difficulty).all(|c| c == '0') {
            return (hash_value, nonce);
        }
        nonce += 1;
    }
}
