pub fn factors(n: u64) -> Vec<u32> {
    let mut p = n as u64;
    let mut factors = Vec::new();

    let mut div = 2;

    while p != 1 {
        if p % div == 0 {
            p = p / div;
            factors.push(div as u32);
        } else {
            div = div + 1;
        }
    }

    factors
}
