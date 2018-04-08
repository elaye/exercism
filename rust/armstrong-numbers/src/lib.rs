fn digits(n: u32) -> Vec<u32> {
    n.to_string().chars().rev().map(|d| d.to_digit(10).unwrap()).collect()
}

pub fn is_armstrong_number(_num: u32) -> bool {
    let digits = digits(_num);
    let n = digits.len();
    digits.iter().map(|i| i.pow(n as u32)).sum::<u32>() == _num
}
