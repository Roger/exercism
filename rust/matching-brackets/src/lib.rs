const OPENS: [&char; 3] = [&'{', &'(', &'['];
const CLOSES: [&char; 3] = [&'}', &')', &']'];

/// find the balanced brackets
pub fn brackets_are_balanced(string: &str) -> bool {
    // keep track of open brackets
    let mut opens: Vec<char> = vec![];

    for chr in string.chars() {
        // Find close pair or None
        let pair = CLOSES.iter().position(|&&x| chr == x).map(|idx| OPENS[idx]);
        let is_open = OPENS.iter().any(|&&x| chr == x);

        match (pair, is_open) {
            // Store open brackets
            (_, true) => opens.push(chr),

            // Search for closes
            (Some(pair), false) => {
                if opens.pop() != Some(*pair) {
                    return false;
                }
            }
            // Ignore non bracket characters
            _ => (),
        }
    }
    opens.is_empty()
}
