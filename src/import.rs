use crate::totp::{TokenType, TokenEntry};

pub fn parse_otpauth_uri(uri: &str) -> Result<TokenEntry, String> {
    let uri = uri.trim();
    if !uri.starts_with("otpauth://") {
        return Err("Invalid OTP URI: must start with otpauth://".to_string());
    }

    let without_scheme = &uri["otpauth://".len()..];
    let (token_type_str, rest) = without_scheme
        .split_once('/')
        .ok_or("Invalid OTP URI: missing type separator")?;

    let token_type = match token_type_str.to_lowercase().as_str() {
        "totp" => TokenType::Totp,
        "hotp" => {
            let counter = parse_query_param(rest, "counter")
                .and_then(|c| c.parse::<u64>().ok())
                .unwrap_or(0);
            TokenType::Hotp { counter }
        }
        "steam" => TokenType::Steam,
        _ => return Err(format!("Unknown token type: {}", token_type_str)),
    };

    let (label, query) = rest
        .split_once('?')
        .ok_or("Invalid OTP URI: missing query string")?;

    let (issuer, name) = if let Some((iss, n)) = label.split_once(':') {
        (iss.to_string(), n.to_string())
    } else {
        (parse_query_param(query, "issuer").unwrap_or_default(), label.to_string())
    };

    let secret = parse_query_param(query, "secret").ok_or("Missing secret parameter")?;

    let digits = parse_query_param(query, "digits")
        .and_then(|d| d.parse().ok())
        .unwrap_or(if matches!(token_type, TokenType::Steam) { 5 } else { 6 });

    let period = parse_query_param(query, "period")
        .and_then(|p| p.parse().ok())
        .unwrap_or(if matches!(token_type, TokenType::Steam) { 30 } else { 30 });

    Ok(TokenEntry {
        name: name.replace('+', " "),
        issuer: issuer.replace('+', " "),
        secret,
        token_type,
        digits,
        period,
    })
}

fn parse_query_param(query: &str, param: &str) -> Option<String> {
    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == param {
                return Some(url_decode(value));
            }
        }
    }
    None
}

fn url_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    result
}

#[derive(serde::Deserialize)]
struct AegisEntry {
    #[serde(rename = "type")]
    entry_type: String,
    name: String,
    issuer: String,
    info: AegisInfo,
}

#[derive(serde::Deserialize)]
struct AegisInfo {
    secret: String,
    digits: Option<u32>,
    period: Option<u64>,
    algo: Option<String>,
    counter: Option<u64>,
}

#[derive(serde::Deserialize)]
struct AegisDb {
    entries: Vec<AegisEntry>,
}

#[derive(serde::Deserialize)]
struct AegisVault {
    db: AegisDb,
}

pub fn parse_aegis_json(json: &str) -> Result<Vec<TokenEntry>, String> {
    let vault: AegisVault = serde_json::from_str(json).map_err(|e| format!("Aegis parse error: {}", e))?;
    Ok(vault
        .db
        .entries
        .into_iter()
        .map(|e| TokenEntry {
            name: e.name,
            issuer: e.issuer,
            secret: e.info.secret,
            token_type: match e.entry_type.as_str() {
                "steam" => TokenType::Steam,
                "hotp" => TokenType::Hotp { counter: e.info.counter.unwrap_or(0) },
                _ => TokenType::Totp,
            },
            digits: e.info.digits.unwrap_or(6),
            period: e.info.period.unwrap_or(30),
        })
        .collect())
}

#[derive(serde::Deserialize)]
struct AndotpEntry {
    secret: String,
    issuer: String,
    label: String,
    digits: Option<u32>,
    #[serde(rename = "type")]
    entry_type: Option<String>,
    period: Option<u64>,
}

pub fn parse_andotp_json(json: &str) -> Result<Vec<TokenEntry>, String> {
    let entries: Vec<AndotpEntry> =
        serde_json::from_str(json).map_err(|e| format!("andOTP parse error: {}", e))?;
    Ok(entries
        .into_iter()
        .map(|e| TokenEntry {
            name: e.label,
            issuer: e.issuer,
            secret: e.secret,
            token_type: match e.entry_type.as_deref() {
                Some("STEAM") => TokenType::Steam,
                Some("HOTP") => TokenType::Hotp { counter: 0 },
                _ => TokenType::Totp,
            },
            digits: e.digits.unwrap_or(6),
            period: e.period.unwrap_or(30),
        })
        .collect())
}

pub fn import_tokens(text: &str) -> Vec<TokenEntry> {
    let text = text.trim();

    if let Ok(tokens) = parse_otpauth_uri(text) {
        return vec![tokens];
    }

    if let Ok(tokens) = parse_aegis_json(text) {
        return tokens;
    }

    if let Ok(tokens) = parse_andotp_json(text) {
        return tokens;
    }

    let mut results = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("otpauth://") {
            if let Ok(token) = parse_otpauth_uri(line) {
                results.push(token);
            }
        }
    }
    results
}
