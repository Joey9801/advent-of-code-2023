#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn l1_norm(self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl std::ops::Mul<i64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<Self> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        *self + *rhs
    }
}

impl std::ops::AddAssign<Self> for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<Self> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}

impl std::ops::SubAssign<Self> for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
}

impl std::ops::Add<Dir> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Dir) -> Self::Output {
        self + rhs.to_vec2()
    }
}

pub fn gcm(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcm(a, b)
}

pub fn lcm_iter<I>(mut iter: I) -> i64
where
    I: Iterator<Item = i64>,
{
    let mut result = iter.next().unwrap();
    for i in iter {
        result = lcm(result, i);
    }
    result
}

/// Return the number of ways to choose k items from n items without repetition
/// and without order.
pub fn binomial_coefficient(n: i64, k: i64) -> i64 {
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}

pub fn pair_indices(len: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..len).flat_map(move |i| (i + 1..len).map(move |j| (i, j)))
}

pub fn pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    pair_indices(slice.len()).map(move |(i, j)| (&slice[i], &slice[j]))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(super::binomial_coefficient(5, 3), 10);
        assert_eq!(super::binomial_coefficient(5, 2), 10);
        assert_eq!(super::binomial_coefficient(5, 1), 5);
        assert_eq!(super::binomial_coefficient(5, 0), 1);
    }
}
