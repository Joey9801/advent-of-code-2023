use std::collections::{hash_map::Entry, HashMap};
use std::str::FromStr;

fn aoc_hash(chars: impl Iterator<Item = char>) -> u8 {
    let mut hash = 0u32;
    for c in chars {
        hash = hash + c as u32;
        hash = hash * 17;
        hash = hash % 256;
    }

    hash as u8
}

#[derive(Clone, Debug)]
pub enum Operation {
    Insert { label: String, value: u8 },
    Remove { label: String },
}

impl Operation {
    fn chars(&self) -> impl Iterator<Item = char> + '_ {
        match self {
            Operation::Insert { label, value } => label
                .chars()
                .chain(std::iter::once('='))
                .chain(std::iter::once(Some((value + b'0') as char)).flatten()),
            Operation::Remove { label } => label
                .chars()
                .chain(std::iter::once('-'))
                .chain(std::iter::once(None).flatten()),
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut label = String::new();

        let mut chars = s.chars();
        for c in chars.by_ref() {
            match c {
                c if c.is_alphabetic() => label.push(c),
                '-' => return Ok(Operation::Remove { label }),
                '=' => break,
                _ => return Err(()),
            }
        }

        let value = chars
            .next()
            .and_then(|c| c.to_digit(10))
            .map(|c| c as u8)
            .ok_or(())?;

        Ok(Operation::Insert { label, value })
    }
}

pub fn parse(input: &str) -> Vec<Operation> {
    input
        .split(',')
        .map(Operation::from_str)
        .map(Result::unwrap)
        .collect()
}

pub fn solve_part_1(input: &[Operation]) -> u64 {
    input.iter().map(|op| aoc_hash(op.chars()) as u64).sum()
}

pub fn solve_part_2(input: &[Operation]) -> usize {
    // Each box contains a map from label -> (global idx, power))
    let mut boxes: [HashMap<&str, (usize, u8)>; 256] = std::array::from_fn(|_| HashMap::new());

    for (idx, op) in input.iter().enumerate() {
        match op {
            Operation::Insert { label, value } => {
                let box_ref = &mut boxes[aoc_hash(label.chars()) as usize];
                match box_ref.entry(label.as_str()) {
                    Entry::Occupied(mut entry) => {
                        // Overwrites keep the old index, but do update the value
                        let (_, old_value) = entry.get_mut();
                        *old_value = *value;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert((idx, *value));
                    }
                }
            }
            Operation::Remove { label } => {
                let box_ref = &mut boxes[aoc_hash(label.chars()) as usize];
                box_ref.remove(label.as_str());
            }
        }
    }

    // Now each boxes values can be sorted by original insertion order, and the
    // answer computed
    let mut sum = 0;
    for (box_idx, box_ref) in boxes.iter().enumerate() {
        let mut sorted: Vec<_> = box_ref.values().collect();
        sorted.sort_by_key(|(idx, _)| idx);
        sum += sorted
            .iter()
            .enumerate()
            .map(|(lens_idx, (_, power))| (box_idx + 1) * (lens_idx + 1) * *power as usize)
            .sum::<usize>();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_hash() {
        assert_eq!(aoc_hash("HASH".chars()), 52);
    }
}
