fn is_shouting(message: &str) -> bool {
  let letters: String = message
      .chars()
      .filter(|c| c.is_alphabetic())
      .collect();

  letters.len() != 0 && letters.chars().all(|c| c.is_uppercase())
}


pub fn reply(message: &str) -> &str {
  let message = message.trim();
  let is_shouting = is_shouting(message);

  if message.is_empty() {
    "Fine. Be that way!"
  } else if is_shouting && message.ends_with("?") {
    "Calm down, I know what I'm doing!"
  } else if is_shouting {
    "Whoa, chill out!"
  } else if message.ends_with("?") {
    "Sure."
  } else {
    "Whatever."
  }
}
