// Quotient of euclidian division of n by p
fn quotient(n: u32, p: &u32) -> u32 {
    match n % p {
        0 => n / p - 1,
        r => (n - r) / p
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = factors
        .iter()
        .filter(|factor| **factor < limit)
        .map(|factor: &u32| (1..quotient(limit, factor) + 1)
             .map(|i| i * factor).collect::<Vec<u32>>()
        )
        .flat_map(|multiples| multiples.into_iter())
        .collect::<Vec<u32>>();

    multiples.sort();

    multiples.dedup();

    multiples.iter().sum::<u32>()
}
