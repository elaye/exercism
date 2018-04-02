pub fn bottles(n: i32) -> String {
    if n < 1 {
        String::from("no more bottles")
    } else if n == 1 {
        String::from("1 bottle")
    } else {
        format!("{} bottles", n)
    }
}

pub fn verse(n: i32) -> String {
    if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }

    let one_it = if n == 1 { "it" } else { "one" };

    let n_bottles = bottles(n);
    let n_minus_one_bottles = bottles(n - 1);

    format!(
        "{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n",
        n_bottles,
        n_bottles,
        one_it,
        n_minus_one_bottles
    )
}

pub fn sing(start: i32, end: i32) -> String {
    let verses: Vec<String> = (end..start + 1).rev().map(|i| verse(i)).collect();

    verses.join("\n")
}
