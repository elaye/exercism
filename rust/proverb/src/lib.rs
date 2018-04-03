fn sentence(first_word: &str, second_word: &str) -> String {
    format!("For want of a {} the {} was lost.", first_word, second_word)
}

pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() {
        return String::new();
    }

    let words = list.iter().zip(list.iter().skip(1));

    let mut proverb: Vec<String> = words.map(|(first, second)| sentence(first, second)).collect();

    let last_sentence = format!("And all for the want of a {}.", list[0]);

    proverb.push(last_sentence);

    proverb.join("\n")
}
