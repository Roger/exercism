use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut uniq = HashSet::new();

    candidate
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .all(|c| uniq.insert(c))
}
