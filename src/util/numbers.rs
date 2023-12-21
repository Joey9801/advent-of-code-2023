pub fn gcm(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcm(a, b)
}

pub fn lcm_iter<I>(mut iter: I) -> i64
where
    I: Iterator<Item = i64>,
{
    let mut result = iter.next().unwrap();
    for i in iter {
        result = lcm(result, i);
    }
    result
}

/// Return the number of ways to choose k items from n items without repetition
/// and without order.
pub fn binomial_coefficient(n: i64, k: i64) -> i64 {
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(super::binomial_coefficient(5, 3), 10);
        assert_eq!(super::binomial_coefficient(5, 2), 10);
        assert_eq!(super::binomial_coefficient(5, 1), 5);
        assert_eq!(super::binomial_coefficient(5, 0), 1);
    }
}
