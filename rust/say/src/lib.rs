fn digits(n: u64) -> Vec<u32> {
    n.to_string().chars().rev().map(|d| d.to_digit(10).unwrap()).collect()
}

fn encode_1_digit(units: u32, order: Option<&str>) -> String {
    let s = match units {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("units must be between 0 and 9")
    };

    match order {
        Some(o) => format!("{} {}", s, o),
        None => s.to_string()
    }
}

fn encode_2_digits(tens: u32, units: u32, order: Option<&str>) -> String {
    let s = match (tens, units) {
        (1, 0) => "ten".to_string(),
        (1, 1) => "eleven".to_string(),
        (1, 2) => "twelve".to_string(),
        (1, 3) => "thirteen".to_string(),
        (1, 4) => "fourteen".to_string(),
        (1, 5) => "fifteen".to_string(),
        (1, 6) => "sixteen".to_string(),
        (1, 7) => "seventeen".to_string(),
        (1, 8) => "eighteen".to_string(),
        (1, 9) => "nineteen".to_string(),

        (2, 0) => "twenty".to_string(),
        (3, 0) => "thirty".to_string(),
        (4, 0) => "forty".to_string(),
        (5, 0) => "fifty".to_string(),
        (6, 0) => "sixty".to_string(),
        (7, 0) => "seventy".to_string(),
        (8, 0) => "eighty".to_string(),
        (9, 0) => "ninety".to_string(),

        (2, i) => format!("twenty-{}", encode_1_digit(i, None)),
        (3, i) => format!("thirty-{}", encode_1_digit(i, None)),
        (4, i) => format!("forty-{}", encode_1_digit(i, None)),
        (5, i) => format!("fifty-{}", encode_1_digit(i, None)),
        (6, i) => format!("sixty-{}", encode_1_digit(i, None)),
        (7, i) => format!("seventy-{}", encode_1_digit(i, None)),
        (8, i) => format!("eighty-{}", encode_1_digit(i, None)),
        (9, i) => format!("ninety-{}", encode_1_digit(i, None)),
        (0, 0) => "".to_string(),
        (_, i) => encode_1_digit(i, None)
    };

    match order {
        Some(o) => format!("{} {}", s, o),
        None => s
    }
}

fn encode_3_digits(hundreds: u32, tens: u32, units: u32, order: Option<&str>) -> String {
    match (hundreds, tens, units) {
        (1, 0, 0) => match order {
            Some(o) => format!("one hundred {}", o),
            None => "one hundred".to_string()
        },
        (0, 0, 0) => "".to_string(),
        (0, 0, u) => encode_1_digit(u, order),
        (0, t, u) => encode_2_digits(t, u, order),
        (h, t, u)  => format!("{} hundred {}", encode_1_digit(h, None), encode_2_digits(tens, units, order))
    }
}

fn encode_chunk(digits: &[u32], order: Option<&str>) -> String {
    println!("chunk {:?}", digits);
    match digits.len() {
        1 => encode_1_digit(digits[0], order),
        2 => encode_2_digits(digits[1], digits[0], order),
        3 => encode_3_digits(digits[2], digits[1], digits[0], order),
        _ => panic!("number must have between 1 and 3 digits")
    }
}

//
// See Rust Language Specific Instructions
// below normal exercise description.
//
pub fn encode(n: u64) -> String {
    let order = vec![None, Some("thousand"), Some("million"), Some("billion"), Some("trillion"), Some("quadrillion"), Some("quintillion")];

    let digits = digits(n);

    // Group digits by 3
    let chunks = digits.chunks(3);

    let groups_str = chunks.zip(order.iter()).map(|(chunk, order)| encode_chunk(chunk, *order))
        .rev()
        .filter(|s| s != "")
        .collect::<Vec<String>>();

    groups_str.join(" ")
}
