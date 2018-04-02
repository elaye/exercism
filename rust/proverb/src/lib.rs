fn sentence(first_word: &str, second_word: &str) -> String {
    format!("For want of a {} the {} was lost.", first_word, second_word)
}

pub fn build_proverb(list: Vec<&str>) -> String {
    if list.len() < 1 {
        return String::new();
    }

    let mut proverb = Vec::new();

    let words = list.iter().zip(list.iter().skip(1));

    words
        .map(|(first, second)| sentence(first, second))
        .for_each(|sentence| proverb.push(sentence));

    let nail_or_pin = if list.first().unwrap() == &"pin" { "pin" } else { "nail" };

    let last_sentence = format!("And all for the want of a {}.", nail_or_pin);

    proverb.push(last_sentence);

    proverb.join("\n")
}
