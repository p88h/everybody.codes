use aes::Aes256;
use cbc::Decryptor;
use cbc::cipher::block_padding::Pkcs7;
use cbc::cipher::{BlockModeDecrypt, KeyIvInit};
use serde_json::Value;
use std::path::Path;

const USER_AGENT: &str = "https://github.com/p88h/everybody.codes helper";

fn cookie() -> Result<String, String> {
    let raw = match std::env::var("EC_COOKIE") {
        Ok(value) => value,
        Err(_) => std::fs::read_to_string(".cookie")
            .map_err(|_| "No session cookie: set EC_COOKIE or create a .cookie file".to_string())?,
    };
    let guid = raw.trim().trim_start_matches("everybody-codes=");
    if guid.is_empty() || guid.chars().any(|c| c.is_whitespace() || c == ';') {
        return Err("Invalid session cookie".to_string());
    }
    Ok(format!("everybody-codes={guid}"))
}

fn request(cookie: &str, url: &str, field: &str) -> Result<String, String> {
    let mut response = ureq::get(url)
        .header("Cookie", cookie)
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|e| format!("HTTP request failed for URL \"{url}\": {e}"))?;
    let body = response.body_mut().read_to_string().map_err(|e| format!("Unable to read response from \"{url}\": {e}"))?;
    let json: Value = serde_json::from_str(&body).map_err(|e| format!("Invalid JSON from \"{url}\": {e}"))?;
    match json.get(field) {
        Some(Value::String(s)) => Ok(s.clone()),
        Some(Value::Null) | None => Err(format!("Field \"{field}\" missing from JSON response")),
        Some(other) => Ok(other.to_string()),
    }
}

fn decode_hex(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.trim();
    if hex.len() % 2 != 0 || !hex.is_ascii() {
        return Err("Invalid hex encoded data".to_string());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| format!("Invalid hex encoded data: {e}")))
        .collect()
}

/// Downloads, decrypts and returns the notes for the given event, quest and part.
pub fn download_notes(event: i32, quest: i32, part: i32) -> Result<String, String> {
    let cookie = cookie()?;

    let seed = request(&cookie, "https://api.everybody.codes/user/me", "seed")?;
    if seed == "0" {
        return Err("Invalid seed parameter. Check that session cookie is valid".to_string());
    }

    let encrypted = request(&cookie, &format!("https://everybody.codes/assets/{event}/{quest}/input/{seed}.json"), &part.to_string())
        .map_err(|e| format!("Unable to read encrypted data ({e}). Check that event, quest and part are correct"))?;
    let key = request(&cookie, &format!("https://api.everybody.codes/event/{event}/quest/{quest}"), &format!("key{part}"))
        .map_err(|e| format!("Unable to read decryption key ({e}). Check that previous parts are solved"))?;

    // The key is used as raw bytes, with the first 16 bytes doubling as the IV.
    let key = key.as_bytes();
    if key.len() != 32 {
        return Err(format!("Unexpected decryption key length {}", key.len()));
    }
    let data = decode_hex(&encrypted)?;
    let decrypted = Decryptor::<Aes256>::new_from_slices(key, &key[..16])
        .map_err(|e| format!("Invalid key: {e}"))?
        .decrypt_padded_vec::<Pkcs7>(&data)
        .map_err(|e| format!("Unable to decrypt notes: {e}"))?;
    String::from_utf8(decrypted).map_err(|e| format!("Decrypted notes are not valid UTF-8: {e}"))
}

/// Downloads notes and saves them to `path`, creating parent directories if needed.
pub fn download_to(event: i32, quest: i32, part: i32, path: &Path) -> Result<String, String> {
    let notes = download_notes(event, quest, part)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Unable to create \"{}\": {e}", parent.display()))?;
    }
    std::fs::write(path, &notes).map_err(|e| format!("Unable to write \"{}\": {e}", path.display()))?;
    Ok(notes)
}
