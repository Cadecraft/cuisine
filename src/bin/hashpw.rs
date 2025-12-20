fn escape_dollars(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if c == '$' {
            res.push('\\');
        }
        res.push(c);
    }
    res
}

/// Utility to hash a password
fn main() {
    let mut args = std::env::args();
    args.next();
    let password = args.next().expect("Expected one argument (the password)");

    match cuisine::auth::hash_password(&password) {
        Some(hashed) => {
            println!("{}", hashed);
            println!("With dollar signs escaped:");
            println!("{}", escape_dollars(&hashed));
        }
        None => {
            println!("Failed to hash");
        }
    }
}
