use std::collections::HashSet;

pub fn check(s: &str) -> bool {
    let chars = s.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().to_string())
        .collect::<Vec<String>>();

    chars.iter().collect::<HashSet<&String>>().len() == chars.len()
}
