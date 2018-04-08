#[macro_use]
extern crate nom;

// use nom;

// named!(chunk<&str, (usize, char)>,
//        alt!(
//            do_parse!(
//                u32!(be_u32) >> take!(1)
//            )
//                |
//            take!(1)
//        )
// )

// named!(encoded_str<&str, Vec<(usize, char)>>,
//        many0!(chunk)
// )

// named!(chunk<&str, (u32, char)>,
//     do_parse!(
//         nb: u32!(nom::Endianness::Big) >> ch: take!(1) >> (nb, ch)
//     )
// );

use nom::{digit};

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

pub fn test_chunks(s: &str) -> Vec<(u32, &str)> {
    chunks(s.as_bytes()).unwrap().1
}

pub fn test_either_chunk(s: &str) -> (u32, &str) {
    either_chunk(s.as_bytes()).unwrap().1
}

pub fn test_chunk(s: &str) -> (u32, &str) {
    chunk(s.as_bytes()).unwrap().1
}

pub fn test_number(s: &str) -> u32 {
    number(s.as_bytes()).unwrap().1
}

// named!(number<u32>, map_res!(take_while!(is_digit), std::str::from_utf8));

// named!(chunk<(u32, &[u8])>,
//     tuple!(number, take!(1))
// );

// tuple!(many1!(be_u32), take!(1))

// named!(chunk<(u32, &str)>,
//        tuple!(u32!(nom::Endianness::Big), take!(1))
// );



// pub fn test_nom(s: &str) -> (u32, char) {
//     let r = chunk(s.as_bytes());
//     println!("{:?}", r);
//     (1, 'a')
// }

pub fn decode(s: &str) -> String {
    String::from("")
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

// pub fn encode(s: &str) -> String {
//     let f = |mut (prev_char, acc): (char, Vec<(char, usize)>), c| {
//         match prev_char {
//             Some(prev) => if c == prev {
//                 last.1 = last.1 + 1;
//             } else {
//                 vec.push((c, 1));
//             },
//             None => vec.push((c, 1))
//         }

//         (c, acc)
//     };

//     s.chars().fold((None, Vec::new()), f)
//         .iter().fold(String::new(), |acc, &(c, n)| format!("{}{}{}", acc, n, c))
// }
