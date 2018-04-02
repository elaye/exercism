fn is_yelling(message: &str) -> bool {
    let all_uppercase = message.chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase());

    let at_least_one_letter = message.chars().any(|c| c.is_alphabetic());

    all_uppercase && at_least_one_letter
}

pub fn reply(message: &str) -> &str {
    let is_yelling = is_yelling(message);

    // Filter whitespace
    let chars: Vec<char> = message
        .chars()
        .rev()
        .skip_while(|c| c.is_whitespace())
        .collect();

    // Message is reversed
    match chars.first() {
        Some(ch) => match *ch {
            '?' => if is_yelling { "Calm down, I know what I'm doing!" } else { "Sure." },
            _ => if is_yelling { "Whoa, chill out!" } else { "Whatever." }
        },
        None => "Fine. Be that way!"
    }
}
