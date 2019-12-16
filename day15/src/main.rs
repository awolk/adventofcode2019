mod emulator;

use std::cmp::max;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};

// interface for robot in maze

#[derive(Eq, PartialEq, Copy, Clone)]
enum Movement {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Movement {
    fn reverse(&self) -> Self {
        match self {
            Movement::North => Movement::South,
            Movement::South => Movement::North,
            Movement::West => Movement::East,
            Movement::East => Movement::West,
        }
    }

    fn to_delta(&self) -> (i64, i64) {
        match self {
            Movement::North => (0, 1),
            Movement::South => (0, -1),
            Movement::West => (-1, 0),
            Movement::East => (1, 0),
        }
    }
}

const DIRECTIONS: [Movement; 4] = [
    Movement::North,
    Movement::South,
    Movement::West,
    Movement::East,
];

#[derive(Eq, PartialEq)]
enum Status {
    HitWall = 0,
    Moved = 1,
    MovedAndFinished = 2,
}

impl TryFrom<i64> for Status {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Status::HitWall,
            1 => Status::Moved,
            2 => Status::MovedAndFinished,
            _ => return Err("invalid status code"),
        })
    }
}

trait Robot {
    type Error;
    fn send_move(&mut self, movement: Movement) -> Result<Status, Self::Error>;
}

// maze exploration

enum Tile {
    Empty,
    Wall,
    Oxygen,
}

struct Maze(HashMap<(i64, i64), Tile>);

fn explore_map<T: Robot>(
    robot: &mut T,
    pos: (i64, i64),
    maze: &mut Maze,
) -> Result<(), &'static str> {
    // depth first search
    for dir in DIRECTIONS.iter() {
        let (dx, dy) = dir.to_delta();
        let new_pos = (pos.0 + dx, pos.1 + dy);
        let entry = maze.0.entry(new_pos);
        if let Entry::Vacant(ve) = entry {
            // unexplored position
            let status = robot.send_move(*dir).map_err(|_| "robot failed")?;
            match status {
                Status::HitWall => {
                    ve.insert(Tile::Wall);
                }
                Status::Moved => {
                    ve.insert(Tile::Empty);
                    explore_map(robot, new_pos, maze);
                    robot.send_move(dir.reverse()).map_err(|_| "robot failed")?;
                }
                Status::MovedAndFinished => {
                    ve.insert(Tile::Oxygen);
                    explore_map(robot, new_pos, maze);
                    robot.send_move(dir.reverse()).map_err(|_| "robot failed")?;
                }
            }
        }
    }

    Ok(())
}

// maze solving

impl Maze {
    fn new() -> Self {
        Maze(HashMap::new())
    }

    // returns (pos, steps)
    fn find_oxygen(&self) -> ((i64, i64), usize) {
        // breadth first search
        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let mut queue: VecDeque<((i64, i64), usize)> = VecDeque::new();
        queue.push_back(((0, 0), 0));

        loop {
            let ((x, y), steps) = queue.pop_front().unwrap();

            let is_new = seen.insert((x, y));
            if !is_new {
                continue;
            }

            match self.0.get(&(x, y)).unwrap() {
                Tile::Oxygen => return ((x, y), steps),
                Tile::Wall => continue,
                Tile::Empty => {}
            }

            for dir in DIRECTIONS.iter() {
                let (dx, dy) = dir.to_delta();
                queue.push_back(((x + dx, y + dy), steps + 1));
            }
        }
    }

    fn time_to_fill_oxygen_from(&self, pos: (i64, i64)) -> usize {
        // exhaustive breadth first search from oxygen source
        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let mut queue: VecDeque<((i64, i64), usize)> = VecDeque::new();
        queue.push_back((pos, 0));

        let mut max_depth = 0;

        while !queue.is_empty() {
            let ((x, y), steps) = queue.pop_front().unwrap();

            let is_new = seen.insert((x, y));
            if !is_new {
                continue;
            }

            if let Tile::Wall = self.0.get(&(x, y)).unwrap() {
                continue;
            }

            max_depth = max(max_depth, steps);

            for dir in DIRECTIONS.iter() {
                let (dx, dy) = dir.to_delta();
                queue.push_back(((x + dx, y + dy), steps + 1));
            }
        }

        max_depth
    }
}

// intcode robot

struct IntcodeRobot {
    emu: emulator::Emulator,
}

impl IntcodeRobot {
    fn new(program: emulator::Program) -> Self {
        Self {
            emu: emulator::Emulator::new(program),
        }
    }
}

impl Robot for IntcodeRobot {
    type Error = &'static str;

    fn send_move(&mut self, movement: Movement) -> Result<Status, Self::Error> {
        let mut result = None;

        let input = movement as i64;
        while result.is_none() {
            self.emu.step(
                || Ok(input),
                |output| {
                    result = Some(output);
                    Ok(())
                },
            )?;
        }

        Ok(result.unwrap().try_into()?)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let program = emulator::Program::new(input).expect("failed to parse program");

    let mut robot = IntcodeRobot::new(program);
    let mut maze = Maze::new();

    explore_map(&mut robot, (0, 0), &mut maze).expect("failed to explore maze");
    let (oxygen_pos, length) = maze.find_oxygen();
    println!("Part 1: steps to solve maze = {}", length);

    let time_to_fill_oxygen = maze.time_to_fill_oxygen_from(oxygen_pos);
    println!("Part 2: time to fill oxygen = {}", time_to_fill_oxygen);
}
