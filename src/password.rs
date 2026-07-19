use rand::Rng;

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

pub struct PasswordOptions {
    pub length: u32,
    pub uppercase: bool,
    pub lowercase: bool,
    pub digits: bool,
    pub symbols: bool,
}

impl Default for PasswordOptions {
    fn default() -> Self {
        Self {
            length: 20,
            uppercase: true,
            lowercase: true,
            digits: true,
            symbols: true,
        }
    }
}

pub fn generate_password(opts: &PasswordOptions) -> String {
    let mut charset: Vec<u8> = Vec::new();
    let mut required: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();

    if opts.uppercase {
        charset.extend_from_slice(UPPERCASE);
        required.push(UPPERCASE[rng.gen_range(0..UPPERCASE.len())]);
    }
    if opts.lowercase {
        charset.extend_from_slice(LOWERCASE);
        required.push(LOWERCASE[rng.gen_range(0..LOWERCASE.len())]);
    }
    if opts.digits {
        charset.extend_from_slice(DIGITS);
        required.push(DIGITS[rng.gen_range(0..DIGITS.len())]);
    }
    if opts.symbols {
        charset.extend_from_slice(SYMBOLS);
        required.push(SYMBOLS[rng.gen_range(0..SYMBOLS.len())]);
    }

    if charset.is_empty() {
        charset.extend_from_slice(LOWERCASE);
    }

    let length = opts.length as usize;
    let mut result: Vec<u8> = Vec::with_capacity(length);

    for i in 0..length {
        if i < required.len() {
            result.push(required[i]);
        } else {
            result.push(charset[rng.gen_range(0..charset.len())]);
        }
    }

    for i in 0..result.len() {
        let j = rng.gen_range(0..result.len());
        result.swap(i, j);
    }

    String::from_utf8(result).unwrap_or_default()
}
