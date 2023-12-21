use crate::util::{Vec2, Map2d};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Ash,
    Rock,
}

pub fn parse(input: &str) -> Vec<Map2d<Tile>> {
    let parse_char = |c| match c {
        '#' => Tile::Rock,
        '.' => Tile::Ash,
        _ => panic!("Invalid tile: {}", c),
    };
    
    input.split("\n\n").map(|s| Map2d::parse_grid(s, parse_char)).collect()
}

fn row_bitmap(map: &Map2d<Tile>, y: i64) -> u64 {
    let mut bitmap = 0u64;
    for x in 0..map.size.x {
        if map.get(Vec2 { x, y }).unwrap() == Tile::Rock {
            bitmap |= 1 << x;
        }
    }
    bitmap
}

fn col_bitmap(map: &Map2d<Tile>, x: i64) -> u64 {
    let mut bitmap = 0u64;
    for y in 0..map.size.y {
        if map.get(Vec2 { x, y }).unwrap() == Tile::Rock {
            bitmap |= 1 << y;
        }
    }
    bitmap
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

pub fn solve(input: &[Map2d<Tile>], required_bit_errors: u32) -> u64 {
    let mut sum = 0;
    for map in input.iter() {
        let cols = (0..map.size.x)
            .map(|x| col_bitmap(map, x))
            .collect::<Vec<_>>();
        let rows = (0..map.size.y)
            .map(|y| row_bitmap(map, y))
            .collect::<Vec<_>>();

        if let Some(x) = find_reflection(&cols, required_bit_errors) {
            sum += x;
        } else if let Some(y) = find_reflection(&rows, required_bit_errors) {
            sum += y * 100;
        }
    }

    sum
}

pub fn solve_part_1(input: &[Map2d<Tile>]) -> u64 {
    solve(input, 0)
}

pub fn solve_part_2(input: &[Map2d<Tile>]) -> u64 {
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
