use crate::util::{Map2d, Vec2};

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

fn slide_up_single(map: &mut Map2d<Cell>, x: i64) {
    let mut stop = 0;
    let mut mobile_count = 0;

    for y in 0..map.size.y {
        let pos = Vec2::new(x, y);
        match map.get(pos).unwrap() {
            Cell::Empty => (),
            Cell::Mobile => {
                mobile_count += 1;
                *map.get_mut(pos).unwrap() = Cell::Empty;
            },
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

fn slide_up(map: &mut Map2d<Cell>) {
    // Slide each column individually
    for x in 0..map.size.x {
        slide_up_single(map, x);
    }
}

pub fn solve_part_1(input: &Map2d<Cell>) -> i64 {
    let mut map = input.clone();
    slide_up(&mut map);

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

pub fn solve_part_2(input: &Map2d<Cell>) -> u64 {
    0
}
