use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::f64::consts::{FRAC_PI_2, PI};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

//// Helpers:

fn gcd(a: u64, b: u64) -> u64 {
    // Euclid's algorithm
    if b == 0 {
        a
    } else {
        gcd(b, a - b * (a / b))
    }
}

// RationalAngle is a hashable, comparable angle derived from a vector
#[derive(Clone)]
struct RationalAngle {
    x: i64,
    y: i64,
    angle: f64,
}

impl RationalAngle {
    fn new(dx: i64, dy: i64) -> RationalAngle {
        let gcd = gcd(dx.abs() as u64, dy.abs() as u64) as i64;
        let x = dx / gcd;
        let y = dy / gcd;

        // order angle so straight up is first, going in order clockwise
        // invert y because in in our system +y is down
        let mut angle = FRAC_PI_2 - (-y as f64).atan2(x as f64);
        if angle < 0. {
            angle += 2. * PI;
        }
        RationalAngle { x, y, angle }
    }
}

impl PartialEq for RationalAngle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for RationalAngle {}

impl Hash for RationalAngle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Ord for RationalAngle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.angle.partial_cmp(&other.angle).unwrap()
    }
}

impl PartialOrd for RationalAngle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// PosWithDist is a point struct sorted by an included distance value
#[derive(Clone)]
struct PosWithDist {
    x: usize,
    y: usize,
    dist: usize,
}

impl Ord for PosWithDist {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for PosWithDist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PosWithDist {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for PosWithDist {}

//// Implementation:

struct Grid {
    map: Vec<Vec<bool>>,
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|byte| match byte {
                        b'.' => Ok(false),
                        b'#' => Ok(true),
                        _ => Err("invalid map entry"),
                    })
                    .collect::<Result<Vec<bool>, &'static str>>()
            })
            .collect::<Result<Vec<Vec<bool>>, &'static str>>()?;

        if map.is_empty() || map[0].is_empty() {
            return Err("invalid map size");
        }

        Ok(Grid { map })
    }
}

impl Grid {
    fn asteroids_visible_from(&self, ax: usize, ay: usize) -> usize {
        let height = self.map.len();
        let width = self.map[0].len();

        // we can see at most one asteroid per angle
        let mut angles = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                if (x, y) != (ax, ay) && self.map[y][x] {
                    let dx = x as i64 - ax as i64;
                    let dy = y as i64 - ay as i64;
                    let angle = RationalAngle::new(dx, dy);
                    angles.insert(angle);
                }
            }
        }

        angles.len()
    }

    fn most_asteroids_visible(&self) -> ((usize, usize), usize) {
        let height = self.map.len();
        let width = self.map[0].len();

        (0..height)
            .flat_map(|y| (0..width).map(move |x| ((x, y), self.asteroids_visible_from(x, y))))
            .max_by_key(|&(_pos, visible)| visible)
            .unwrap()
    }

    fn asteroids_destroyed_order(&self, ax: usize, ay: usize) -> Vec<(usize, usize)> {
        let height = self.map.len();
        let width = self.map[0].len();

        // we can see at most one asteroid per angle
        let mut angles: BTreeMap<RationalAngle, BTreeSet<PosWithDist>> = BTreeMap::new();
        for y in 0..height {
            for x in 0..width {
                if (x, y) != (ax, ay) && self.map[y][x] {
                    let dx = x as i64 - ax as i64;
                    let dy = y as i64 - ay as i64;
                    let angle = RationalAngle::new(dx, dy);
                    let dist = (dy.pow(2) + dx.pow(2)) as usize;

                    angles
                        .entry(angle)
                        .or_insert_with(BTreeSet::new)
                        .insert(PosWithDist { x, y, dist });
                }
            }
        }

        let mut res = Vec::new();
        while !angles.is_empty() {
            let mut to_remove = Vec::new();
            for (angle, positions) in &mut angles {
                let first = positions.iter().next().unwrap().clone();
                positions.remove(&first);
                if positions.is_empty() {
                    to_remove.push(angle.clone());
                }

                res.push((first.x, first.y));
            }

            for angle in to_remove {
                angles.remove(&angle);
            }
        }

        res
    }
}

fn main() {
    let input = include_str!("input.txt");
    let grid: Grid = input.parse().expect("failed to parse input");

    let ((x, y), most_visible) = grid.most_asteroids_visible();
    println!("Part 1: result = {} at ({}, {})", most_visible, x, y);

    let ad = grid.asteroids_destroyed_order(x, y);
    let (x, y) = ad[199];
    println!("Part 2: result = {}", x * 100 + y);
}
