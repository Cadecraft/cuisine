use std::fs;
use std::io;

const RELATIVE_KV_PATH: &str = "/kv";

/// Whether the string contains only underscores and lowercase letters
fn valid_key(s: &str) -> bool {
    s.len() > 0 && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Get the value given a key, or an empty string if it does not exist
pub fn get_value(key: &str) -> io::Result<String> {
    if !valid_key(key) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Key must only contain lowercase letters, numbers, and underscores",
        ));
    }

    let full_kv_dir_str = format!("{}{}", ".", RELATIVE_KV_PATH);
    let full_kv_dir = std::path::Path::new(&full_kv_dir_str);
    fs::create_dir_all(full_kv_dir)?;

    let file_path = full_kv_dir.join(key);

    Ok(format!("Hello world {}", file_path.to_str().unwrap_or("failed")))
}

/// Update the value of a key. If the value is an empty string, delete the key
pub fn put_value(key: &str, val: &str) -> io::Result<()> {
    if !valid_key(key) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Key must only contain lowercase letters, numbers, and underscores",
        ));
    }

    // TODO: key validation with error
    let full_kv_dir_str = format!("{}{}", ".", RELATIVE_KV_PATH);
    let full_kv_dir = std::path::Path::new(&full_kv_dir_str);
    fs::create_dir_all(full_kv_dir)?;

    Ok(())
}
