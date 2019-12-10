use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::f64::consts::{FRAC_PI_2, PI};
use std::str::FromStr;

#[derive(Eq, Clone)]
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
                    let angle = (ay as f64 - y as f64).atan2(x as f64 - ax as f64);
                    let comparable_angle = (angle * 1_000_000.).round() as i64;
                    angles.insert(comparable_angle);
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
        let mut angles: BTreeMap<i64, BTreeSet<PosWithDist>> = BTreeMap::new();
        for y in 0..height {
            for x in 0..width {
                if (x, y) != (ax, ay) && self.map[y][x] {
                    let dy = ay as f64 - y as f64;
                    let dx = x as f64 - ax as f64;

                    let mut angle = -(dy.atan2(dx) - FRAC_PI_2);
                    if angle < 0. {
                        angle += 2. * PI;
                    }
                    let comparable_angle = (angle * 1_000_000.).round() as i64;

                    let dist = dy.powi(2) + dx.powi(2);
                    let comparable_dist = (dist * 1_000_000.).round() as usize;

                    angles
                        .entry(comparable_angle)
                        .or_insert_with(BTreeSet::new)
                        .insert(PosWithDist {
                            x,
                            y,
                            dist: comparable_dist,
                        });
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
                    to_remove.push(*angle);
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
