use std::collections::HashSet;

use crate::util::{pairs, Vec2};

pub fn parse(input: &str) -> Vec<Vec2> {
    let mut positions = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                positions.push(Vec2 {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    positions
}

pub fn expand_universe(input: &[Vec2], multiple: i64) -> Vec<Vec2> {
    // Compute the bounds of the universe
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    for point in input {
        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
    }

    // Identify all the x's and y's that have no points
    let mut vacant_x = HashSet::from_iter(0..max_x);
    let mut vacant_y = HashSet::from_iter(0..max_y);
    for point in input {
        vacant_x.remove(&point.x);
        vacant_y.remove(&point.y);
    }

    // For each x and each y, the cumulative number of vacant columns/rows
    let cumsum = |set: &HashSet<i64>| -> Vec<i64> {
        let mut cumsum = Vec::new();
        for x in 0..=max_x {
            let prev = if x == 0 { 0 } else { cumsum[x as usize - 1] };

            let diff = if set.contains(&x) { 1 } else { 0 };
            cumsum.push(diff + prev);
        }

        cumsum
    };
    let vacant_x_cumulative = cumsum(&vacant_x);
    let vacant_y_cumulative = cumsum(&vacant_y);

    input
        .iter()
        .map(|point| Vec2 {
            x: point.x + (vacant_x_cumulative[point.x as usize] * (multiple - 1)),
            y: point.y + (vacant_y_cumulative[point.y as usize] * (multiple - 1)),
        })
        .collect()
}

pub fn solve_part_1(input: &[Vec2]) -> i64 {
    let expanded = expand_universe(input, 2);
    pairs(&expanded).map(|(a, b)| (a - b).l1_norm()).sum()
}

pub fn solve_part_2(input: &[Vec2]) -> i64 {
    let expanded = expand_universe(input, 1_000_000);
    pairs(&expanded).map(|(a, b)| (a - b).l1_norm()).sum()
}
