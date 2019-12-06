use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn to_delta(&self) -> (i32, i32) {
        match self {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        }
    }
}

struct Movement {
    dir: Direction,
    dist: i32,
}

impl TryFrom<&str> for Movement {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (dir, dist) = input.split_at(1);

        let dir = match dir {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err("failed to parse direction"),
        };
        let dist = dist.parse().map_err(|_e| "failed to parse integer")?;

        Ok(Movement { dir, dist })
    }
}

fn parse_movements(line: &str) -> Result<Vec<Movement>, &'static str> {
    let mut res = Vec::new();
    for move_str in line.split(',') {
        let entry = Movement::try_from(move_str)?;
        res.push(entry);
    }

    Ok(res)
}

fn parse_wires(input: &str) -> Result<(Vec<Movement>, Vec<Movement>), &'static str> {
    let mut lines = input.lines();
    let wire1 = parse_movements(lines.next().ok_or("failed to get first line")?)?;
    let wire2 = parse_movements(lines.next().ok_or("failed to get second line")?)?;
    Ok((wire1, wire2))
}

fn find_closest_intersection_dist(wire1: Vec<Movement>, wire2: Vec<Movement>) -> Option<i32> {
    let mut visited_positions = HashSet::new();
    visited_positions.insert((0, 0));

    let mut x = 0;
    let mut y = 0;
    for movement in wire1 {
        let (dx, dy) = movement.dir.to_delta();
        for _ in 0..movement.dist {
            x += dx;
            y += dy;
            visited_positions.insert((x, y));
        }
    }

    let mut min_intersection_distance = None;

    x = 0;
    y = 0;
    for movement in wire2 {
        let (dx, dy) = movement.dir.to_delta();
        for _ in 0..movement.dist {
            x += dx;
            y += dy;
            if visited_positions.contains(&(x, y)) {
                let new_dist = x.abs() + y.abs(); // manhattan distance from (0, 0)
                min_intersection_distance = Some(match min_intersection_distance {
                    None => new_dist,
                    Some(old_dist) => min(old_dist, new_dist),
                });
            }
        }
    }

    min_intersection_distance
}

fn part1(input: &str) {
    let (wire1, wire2) = parse_wires(input).expect("failed to parse wires");
    let dist = find_closest_intersection_dist(wire1, wire2).expect("failed to find intersection");
    println!("Part 1 distance: {}", dist);
}

fn find_min_signal_dist_on_intersection(wire1: Vec<Movement>, wire2: Vec<Movement>) -> Option<i32> {
    // map of position to signal distance
    let mut visited_positions = HashMap::new();
    visited_positions.insert((0, 0), 0);

    let mut x = 0;
    let mut y = 0;
    let mut signal_dist = 0;
    for movement in wire1 {
        let (dx, dy) = movement.dir.to_delta();
        for _ in 0..movement.dist {
            x += dx;
            y += dy;
            signal_dist += 1;
            visited_positions.entry((x, y)).or_insert(signal_dist);
        }
    }

    let mut min_intersection_signal_dist = None;

    x = 0;
    y = 0;
    signal_dist = 0;
    for movement in wire2 {
        let (dx, dy) = movement.dir.to_delta();
        for _ in 0..movement.dist {
            x += dx;
            y += dy;
            signal_dist += 1;
            if let Some(dist_1) = visited_positions.get(&(x, y)) {
                let new_dist = dist_1 + signal_dist;
                min_intersection_signal_dist = Some(match min_intersection_signal_dist {
                    None => new_dist,
                    Some(old_dist) => min(old_dist, new_dist),
                });
            }
        }
    }

    min_intersection_signal_dist
}

fn part2(input: &str) {
    let (wire1, wire2) = parse_wires(input).expect("failed to parse wires");
    let dist =
        find_min_signal_dist_on_intersection(wire1, wire2).expect("failed to find intersection");
    println!("Part 2 distance: {}", dist);
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
