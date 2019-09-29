fn atbash(c: char) -> char {
    match c.is_numeric() {
        true => c,
        false => ('z' as u8 + 'a' as u8 - c as u8) as char,
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() && c.is_ascii())
        .map(atbash)
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|x| x.iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(atbash)
        .collect()
}
