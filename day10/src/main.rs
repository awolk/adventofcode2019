use std::collections::HashSet;
use std::str::FromStr;

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

        if map.len() == 0 || map[0].len() == 0 {
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
                    let angle = (ay as f64 - y as f64).atan2(ax as f64 - x as f64);
                    let comparable_angle = (angle * 1000.).round() as i32;
                    angles.insert(comparable_angle);
                }
            }
        }

        angles.len()
    }

    fn most_asteroids_visible(&self) -> usize {
        let height = self.map.len();
        let width = self.map[0].len();

        (0..height)
            .flat_map(|y| (0..width).map(move |x| self.asteroids_visible_from(x, y)))
            .max()
            .unwrap()
    }
}

fn main() {
    let input = include_str!("input.txt");
    let grid: Grid = input.parse().expect("failed to parse input");
    println!("Part 1: result = {}", grid.most_asteroids_visible());
}
