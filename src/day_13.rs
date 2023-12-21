use std::str::FromStr;

use crate::util::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug)]
pub struct Map {
    size: Vec2,
    tiles: Vec<Tile>,
}

impl Map {
    fn pos_to_index(&self, pos: Vec2) -> usize {
        (pos.y * self.size.x + pos.x) as usize
    }

    fn get(&self, pos: Vec2) -> Option<Tile> {
        if pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y {
            None
        } else {
            Some(self.tiles[self.pos_to_index(pos)])
        }
    }

    fn row_bitmap(&self, y: i64) -> u64 {
        let mut bitmap = 0u64;
        for x in 0..self.size.x {
            if self.get(Vec2 { x, y }).unwrap() == Tile::Rock {
                bitmap |= 1 << x;
            }
        }
        bitmap
    }

    fn col_bitmap(&self, x: i64) -> u64 {
        let mut bitmap = 0u64;
        for y in 0..self.size.y {
            if self.get(Vec2 { x, y }).unwrap() == Tile::Rock {
                bitmap |= 1 << y;
            }
        }
        bitmap
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size_x = s.lines().next().unwrap().len();
        let size_y = s.lines().count();
        let size = Vec2 {
            x: size_x as i64,
            y: size_y as i64,
        };

        debug_assert!(size_x <= 64);
        debug_assert!(size_y <= 64);

        let tiles = s
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| match c {
                '#' => Tile::Rock,
                '.' => Tile::Ash,
                _ => panic!("Invalid tile: {}", c),
            })
            .collect();

        Ok(Self { size, tiles })
    }
}

pub fn parse(input: &str) -> Vec<Map> {
    input.split("\n\n").map(|s| s.parse().unwrap()).collect()
}

fn find_reflection(values: &[u64], required_bit_errors: u32) -> Option<u64> {
    (1..values.len())
        .find(move |test| {
            let left = values[0..*test].iter().rev();
            let right = values[*test..].iter();
            let errors = left
                .zip(right)
                .map(|(l, r)| l ^ r)
                .map(|x| x.count_ones())
                .sum::<u32>();
            errors == required_bit_errors
        })
        .map(|x| x as u64)
}

pub fn solve(input: &[Map], required_bit_errors: u32) -> u64 {
    let mut sum = 0;
    for map in input.iter() {
        let cols = (0..map.size.x)
            .map(|x| map.col_bitmap(x))
            .collect::<Vec<_>>();
        let rows = (0..map.size.y)
            .map(|y| map.row_bitmap(y))
            .collect::<Vec<_>>();

        if let Some(x) = find_reflection(&cols, required_bit_errors) {
            sum += x;
        } else if let Some(y) = find_reflection(&rows, required_bit_errors) {
            sum += y * 100;
        }
    }

    sum
}

pub fn solve_part_1(input: &[Map]) -> u64 {
    solve(input, 0)
}

pub fn solve_part_2(input: &[Map]) -> u64 {
    solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE_INPUT);
        assert_eq!(solve_part_1(&input), 405);
    }
}
