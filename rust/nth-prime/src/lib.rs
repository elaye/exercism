// Test whether a number n is a prime by checking if
// it can be divided by any previous prime between
// 2 and sqrt(n)
fn is_prime(n: u32, primes: &Vec<u32>) -> bool {
    let divisor_max = (n as f64).sqrt().ceil() as u32;
    !primes
        .into_iter()
        .filter(|i| **i <= divisor_max)
        .any(|i| n % *i == 0)
}

// Find the next prime number given
// a list of all the previous prime numbers
fn next_prime(primes: &Vec<u32>) -> u32 {
    // If there's no previous prime number,
    // return the first prime number
    if primes.len() < 1 {
        return 2;
    }

    if primes.len() < 2 {
        return 3;
    }

    let mut n = primes[primes.len() - 1] + 2;

    while !is_prime(n, primes) {
        // We only check odd numbers
        n = n + 2;
    }

    n
}

// Find the nth prime number
pub fn nth(n: u32) -> Option<u32> {
    if n < 1 {
        return None;
    }

    let mut primes = Vec::new();

    while primes.len() < n as usize {
        let prime = next_prime(&primes);
        primes.push(prime);
    }

    Some(primes[primes.len() - 1])
}
