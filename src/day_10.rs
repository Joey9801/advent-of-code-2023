use crate::util::{Dir, Map2d, Vec2, Map2dExt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    UpDown,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    RightLeft,
    Starting,
    Empty,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Cell::UpDown,
            '-' => Cell::RightLeft,
            'L' => Cell::UpRight,
            'J' => Cell::UpLeft,
            '7' => Cell::DownLeft,
            'F' => Cell::DownRight,
            'S' => Cell::Starting,
            _ => Cell::Empty,
        }
    }

    const fn dir_pair(self) -> Option<(Dir, Dir)> {
        match self {
            Cell::UpDown => Some((Dir::Up, Dir::Down)),
            Cell::UpRight => Some((Dir::Up, Dir::Right)),
            Cell::UpLeft => Some((Dir::Up, Dir::Left)),
            Cell::DownRight => Some((Dir::Down, Dir::Right)),
            Cell::DownLeft => Some((Dir::Down, Dir::Left)),
            Cell::RightLeft => Some((Dir::Right, Dir::Left)),
            Cell::Starting => None,
            Cell::Empty => None,
        }
    }

    fn from_dir_pair(dir1: Dir, dir2: Dir) -> Self {
        let dir1 = std::cmp::min(dir1, dir2);
        let dir2 = std::cmp::max(dir1, dir2);

        match (dir1, dir2) {
            (Dir::Up, Dir::Down) => Cell::UpDown,
            (Dir::Up, Dir::Right) => Cell::UpRight,
            (Dir::Up, Dir::Left) => Cell::UpLeft,
            (Dir::Down, Dir::Right) => Cell::DownRight,
            (Dir::Down, Dir::Left) => Cell::DownLeft,
            (Dir::Right, Dir::Left) => Cell::RightLeft,
            _ => panic!("Invalid dir pair {:?}, {:?}", dir1, dir2),
        }
    }

    fn connects(&self, dir: Dir) -> bool {
        self.dir_pair()
            .map(|(dir1, dir2)| dir == dir1 || dir == dir2)
            .unwrap_or(false)
    }

    fn exit_dir(&self, entry_dir: Dir) -> Dir {
        let (dir1, dir2) = self.dir_pair().unwrap();
        if entry_dir == dir1 {
            dir2
        } else if entry_dir == dir2 {
            dir1
        } else {
            panic!("Cell {:?} does not connect to {:?}", self, entry_dir);
        }
    }
}

pub struct Input {
    map: Map2d<Cell>,
    source: Vec2,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

pub fn parse(input: &str) -> Input {
    let mut map = Map2d::parse_grid(input, Cell::from_char);

    let source = map.find(|x| *x == Cell::Starting).unwrap();

    // Work out what the starting cell is
    let mut candidate_connections = Vec::new();
    for dir in Dir::ALL {
        let neighbor = map.get(source + dir).unwrap();
        if neighbor.connects(dir.opposite()) {
            candidate_connections.push(dir);
        }
    }

    // Cross our fingers and hope that deducing what the starting cell is is
    // trivial
    assert_eq!(candidate_connections.len(), 2);

    let source_cell = Cell::from_dir_pair(candidate_connections[0], candidate_connections[1]);
    *map.get_mut(source).unwrap() = source_cell;

    Input { map, source }
}

/// Iterate the coordinates of all the tiles in the pipe loop
fn iter_pipe_loop(input: &Input) -> impl Iterator<Item = Vec2> + '_ {
    // Trace around the map until we get back to the starting cell
    let initial_dir = input.map.get(input.source).unwrap().dir_pair().unwrap().0;
    let mut pos = input.source + initial_dir;
    let mut from_dir = initial_dir.opposite();

    let rest = std::iter::from_fn(move || {
        if pos == input.source {
            return None;
        }

        let this_pos = pos;

        let cell = input.map.get(pos).unwrap();
        let exit_dir = cell.exit_dir(from_dir);

        pos = pos + exit_dir;
        from_dir = exit_dir.opposite();

        Some(this_pos)
    });

    std::iter::once(input.source).chain(rest)
}

pub fn solve_part_1(input: &Input) -> u64 {
    iter_pipe_loop(input).count() as u64 / 2
}

pub fn solve_part_2(input: &Input) -> u64 {
    // Create a second map with just the loop elements
    let mut loop_map = Map2d::new_default(input.map.size, Cell::Empty);

    for pos in iter_pipe_loop(input) {
        *loop_map.get_mut(pos).unwrap() = input.map.get(pos).unwrap();
    }

    // Now count up in scanlines
    let mut count = 0;
    for y in 0..loop_map.size.y {
        let line = loop_map.get_row(y);
        let mut is_in = false;
        for cell in line.iter() {
            match cell {
                Cell::Empty if is_in => count += 1,
                cell if cell.connects(Dir::Down) => is_in = !is_in,
                _ => (),
            }
        }
    }

    count
}
