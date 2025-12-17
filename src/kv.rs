use std::fs;
use std::io;
use std::path::Path;

const RELATIVE_KV_PATH: &str = "/kv";

/// Whether the string contains only underscores and lowercase letters
fn valid_key(s: &str) -> bool {
    s.len() > 0
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn get_kv_dir() -> String {
    format!("{}{}", ".", RELATIVE_KV_PATH)
}

/// Get the value given a key, or an empty string if it does not exist
pub fn get_value(key: &str) -> io::Result<String> {
    if !valid_key(key) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Key must only contain lowercase letters, numbers, and underscores",
        ));
    }

    let kv_dir = get_kv_dir();
    let file_path = Path::new(&kv_dir).join(key);

    let contents = fs::read_to_string(file_path).unwrap_or(String::new());
    Ok(contents)
}

/// Update the value of a key. If the value is an empty string, delete the key
pub fn put_value(key: &str, val: &str) -> io::Result<()> {
    if !valid_key(key) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Key must only contain lowercase letters, numbers, and underscores",
        ));
    }

    let kv_dir = get_kv_dir();
    let kv_dir_path = std::path::Path::new(&kv_dir);
    fs::create_dir_all(kv_dir_path)?;

    let file_path = kv_dir_path.join(key);

    fs::write(file_path, val)?;

    Ok(())
}
