use std::str::FromStr;

use crate::util::{Dir, Vec2};

#[derive(Debug)]
pub struct Instruction {
    dir: Dir,
    digit: u32,
    code: u32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // From a string like "R 6 (#70c710)"

        let mut parts = s.split_whitespace();
        let dir = match parts.next().unwrap() {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("Invalid direction"),
        };

        let digit = parts.next().unwrap().parse().unwrap();

        let code = parts
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(")");
        let code = u32::from_str_radix(code, 16).unwrap();

        Ok(Instruction { dir, digit, code })
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// Yields all the vertices of the path once
fn vertices(instructions: impl Iterator<Item = (Dir, i64)>) -> impl Iterator<Item = Vec2> {
    let mut pos = Vec2::new(0, 0);

    instructions.map(move |(dir, distance)| {
        pos += dir.to_vec2() * distance;
        pos
    })
}

pub fn solve(instructions: impl Iterator<Item = (Dir, i64)> + Clone) -> i64 {
    let vertices = || vertices(instructions.clone());

    // The shoelace formula for the area of a polygon
    // A = 1/2 * ∑(y_i + y_(i+1_)) * (x_i - x_(i+1_)
    let shifted = vertices()
        .skip(1)
        .chain(std::iter::once(vertices().next().unwrap()));
    let pairs = vertices().zip(shifted);
    let mut shoelace_area = 0;
    for (a, b) in pairs {
        shoelace_area += (a.y + b.y) * (a.x - b.x)
    }
    shoelace_area /= 2;

    // The shoelace formula doesn't quite give us the right answer as our
    // indices are effectively at the center of each grid square rather than on
    // the 'outer' edges of each square that makes up our boundary.

    // Pick's theorem: A = i + b/2 - 1
    // Where A is the area of the polygon, i is the number of interior points
    // and b is the number of boundary points
    let boundary_count = instructions.map(|(_, distance)| distance).sum::<i64>();
    let interior_count = shoelace_area - boundary_count / 2 + 1;

    // Our actual area is the number of boundary points + the number of interior points
    boundary_count + interior_count
}

pub fn solve_part_1(input: &[Instruction]) -> i64 {
    solve(input.iter().map(|i| (i.dir, i.digit as i64)))
}

pub fn solve_part_2(input: &[Instruction]) -> i64 {
    solve(input.iter().map(|i| {
        let dir = match i.code & 0b11 {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => unreachable!(),
        };
        let distance = (i.code >> 4) as i64;

        (dir, distance)
    }))
}
