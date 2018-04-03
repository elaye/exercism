pub fn square_of_sum(n: usize) -> usize {
    let sum = (0..n + 1).sum::<usize>();
    sum * sum
}

pub fn sum_of_squares(n: usize) -> usize {
    (0..n + 1).map(|x| x * x).sum()
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
