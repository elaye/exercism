pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input.iter().enumerate().map(|(i, row)|
        row.iter().enumerate().filter_map(|(j, v)| {
            if row.iter().max() == Some(v) {
                let col_min = input.iter().map(|row| row[j]).min();

                if col_min == Some(*v) {
                    return Some((i, j));
                }

                return None;
            }

            return None;
        }).collect::<Vec<(usize, usize)>>()
    ).flat_map(|x| x).collect()
}
