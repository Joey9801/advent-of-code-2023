use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug)]
pub struct Card {
    winning_numbers: Vec<u8>,
    our_numbers: Vec<u8>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strings like "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        // => winning: [41, 48, 83, 86, 17]
        //    ours:    [83, 86, 6, 31, 17, 9, 48, 53]

        let (_card_str, numbers_str) = s.split_once(": ").ok_or(anyhow!("Invalid card string"))?;
        let (winning_str, ours_str) = numbers_str
            .split_once(" | ")
            .ok_or(anyhow!("Invalid card string"))?;
        let winning_numbers = winning_str
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<u8>, _>>()?;
        let our_numbers = ours_str
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<u8>, _>>()?;

        Ok(Self {
            winning_numbers,
            our_numbers,
        })
    }
}

pub fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn solve_part_1(input: &[Card]) -> u32 {
    let mut sum = 0;

    for card in input {
        // Count the number of winning numbers that are in our numbers
        let num_winning = card
            .winning_numbers
            .iter()
            .filter(|n| card.our_numbers.contains(n))
            .count();
        if num_winning > 0 {
            sum += 1 << (num_winning as u32 - 1);
        }
    }

    sum
}

pub fn solve_part_2(input: &[Card]) -> u32 {
    let mut card_counts = vec![1; input.len()];

    for i in 0..input.len() {
        let num_winning = input[i]
            .winning_numbers
            .iter()
            .filter(|n| input[i].our_numbers.contains(n))
            .count();

        for x in 0..num_winning {
            let x = i + x + 1;
            if x < input.len() {
                card_counts[x] += card_counts[i];
            }
        }
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_1(&input), 13);
    }

    #[test]
    fn test_part_2() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_2(&input), 30);
    }
}
