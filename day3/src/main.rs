use std::collections::HashSet;
use std::convert::{TryFrom};

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

fn manhattan_dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn find_closest_intersection(input: &str) -> Result<(i32, i32), &'static str> {
    let mut lines = input.lines();
    let wire1 = parse_movements(lines.next().ok_or("failed to get first line")?)?;
    let wire2 = parse_movements(lines.next().ok_or("failed to get second line")?)?;

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

    let mut min_intersection = None;
    let mut min_intersection_distance = None;

    x = 0;
    y = 0;
    for movement in wire2 {
        let (dx, dy) = movement.dir.to_delta();
        for _ in 0..movement.dist {
            x += dx;
            y += dy;
            if visited_positions.contains(&(x, y)) {
                let new_dist = manhattan_dist((x, y), (0, 0));
                let is_new_min_intersection = match min_intersection_distance {
                    None => true,
                    Some(old_dist) => new_dist < old_dist
                };

                if is_new_min_intersection {
                    min_intersection = Some((x, y));
                    min_intersection_distance = Some(new_dist);
                }
            }
        }
    }

    match min_intersection {
        None => Err("failed to find intersection"),
        Some(pos) => Ok(pos)
    }
}

fn part1(input: &str) {
    let pos = find_closest_intersection(input)
        .expect("failed to find closest intersection");
    let dist = manhattan_dist(pos, (0, 0));
    println!("Part 1 result: {}", dist);
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
}
