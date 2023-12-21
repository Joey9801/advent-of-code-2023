use std::collections::HashMap;

pub struct Row {
    cells: Vec<u8>,
    blocks: Vec<usize>,
}

pub fn parse(input: &str) -> Vec<Row> {
    // Input like:
    //
    //  ???.### 1,1,3
    //  .??..??...?##. 1,1,3
    //  ?#?#?#?#?#?#?#? 1,3,1,6

    let mut rows = Vec::new();
    for line in input.lines() {
        let (cells, blocks) = line.split_once(' ').unwrap();
        let cells = cells.as_bytes().to_vec();
        let blocks = blocks.split(',').map(|s| s.parse().unwrap()).collect();

        rows.push(Row { cells, blocks })
    }

    rows
}

// Does the given block fit in the given position?
fn can_fit(cells: &[u8], pos: usize, block: usize) -> bool {
    if pos + block > cells.len() {
        return false;
    }

    let prefix = &cells[0..pos];
    let range = &cells[pos..(pos + block)];
    let postfix = if pos + block < cells.len() {
        cells[pos + block]
    } else {
        b'.'
    };

    let mut fits = true;

    if prefix.iter().any(|&c| c == b'#') {
        fits = false;
    }

    if range.iter().any(|&c| c == b'.') {
        fits = false;
    }

    if postfix == b'#' {
        fits = false;
    }

    fits
}


// Since we never mutate cells/blocks, only trimming elements from the front, we
// can memoize the results on the lengths of the slices rather than their contents.
fn count_ways_to_fit(cells: &[u8], blocks: &[usize], memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if blocks.is_empty() {
        if cells.iter().all(|c| *c != b'#') {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(memo) = memo.get(&(cells.len(), blocks.len())) {
        return *memo;
    }

    let slack = match cells
        .len()
        .checked_sub(blocks.iter().sum::<usize>())
        .and_then(|x| x.checked_sub(blocks.len() - 1))
    {
        Some(slack) => slack,
        None => return 0,
    };

    let mut sum = 0;
    for pos in 0..=slack {
        if can_fit(cells, pos as usize, blocks[0]) {
            let cut = std::cmp::min(cells.len(), pos + blocks[0] + 1);
            let remaining = &cells[cut..];
            sum += count_ways_to_fit(remaining, &blocks[1..], memo);
        }
    }

    memo.insert((cells.len(), blocks.len()), sum);
    sum
}

pub fn solve_part_1(input: &[Row]) -> u64 {
    input
        .iter()
        .map(|row| count_ways_to_fit(&row.cells, &row.blocks, &mut HashMap::new()))
        .sum()
}

pub fn solve_part_2(input: &[Row]) -> u64 {
    input
        .iter()
        .map(|row| {
            let mut cells = Vec::new();
            for _ in 0..5 {
                cells.extend_from_slice(&row.cells);
                cells.push(b'?');
            }
            cells.pop();

            let blocks = row.blocks.repeat(5);

            Row { cells, blocks}
        })
        .map(|row| count_ways_to_fit(&row.cells, &row.blocks, &mut HashMap::new()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE_INPUT);
        assert_eq!(solve_part_1(&input), 21);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE_INPUT);
        assert_eq!(solve_part_2(&input), 525152);
    }
}
