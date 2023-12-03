use anyhow::bail;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    value: u32,
    range: std::ops::Range<usize>,
}

impl Number {
    fn expanded_range(&self) -> std::ops::Range<usize> {
        (self.range.start.saturating_sub(1))..(self.range.end + 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Symbol {
    value: char,
    idx: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Number(Number),
    Symbol(Symbol),
}

#[derive(Debug, PartialEq)]
pub struct Line(Vec<Element>);

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Consecutive digits become a single Element::Number
        // Non '.' or digit characters become a single Element::Symbol

        if !s.is_ascii() {
            bail!("Input must be ASCII")
        }
        let s = s.as_bytes();

        let mut elements = Vec::new();
        let mut idx = 0;
        while idx < s.len() {
            match s[idx] {
                b'.' => {
                    idx += 1;
                }
                b'0'..=b'9' => {
                    let start_idx = idx;
                    while idx < s.len() && s[idx].is_ascii_digit() {
                        idx += 1;
                    }
                    let end_idx = idx;
                    let value = std::str::from_utf8(&s[start_idx..end_idx])?.parse()?;
                    elements.push(Element::Number(Number {
                        value,
                        range: start_idx..end_idx,
                    }));
                }
                _ => {
                    elements.push(Element::Symbol(Symbol {
                        value: s[idx] as char,
                        idx,
                    }));
                    idx += 1;
                }
            }
        }

        Ok(Line(elements))
    }
}

impl Line {
    fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.0.iter().filter_map(|element| match element {
            Element::Symbol(symbol) => Some(*symbol),
            _ => None,
        })
    }

    fn symbol_indexes(&self) -> impl Iterator<Item = usize> + '_ {
        self.symbols().map(|symbol| symbol.idx)
    }

    fn numbers(&self) -> impl Iterator<Item = Number> + '_ {
        self.0.iter().filter_map(|element| match element {
            Element::Number(number) => Some(number.clone()),
            _ => None,
        })
    }
}

pub fn parse(input: &str) -> Vec<Line> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_part_1(input: &[Line]) -> u32 {
    let mut sum = 0;
    let mut symbols = Vec::new();
    for i in 0..input.len() {
        symbols.clear();
        symbols.extend(input[i].symbol_indexes());
        if i > 0 {
            symbols.extend(input[i - 1].symbol_indexes());
        }
        if i < input.len() - 1 {
            symbols.extend(input[i + 1].symbol_indexes());
        }
        symbols.sort();

        for number in input[i].numbers() {
            let range = number.expanded_range();
            if symbols.iter().any(|idx| range.contains(idx)) {
                sum += number.value;
            }
        }
    }

    sum
}

pub fn solve_part_2(input: &[Line]) -> u32 {
    let mut sum = 0;
    let mut numbers = Vec::new();

    // For each '*' symbol, if it is adjacent to exactly two numbers, multiply
    // those numbers together and add the result to the sum
    for i in 0..input.len() {
        numbers.clear();
        numbers.extend(input[i].numbers());
        if i > 0 {
            numbers.extend(input[i - 1].numbers());
        }
        if i < input.len() - 1 {
            numbers.extend(input[i + 1].numbers());
        }

        let gear_indexes = input[i]
            .symbols()
            .filter(|sym| sym.value == '*')
            .map(|sym| sym.idx);
        for idx in gear_indexes {
            let mut numbers = numbers
                .iter()
                .filter(|number| number.expanded_range().contains(&idx))
                .map(|number| number.value);

            let first = numbers.next();
            let second = numbers.next();
            let third = numbers.next();
            match (first, second, third) {
                (Some(first), Some(second), None) => {
                    sum += first * second;
                }
                _ => {}
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let raw = "467..114..
...*......
617*......";

        let parsed = parse(raw);

        let expected = vec![
            Line(vec![
                Element::Number(Number {
                    value: 467,
                    range: 0..3,
                }),
                Element::Number(Number {
                    value: 114,
                    range: 5..8,
                }),
            ]),
            Line(vec![Element::Symbol(Symbol { value: '*', idx: 3 })]),
            Line(vec![
                Element::Number(Number {
                    value: 617,
                    range: 0..3,
                }),
                Element::Symbol(Symbol { value: '*', idx: 3 }),
            ]),
        ];

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_part_1() {
        let input = parse(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );

        assert_eq!(solve_part_1(&input), 4361);
    }

    #[test]
    fn test_part_2() {
        let input = parse(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );

        assert_eq!(solve_part_2(&input), 467835);
    }
}
