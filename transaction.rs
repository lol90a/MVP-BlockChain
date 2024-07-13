use ring::{signature, rand};
use ring::signature::KeyPair;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn sign(&mut self, key_pair: &signature::Ed25519KeyPair) {
        let message = format!("{}{}{}", self.sender, self.receiver, self.amount);
        self.signature = key_pair.sign(message.as_bytes()).as_ref().to_vec();
    }

    pub fn verify(&self, public_key: &[u8]) -> bool {
        let peer_public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key);
        let message = format!("{}{}{}", self.sender, self.receiver, self.amount);
        peer_public_key.verify(message.as_bytes(), &self.signature).is_ok()
    }
}

pub fn generate_key_pair() -> (signature::Ed25519KeyPair, Vec<u8>) {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
    let public_key = key_pair.public_key().as_ref().to_vec();
    (key_pair, public_key)
}
