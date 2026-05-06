use std::env;
use std::fs;
use std::io;
use std::path::Path;

/// Valid keys contain only underscores and lowercase letters
fn valid_key(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn get_kv_dir() -> String {
    env::var("KV_PATH").unwrap()
}

/// Get the value given a key, or an empty string if it does not exist
pub fn get_value(key: &str) -> io::Result<String> {
    if !valid_key(key) {
        return Err(io::Error::other(
            "Key must only contain lowercase letters, numbers, and underscores",
        ));
    }

    let kv_dir = get_kv_dir();
    let file_path = Path::new(&kv_dir).join(key);

    let contents = fs::read_to_string(file_path).unwrap_or_default();
    Ok(contents)
}

/// Update the value of a key. If the value is an empty string, delete the key
pub fn put_value(key: &str, value: &str) -> io::Result<()> {
    if !valid_key(key) {
        return Err(io::Error::other(
            "Key must only contain lowercase letters, numbers, and underscores",
        ));
    }

    let kv_dir = get_kv_dir();
    let kv_dir_path = std::path::Path::new(&kv_dir);
    fs::create_dir_all(kv_dir_path)?;

    let file_path = kv_dir_path.join(key);

    if value.is_empty() {
        fs::remove_file(file_path)?;
    } else {
        fs::write(file_path, value)?;
    }

    Ok(())
}
