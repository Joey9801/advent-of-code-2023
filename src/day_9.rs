pub fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(values: impl ExactSizeIterator<Item = i64>) -> i64 {
    let len = values.len() as i64;
    let coefficients = (0..)
        .map(|i| crate::util::binomial_coefficient(len, i) * (-1i64).pow((i + len + 1) as u32));

    values
        .zip(coefficients)
        .map(|(value, coef)| value * coef)
        .sum::<i64>()
}

pub fn solve_part_1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|row| extrapolate(row.iter().copied()))
        .sum()
}

pub fn solve_part_2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|row| extrapolate(row.iter().rev().copied()))
        .sum()
}
