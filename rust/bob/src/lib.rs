// Responses
const EMPTY: &str = "Fine. Be that way!";
const SHOUTING: &str = "Whoa, chill out!";
const QUESTION: &str = "Sure.";
const RANDOM_MSG: &str = "Whatever.";
const SHOUTING_QUESTION: &str = "Calm down, I know what I'm doing!";

/// Check if all the alphabetic characters of a message are upercase
fn is_shouting(message: &str) -> bool {
  let letters: String = message
      .chars()
      .filter(|c| c.is_alphabetic())
      .collect();

  !letters.is_empty() && letters.chars().all(|c| c.is_uppercase())
}


pub fn reply(message: &str) -> &str {
  let message = message.trim();
  let is_shouting = is_shouting(message);
  let is_question = message.ends_with("?");

  if message.is_empty() {
      EMPTY
  } else if is_shouting && is_question {
      SHOUTING_QUESTION
  } else if is_shouting {
      SHOUTING
  } else if is_question {
      QUESTION
  } else {
      RANDOM_MSG
  }
}
