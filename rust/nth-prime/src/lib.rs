struct Primes(Vec<u32>);

impl Primes {
    pub fn new() -> Primes {
        Primes(Vec::new())
    }

    pub fn get_vec_mut(&mut self) -> &mut Vec<u32> {
        &mut self.0
    }

    pub fn get_vec(&self) -> &Vec<u32> {
        &self.0
    }

    // Test whether a number n is a prime by checking if
    // it can be divided by any previous prime between
    // 2 and sqrt(n)
    fn is_prime(&self, n: u32) -> bool {
        let divisor_max = (n as f64).sqrt().ceil() as u32;
        !self.0
            .iter()
            .filter(|i| **i <= divisor_max)
            .any(|i| n % *i == 0)
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = {
            let vec = self.get_vec();

            match vec.len() {
                0 => 2,
                1 => 3,
                _ => {
                    let mut n = vec[vec.len() - 1] + 2;

                    while !self.is_prime(n) {
                        n = n + 2;
                    }

                    n
                }
            }
        };

        self.get_vec_mut().push(next);
        Some(next)
    }
}

// Find the nth prime number
pub fn nth(n: u32) -> Option<u32> {
    if n < 1 {
        return None;
    }

    let mut primes = Primes::new();
    primes.nth(n as usize - 1)
}
