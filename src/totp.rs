use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TokenType {
    Totp,
    Hotp { counter: u64 },
    Steam,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenEntry {
    pub name: String,
    pub issuer: String,
    pub secret: String,
    pub token_type: TokenType,
    pub digits: u32,
    pub period: u64,
}

impl TokenEntry {
    pub fn generate_code(&self, time: u64) -> String {
        let secret_bytes = decode_secret(&self.secret);
        match &self.token_type {
            TokenType::Totp => generate_totp(&secret_bytes, time, self.digits, self.period),
            TokenType::Hotp { counter } => generate_hotp(&secret_bytes, *counter, self.digits),
            TokenType::Steam => generate_steam_totp(&secret_bytes, time),
        }
    }

    pub fn time_remaining(&self, time: u64) -> u64 {
        match &self.token_type {
            TokenType::Totp | TokenType::Steam => self.period - (time % self.period),
            TokenType::Hotp { .. } => u64::MAX,
        }
    }

    pub fn progress(&self, time: u64) -> f32 {
        match &self.token_type {
            TokenType::Totp | TokenType::Steam => {
                (time % self.period) as f32 / self.period as f32
            }
            TokenType::Hotp { .. } => 0.0,
        }
    }
}

fn decode_secret(secret: &str) -> Vec<u8> {
    let cleaned: String = secret.chars().filter(|c| !c.is_whitespace()).collect();
    base32::decode(base32::Alphabet::Rfc4648 { padding: false }, &cleaned)
        .or_else(|| base32::decode(base32::Alphabet::Rfc4648 { padding: true }, &cleaned))
        .unwrap_or_else(|| cleaned.as_bytes().to_vec())
}

fn hotp_value(secret: &[u8], counter: u64) -> u32 {
    let counter_bytes = counter.to_be_bytes();
    let mut mac = HmacSha1::new_from_slice(secret).expect("HMAC key error");
    mac.update(&counter_bytes);
    let result = mac.finalize().into_bytes();

    let offset = (result[19] & 0x0f) as usize;
    ((result[offset] as u32 & 0x7f) << 24)
        | ((result[offset + 1] as u32 & 0xff) << 16)
        | ((result[offset + 2] as u32 & 0xff) << 8)
        | (result[offset + 3] as u32 & 0xff)
}

pub fn generate_hotp(secret: &[u8], counter: u64, digits: u32) -> String {
    let value = hotp_value(secret, counter);
    let otp = value % 10u32.pow(digits);
    format!("{:0width$}", otp, width = digits as usize)
}

pub fn generate_totp(secret: &[u8], time: u64, digits: u32, period: u64) -> String {
    let counter = time / period;
    generate_hotp(secret, counter, digits)
}

pub fn generate_steam_totp(secret: &[u8], time: u64) -> String {
    const STEAM_CHARS: &[u8] = b"23456789BCDFGHJKMNPQRTVWXY";
    let counter = time / 30;
    let value = hotp_value(secret, counter);

    let mut v = value;
    let mut code = String::with_capacity(5);
    for _ in 0..5 {
        code.push(STEAM_CHARS[(v % 26) as usize] as char);
        v /= 26;
    }
    code
}

pub fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
