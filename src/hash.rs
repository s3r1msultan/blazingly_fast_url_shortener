
pub fn fnv1a_hash(url: &str) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    for byte in url.as_bytes() {
        hash ^= *byte as u32;
        hash = hash.wrapping_mul(0x01000193);
    }
    hash
}

pub fn base62_encode(mut num: u32) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let base = chars.len() as u32;
    let mut short_str = String::new();

    while num > 0 {
        let idx = (num % base) as usize;
        short_str.push(chars.chars().nth(idx).unwrap());
        num /= base;
    }

    if short_str.is_empty() {
        short_str.push('a');
    }

    short_str
}

pub fn generate_leet_variations(short_url: &str) -> Vec<String> {
    let replacements = vec![
        ("o", "0"), ("e", "3"), ("a", "4"), ("l", "1"), ("s", "5"),
        ("t", "7"), ("i", "1"), ("g", "9"), ("b", "8")
    ];

    let mut variations = Vec::new();
    for (orig, replace) in replacements.iter() {
        let variation = short_url.replacen(orig, replace, 1);
        if variation != short_url {
            variations.push(variation);
        }
    }

    variations
}