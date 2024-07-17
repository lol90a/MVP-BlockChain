use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u128,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn sign(&mut self, key_pair: &Ed25519KeyPair) {
        let data = format!("{}{}{}", self.sender, self.receiver, self.amount);
        self.signature = key_pair.sign(data.as_bytes()).as_ref().to_vec();
    }

    pub fn verify(&self, public_key: &[u8]) -> bool {
        let peer_public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
        let data = format!("{}{}{}", self.sender, self.receiver, self.amount);
        peer_public_key
            .verify(data.as_bytes(), &self.signature)
            .is_ok()
    }
}

pub fn generate_key_pair() -> (Ed25519KeyPair, Vec<u8>) {
    let rng = ring::rand::SystemRandom::new();
    let key_pair = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = Ed25519KeyPair::from_pkcs8(key_pair.as_ref()).unwrap();
    let public_key = key_pair.public_key().as_ref().to_vec();
    (key_pair, public_key)
}
