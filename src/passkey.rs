use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyEntry {
    pub rp_id: String,
    pub username: String,
    pub display_name: String,
    pub credential_id: String,
    pub user_handle: String,
    pub private_key: String,
    pub created_at: u64,
}

impl PasskeyEntry {
    pub fn new(rp_id: &str, username: &str, display_name: &str) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let cred_id: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let user_handle: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        let private_key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

        Self {
            rp_id: rp_id.to_string(),
            username: username.to_string(),
            display_name: display_name.to_string(),
            credential_id: hex::encode(&cred_id),
            user_handle: hex::encode(&user_handle),
            private_key: hex::encode(&private_key),
            created_at: super::totp::current_timestamp(),
        }
    }
}

mod hex {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";

    pub fn encode(data: &[u8]) -> String {
        let mut s = String::with_capacity(data.len() * 2);
        for &byte in data {
            s.push(HEX_CHARS[(byte >> 4) as usize] as char);
            s.push(HEX_CHARS[(byte & 0x0f) as usize] as char);
        }
        s
    }
}
