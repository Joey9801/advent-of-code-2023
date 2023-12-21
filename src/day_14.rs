use std::collections::HashMap;

use crate::util::{Dir, Map2d, Map2dExt, RotatedMap2d, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Fixed,
    Mobile,
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Fixed,
            'O' => Cell::Mobile,
            _ => panic!("Invalid cell type"),
        }
    }
}

pub fn parse(input: &str) -> Map2d<Cell> {
    Map2d::parse_grid(input, Cell::from_char)
}

fn slide_up_single(map: &mut impl Map2dExt<Cell>, x: i64) {
    let mut stop = 0;
    let mut mobile_count = 0;

    for y in 0..map.size().y {
        let pos = Vec2::new(x, y);
        match map.get(pos).unwrap() {
            Cell::Empty => (),
            Cell::Mobile => {
                mobile_count += 1;
                *map.get_mut(pos).unwrap() = Cell::Empty;
            }
            Cell::Fixed => {
                for y2 in stop..(stop + mobile_count) {
                    let pos2 = Vec2::new(x, y2);
                    *map.get_mut(pos2).unwrap() = Cell::Mobile;
                }

                stop = y + 1;
                mobile_count = 0;
            }
        }
    }

    for y in stop..(stop + mobile_count) {
        let pos = Vec2::new(x, y);
        *map.get_mut(pos).unwrap() = Cell::Mobile;
    }
}

fn slide_up(map: &mut impl Map2dExt<Cell>) {
    // Slide each column individually
    for x in 0..map.size().x {
        slide_up_single(map, x);
    }
}

fn slide(map: &mut Map2d<Cell>, dir: Dir) {
    let mut rotated = match dir {
        Dir::Up => RotatedMap2d { map, up: Dir::Up },
        Dir::Down => RotatedMap2d { map, up: Dir::Down },
        Dir::Left => RotatedMap2d {
            map,
            up: Dir::Right,
        },
        Dir::Right => RotatedMap2d { map, up: Dir::Left },
    };

    slide_up(&mut rotated);
}

fn load(map: &Map2d<Cell>) -> i64 {
    let mut load = 0;

    for (i, cell) in map.data.iter().enumerate() {
        if *cell != Cell::Mobile {
            continue;
        }

        let pos = map.pos_of(i);
        load += map.size.y - pos.y
    }

    load
}

pub fn solve_part_1(input: &Map2d<Cell>) -> i64 {
    let mut map = input.clone();
    slide(&mut map, Dir::Up);
    load(&map)
}

// Enough to store a 100*100 bitmap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey([u64; 157]);

impl From<&Map2d<Cell>> for CacheKey {
    fn from(map: &Map2d<Cell>) -> CacheKey {
        let mut key = [0u64; 157];
        for (i, cell) in map.data.iter().enumerate() {
            if *cell == Cell::Mobile {
                key[i / 64] |= 1 << (i % 64);
            }
        }
        CacheKey(key)
    }
}

pub fn solve_part_2(input: &Map2d<Cell>) -> i64 {
    let mut map = input.clone();

    let cycle = |map: &mut Map2d<Cell>| {
        slide(map, Dir::Up);
        slide(map, Dir::Left);
        slide(map, Dir::Down);
        slide(map, Dir::Right);
    };

    // Maps map state -> the first cycle number that state was seen
    let mut seen = HashMap::<CacheKey, usize>::new();

    let mut first_seen = 0;
    let mut second_seen = 0;
    for i in 0..1_000_000_000 {
        if let Some(previous) = seen.insert(CacheKey::from(&map), i) {
            first_seen = previous;
            second_seen = i;
            break;
        }
        cycle(&mut map);
    }

    let preamble = first_seen;
    let period = second_seen - first_seen;

    // The map is currently still at the repeated point of the cycle
    let remaining = (1_000_000_000 - preamble) % period;
    for _ in 0..remaining {
        cycle(&mut map);
    }

    load(&map)
}
