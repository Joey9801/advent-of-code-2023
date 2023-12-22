use super::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn to_vec2(self) -> Vec2 {
        match self {
            Dir::Up => Vec2 { x: 0, y: -1 },
            Dir::Down => Vec2 { x: 0, y: 1 },
            Dir::Left => Vec2 { x: -1, y: 0 },
            Dir::Right => Vec2 { x: 1, y: 0 },
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn rotate_left(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    pub fn rotate_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
}

impl std::ops::Add<Dir> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Dir) -> Self::Output {
        self + rhs.to_vec2()
    }
}
