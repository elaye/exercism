use std::collections::HashSet;

// Number of multiples of p below n
fn nb_multiples(n: u32, p: &u32) -> u32 {
    match n % p {
        0 => n / p,
        r => (n - r) / p  + 1
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();

    factors.iter()
        .for_each(|factor: &u32|
            (1..nb_multiples(limit, factor)).for_each(|i| {
                multiples.insert(i * factor);
            })
        );

    multiples.iter().sum::<u32>()
}
