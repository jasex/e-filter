use openssl::sha;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Message {
    content: String,
    pub nounce: u128,
}
impl Message {
    pub fn new(content: String) -> Self {
        Message { content, nounce: 0 }
    }
    pub fn update(&mut self) {
        self.nounce += 1;
    }
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = sha::Sha512::new();
        hasher.update(serde_json::to_string(&self).unwrap().as_bytes());
        let hash = hasher.finish();
        hash.to_vec()
    }
}
