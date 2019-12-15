mod emulator;

use crate::Status::{HitWall, Moved, MovedAndFinished};
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
            0 => HitWall,
            1 => Moved,
            2 => MovedAndFinished,
            _ => return Err("invalid status code"),
        })
    }
}

trait Robot {
    type Error;
    fn send_move(&mut self, movement: Movement) -> Result<Status, Self::Error>;
}

// maze solver
fn do_moves<T: Robot>(robot: &mut T, moves: &[Movement]) -> Result<(i64, i64), &'static str> {
    let mut x = 0;
    let mut y = 0;
    for m in moves {
        match m {
            Movement::North => y += 1,
            Movement::South => y -= 1,
            Movement::West => x -= 1,
            Movement::East => x += 1,
        }

        let status = robot
            .send_move(*m)
            .map_err(|_| "move failed due to robot error")?;
        if status != Status::Moved {
            return Err("non-standard move");
        }
    }

    Ok((x, y))
}

fn reverse_moves<T: Robot>(robot: &mut T, moves: &[Movement]) -> Result<(), &'static str> {
    for m in moves.iter().rev() {
        let status = robot
            .send_move(m.reverse())
            .map_err(|_| "move failed due to robot error")?;
        if status != Status::Moved {
            return Err("non-standard move");
        }
    }

    Ok(())
}

fn solve_maze<T: Robot>(robot: &mut T) -> Result<usize, &'static str> {
    // bfs
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    let mut queue: VecDeque<Vec<Movement>> = VecDeque::new();
    queue.push_back(Vec::new());

    loop {
        let moves = queue.pop_front().unwrap();
        let pos = do_moves(robot, &moves)?;

        let is_new = seen.insert(pos);
        if !is_new {
            reverse_moves(robot, &moves)?;
            continue;
        }

        for attempt in DIRECTIONS.iter() {
            // don't just undo the last move
            if moves.len() > 0 && *attempt == moves.last().unwrap().reverse() {
                continue;
            }

            let status = robot
                .send_move(*attempt)
                .map_err(|_| "move failed due to robot error")?;

            match status {
                Status::Moved => {
                    let mut new_moves = moves.clone();
                    new_moves.push(*attempt);
                    queue.push_back(new_moves);

                    robot
                        .send_move(attempt.reverse())
                        .map_err(|_| "move failed due to robot error")?;
                }
                Status::HitWall => {}
                Status::MovedAndFinished => {
                    return Ok(moves.len() + 1);
                }
            }
        }
        reverse_moves(robot, &moves)?;
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
    let steps = solve_maze(&mut robot).expect("failed to solve maze");
    println!("Part 1: steps to solve maze = {}", steps);
}
