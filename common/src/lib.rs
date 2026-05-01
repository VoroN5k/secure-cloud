use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedFile {
    pub name_hash: String,
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
}
