use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use pbkdf2::pbkdf2_hmac;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

const PBKDF2_ITERATIONS: u32 = 600_000;
const SALT_LEN: usize = 32;
const NONCE_LEN: usize = 12;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultData {
    pub tokens: Vec<super::totp::TokenEntry>,
    pub passkeys: Vec<super::passkey::PasskeyEntry>,
    pub theme: super::theme::AppTheme,
}

impl Default for VaultData {
    fn default() -> Self {
        Self {
            tokens: Vec::new(),
            passkeys: Vec::new(),
            theme: super::theme::AppTheme::default(),
        }
    }
}

fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    key
}

pub fn encrypt(data: &[u8], password: &str) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let salt: [u8; SALT_LEN] = rng.gen();
    let nonce_bytes: [u8; NONCE_LEN] = rng.gen();

    let key_bytes = derive_key(password, &salt);
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).expect("Valid key length");
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, data).expect("Encryption failed");

    let mut result = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    result
}

pub fn decrypt(encrypted: &[u8], password: &str) -> Result<Vec<u8>, String> {
    if encrypted.len() < SALT_LEN + NONCE_LEN {
        return Err("Data too short".to_string());
    }

    let salt = &encrypted[..SALT_LEN];
    let nonce_bytes = &encrypted[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &encrypted[SALT_LEN + NONCE_LEN..];

    let key_bytes = derive_key(password, salt);
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).map_err(|e| format!("Key error: {}", e))?;
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext).map_err(|e| format!("Decryption failed: {}", e))
}

pub fn vault_path() -> std::path::PathBuf {
    if let Some(dir) = super::DATA_DIR.get() {
        dir.join("vault.dat")
    } else {
        std::path::PathBuf::from("vault.dat")
    }
}

pub fn save_vault(data: &VaultData, password: &str) -> Result<(), String> {
    let json = serde_json::to_vec(data).map_err(|e| e.to_string())?;
    let encrypted = encrypt(&json, password);
    std::fs::write(vault_path(), encrypted).map_err(|e| e.to_string())
}

pub fn load_vault(password: &str) -> Result<VaultData, String> {
    let encrypted = std::fs::read(vault_path()).map_err(|e| e.to_string())?;
    let decrypted = decrypt(&encrypted, password)?;
    serde_json::from_slice(&decrypted).map_err(|e| e.to_string())
}

pub fn vault_exists() -> bool {
    vault_path().exists()
}
