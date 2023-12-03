pub fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn solve_part_1(input: &[String]) -> u32 {
    let mut sum = 0;
    for line in input {
        // First and last digit character of the string
        let mut digits = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap());

        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        let num = first * 10 + last;
        sum += num;
    }
    sum
}

const DIGIT_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct Part2Digits<'a> {
    source: &'a str,
}

impl<'a> Iterator for Part2Digits<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.source.is_empty() {
            // If the first char is a digit, return it
            if let Some(digit) = self.source.chars().next().and_then(|c| c.to_digit(10)) {
                self.source = &self.source[1..];
                return Some(digit);
            }

            // Could do something fancy based around common prefixes here, but
            // it is probably fast enough to just brute-force search through all
            // of the possible digit strings instead
            for (digit_idx, digit_str) in DIGIT_STRS.iter().enumerate() {
                if self.source.starts_with(digit_str) {
                    self.source = &self.source[1..];
                    return Some(digit_idx as u32 + 1);
                }
            }

            // If we get here, we didn't find a digit or digit string, so skip
            // the first character and try again
            self.source = &self.source[1..];
        }

        None
    }
}

pub fn solve_part_2(input: &[String]) -> u32 {
    let mut sum = 0;
    for line in input {
        let mut digits = Part2Digits { source: line };
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        let num = first * 10 + last;
        sum += num;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );

        assert_eq!(solve_part_1(&input), 142);
    }

    #[test]
    fn test_part_2_digits() {
        let line = "fivetwoqmlk22eightfive";
        assert_eq!(
            vec![5, 2, 2, 2, 8, 5],
            Part2Digits { source: line }.collect::<Vec<_>>()
        );

        // "zero" isn't a digit in this problem
        let line = "zeroonetwo012";
        assert_eq!(
            vec![1, 2, 0, 1, 2],
            Part2Digits { source: line }.collect::<Vec<_>>()
        );

        // Stupid overlapping words
        let line = "eightwo";
        assert_eq!(vec![8, 2], Part2Digits { source: line }.collect::<Vec<_>>());
    }

    #[test]
    fn test_part_2() {
        let input = parse(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(solve_part_2(&input), 281);
    }
}
