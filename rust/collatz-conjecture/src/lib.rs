// return Some(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut p = n;

    let mut i = 0;

    while p != 1 {
        match p % 2 {
            0 => p = p / 2,
            _ => p = 3 * p + 1
        }

        i = i + 1;
    }

    Some(i)
}
