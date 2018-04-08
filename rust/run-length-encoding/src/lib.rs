#[macro_use]
extern crate nom;

use nom::{digit};

// Decoding parser combinators

named!(numeric_string<&str>,
       map_res!(digit, std::str::from_utf8)
);

named!(number<u32>,
       map_res!(numeric_string, std::str::FromStr::from_str)
);

named!(one_char<&str>, map_res!(take!(1), std::str::from_utf8));

named!(chunk<(u32, &str)>,
       tuple!(number, one_char)
);

named!(either_chunk<(u32, &str)>,
       alt!(chunk | tuple!(value!(1), one_char))
);

named!(chunks<Vec<(u32, &str)>>,
       many1!(either_chunk)
);

// ---------------------------->

pub fn decode(s: &str) -> String {
    if s == "" {
        return String::new();
    }

    let parsed_chunks = chunks(s.as_bytes()).unwrap().1;

    let f = |acc, &(n, ch): &(u32, &str)| format!("{}{}", acc, ch.repeat(n as usize));

    parsed_chunks.iter().fold(String::new(), f)
}

pub fn encode(s: &str) -> String {
    let mut chars = s.chars();

    let mut prev = None;
    let mut vec = Vec::new();

    while let Some(c) = chars.next() {
        match prev {
            None => {
                prev = Some(c);
                vec.push((c, 1));
            },
            Some(p) => {
                if p == c {
                    let last = vec.last_mut().unwrap();
                    last.1 = last.1 + 1;
                } else {
                    vec.push((c, 1));
                    prev = Some(c)
                }
            }
        }
    }

    vec.iter().fold(String::new(), |acc, &(ch, n)| {
        match n {
            1 => format!("{}{}", acc, ch),
            i => format!("{}{}{}", acc, i, ch)
        }
    })
}
