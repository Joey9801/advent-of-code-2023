use crate::util::pairs;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

pub fn parse(input: &str) -> Vec<Hailstone> {
    // Input lines like:
    // px, py, pz @ vx, vy, vz
    let parse_vec3 = |s: &str| {
        let mut s = s.split(", ").map(|x| x.trim().parse().unwrap());
        Vec3 {
            x: s.next().unwrap(),
            y: s.next().unwrap(),
            z: s.next().unwrap(),
        }
    };

    let mut stones = Vec::new();
    for line in input.lines() {
        let (pos, vel) = line.split_once(" @ ").unwrap();
        stones.push(Hailstone {
            pos: parse_vec3(pos),
            vel: parse_vec3(vel),
        });
    }

    stones
}

fn intersects_xy(a: &Hailstone, b: &Hailstone, range_min: f64, range_max: f64) -> bool {
    // The line trace by a hailstone in the xy plane is: (px + vx * t, py + vy * t)
    // The gradient of the line is vy / vx, and the intercept is py - px * vy / vx
    // => y = (vy / vx) * x + (py - px * vy / vx)

    assert!(a.vel.x != 0f64 || a.vel.y != 0f64);

    let grad = |s: &Hailstone| s.vel.y / s.vel.x;
    let intercept = |s: &Hailstone| s.pos.y as f64 - s.pos.x as f64 * grad(s);

    let a_grad = grad(&a);
    let a_intercept = intercept(&a);
    let b_grad = grad(&b);
    let b_intercept = intercept(&b);

    // If the lines are parallel, return false as they don't have a single well
    // defined point of intersection
    if a_grad == b_grad {
        return false;
    }

    // Find the point of intersection of the two lines:
    // a_grad * x + a_intercept = b_grad * x + b_intercept

    let x = (b_intercept - a_intercept) / (a_grad - b_grad);
    let y = a_grad * x + a_intercept;

    // Check that the point of intersection happens with a non-negative 't' for each line
    if (x - a.pos.x as f64).signum() != (a.vel.x as f64).signum() {
        return false;
    }
    if (x - b.pos.x as f64).signum() != (b.vel.x as f64).signum() {
        return false;
    }

    x >= range_min && x <= range_max && y >= range_min && y <= range_max
}

pub fn solve_part_1(input: &[Hailstone]) -> usize {
    let (range_min, range_max) = (200_000_000_000_000f64, 400_000_000_000_000f64);
    // let (range_min, range_max) = (7f64, 27f64);

    pairs(input)
        .filter(|(a, b)| intersects_xy(a, b, range_min, range_max))
        .count()
}

pub fn solve_part_2(input: &[Hailstone]) -> i64 {
    // Each stone follows the path `p_i + v_i*t` in 3d space Need to find a new
    // line, `a + b*t` that intersects every stone at some point in time Ie for
    // each stone i:
    // - `a + b*t = p_i + v_i*t`
    // - => `(p_i - a) + (v_i - b)*t = 0`
    // - => ie the constraint defines another line in 3d space
    //
    // The problem is massively overdetermined - it should only require three
    // linearly independent stones to find a unique solution, and the input has
    // 300
    //
    // Two lines intersect if they are coplanar and they aren't parallel. Ie:
    // - `v_1 x v_2 != 0` (They are not parallel)
    // - (p_1 - p_2) . (v_1 x v_2) = 0` (They are coplanar)
    //
    // If we plug in the lines that defines the constraints for stones i and j:
    // - `((p_i - a) - (p_j - a)) . ((v_i - b) x (v_j - b)) = 0`
    // - => `(p_i - p_j) . ((v_i - b) x (v_j - b)) = 0`
    //
    // - (v_i - b) x (v_j - b) = (v_i x v_j) - (v_i - v_j) x b
    // - (dot both sides by (p_i - p_j)) => `0 = (p_i - p_j) . (v_i x v_j) - (p_i - p_j) . ((v_i - v_j) x b) = 0`
    // - (scalar triple product) => `0 = (p_i - p_j) . (v_i x v_j) - b . ((p_i - p_j) x (v_i - v_j))`
    // - let `c_ij = (p_i - p_j) x (v_i - v_j)`
    // - let `d_ij = (p_i - p_j) . (v_i x v_j)`
    // - => `b . c_ij = d_ij`
    // - => Ie the constraints based on the pair of stones i, j defines a plane in 3d space
    //
    // We take any three (linearly independent) pairs of stones, say (1, 2), (2,
    // 3), and (1, 3), and find the point the three planes intersect

    let plane = |h1: &Hailstone, h2: &Hailstone| {
        let c_12 = (h1.pos - h2.pos).cross(h1.vel - h2.vel);
        let d_12 = (h1.pos - h2.pos).dot(h1.vel.cross(h2.vel));
        (c_12, d_12)
    };

    // The three planes that define the constraints
    let (c_12, d_12) = plane(&input[0], &input[1]);
    let (c_13, d_13) = plane(&input[0], &input[2]);
    let (c_23, d_23) = plane(&input[1], &input[2]);

    // The point of intersection of the three planes
    let mut b = (c_13.cross(c_23) * d_12) + (c_23.cross(c_12) * d_13) + (c_12.cross(c_13) * d_23);
    let t = c_12.dot(c_13.cross(c_23));
    b.x = b.x / t;
    b.y = b.y / t;
    b.z = b.z / t;

    // Round away any floating point precision errors
    debug_assert!((b.x - (b.x.round())).abs() < 1e-6);
    debug_assert!((b.y - (b.y.round())).abs() < 1e-6);
    debug_assert!((b.z - (b.z.round())).abs() < 1e-6);
    b.x = b.x.round();
    b.y = b.y.round();
    b.z = b.z.round();

    dbg!(b);

    // Now we hae the velocity term, we can work backwards to find the position at t=0

    let b1 = input[0].vel - b;
    let b2 = input[1].vel - b;
    let bb = b1.cross(b2);

    let e = bb.dot(input[1].pos.cross(b2));
    let f = bb.dot(input[0].pos.cross(b1));
    let g = input[0].pos.dot(bb);
    let s = bb.dot(bb);

    let mut a = b1 * e - b2 * f + bb * g;
    a.x = (a.x / s).round();
    a.y = (a.y / s).round();
    a.z = (a.z / s).round();

    dbg!(a);

    (a.x + a.y + a.z) as i64
}
