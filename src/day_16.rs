use crate::util::{Dir, Map2d, Map2dExt, Vec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    /// The empty void beyond the input map, where beams go to die
    Void,

    /// An empty tile that beams pass straight through
    Empty,

    /// A mirror like /, that reflects a beam from the left upwards, and from
    /// the right downwards
    MirrorLeft,

    /// A mirror like \, that reflects a beam from the left downwards, and from
    /// the right upwards
    MirrorRight,

    /// A splitter that spits a horizontal beam into two vertical beams, and
    /// leaves vertical beams untouched
    SplitterVertical,

    /// A splitter that spits a vertical beam into two horizontal beams, and
    /// leaves horizontal beams untouched
    SplitterHorizontal,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Void
    }
}

enum Propagation {
    /// No further propagation
    Terminate,

    /// The beam continues in the single given dir
    Single(Dir),

    /// The beam continues in the two given dirs
    Double(Dir, Dir),
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::MirrorLeft,
            '\\' => Tile::MirrorRight,
            '|' => Tile::SplitterVertical,
            '-' => Tile::SplitterHorizontal,
            _ => panic!("Invalid character"),
        }
    }

    fn propagate(&self, dir: Dir) -> Propagation {
        match self {
            Tile::Void => Propagation::Terminate,
            Tile::Empty => Propagation::Single(dir),
            //  a '/' mirror
            Tile::MirrorLeft => match dir {
                Dir::Up => Propagation::Single(Dir::Right),
                Dir::Right => Propagation::Single(Dir::Up),
                Dir::Down => Propagation::Single(Dir::Left),
                Dir::Left => Propagation::Single(Dir::Down),
            },
            // a '\' mirror
            Tile::MirrorRight => match dir {
                Dir::Up => Propagation::Single(Dir::Left),
                Dir::Right => Propagation::Single(Dir::Down),
                Dir::Down => Propagation::Single(Dir::Right),
                Dir::Left => Propagation::Single(Dir::Up),
            },
            Tile::SplitterVertical => match dir {
                Dir::Up | Dir::Down => Propagation::Single(dir),
                Dir::Left | Dir::Right => Propagation::Double(Dir::Up, Dir::Down),
            },
            Tile::SplitterHorizontal => match dir {
                Dir::Up | Dir::Down => Propagation::Double(Dir::Left, Dir::Right),
                Dir::Left | Dir::Right => Propagation::Single(dir),
            },
        }
    }
}

/// A bitset of directions
///
/// - lsb: up
/// - 2nd: right
/// - 3rd: down
/// - 4th: left
/// - upper 4 bits: unused
#[derive(Clone, Copy, Debug, PartialEq)]
struct DirSet(u8);

impl DirSet {
    fn new_empty() -> DirSet {
        DirSet(0)
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn insert(&mut self, dir: Dir) {
        self.0 |= 1 << dir as u8;
    }

    fn contains(&self, dir: Dir) -> bool {
        self.0 & (1 << dir as u8) != 0
    }
}

impl Default for DirSet {
    fn default() -> DirSet {
        DirSet::new_empty()
    }
}

pub fn parse(input: &str) -> Map2d<Tile> {
    Map2d::parse_grid(input, Tile::from_char)
}

fn count_energized(map: &Map2d<Tile>, source_pos: Vec2, source_dir: Dir) -> usize {
    // A second map that traces where the beams have been so far
    let mut beam_paths = Map2d::new_default(map.size, DirSet::new_empty());
    let mut stack = vec![(source_pos, source_dir)];

    while let Some((pos, dir)) = stack.pop() {
        if beam_paths.get(pos).unwrap_or_default().contains(dir) {
            continue;
        }

        beam_paths.get_mut(pos).map(|dir_set| dir_set.insert(dir));
        match map.get(pos).unwrap_or_default().propagate(dir) {
            Propagation::Terminate => (),
            Propagation::Single(dir) => stack.push((pos + dir, dir)),
            Propagation::Double(dir1, dir2) => {
                stack.push((pos + dir1, dir1));
                stack.push((pos + dir2, dir2));
            }
        }
    }

    beam_paths
        .data
        .iter()
        .filter(|dir_set| !dir_set.is_empty())
        .count()
}

pub fn solve_part_1(map: &Map2d<Tile>) -> usize {
    count_energized(map, Vec2::new(0, 0), Dir::Right)
}

pub fn solve_part_2(map: &Map2d<Tile>) -> usize {
    // Perhaps possible to do some fancy memoization, but brute forcing 440 edge
    // tile+dir tuples in the real input is fast enough

    let top = (0..map.size().x).map(|x| count_energized(map, Vec2::new(x, 0), Dir::Down));
    let left = (0..map.size().y).map(|y| count_energized(map, Vec2::new(0, y), Dir::Right));
    let bottom =
        (0..map.size().x).map(|x| count_energized(map, Vec2::new(x, map.size().y - 1), Dir::Up));
    let right =
        (0..map.size().y).map(|y| count_energized(map, Vec2::new(map.size().x - 1, y), Dir::Left));
    let all = top.chain(left).chain(bottom).chain(right);

    all.max().unwrap()
}
