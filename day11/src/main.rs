use crate::emulator::Program;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::sync::mpsc::sync_channel;
use std::thread;

mod emulator;

#[derive(Clone, Copy, Debug)]
enum Color {
    Black = 0,
    White = 1,
}

impl TryFrom<i64> for Color {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Color::Black,
            1 => Color::White,
            _ => return Err("invalid color"),
        })
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn counterclockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn move_along(&self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Direction::Up => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y - 1),
            Direction::Right => (x + 1, y),
        }
    }
}

struct Robot {
    painted: HashMap<(i64, i64), Color>,
    pos: (i64, i64),
    direction: Direction,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            painted: HashMap::new(),
            pos: (0, 0),
            direction: Direction::Up,
        }
    }

    fn run(&mut self, program: Program) -> Result<(), &'static str> {
        let (tx_in, rx_in) = sync_channel(0);
        let (tx_out, rx_out) = sync_channel(0);
        let mut emu = emulator::Emulator::new(program, rx_in, tx_out);
        let handle = thread::spawn(move || emu.run());

        loop {
            if tx_in
                .send(self.painted.get(&self.pos).copied().unwrap_or(Color::Black) as i64)
                .is_err()
            {
                return handle
                    .join()
                    .map_err(|_| "failed to join thread")
                    .and_then(|r| r);
            }

            let color: Color = rx_out.recv().map_err(|_| "failed to receive")?.try_into()?;

            let rotation = rx_out.recv().map_err(|_| "receive failed")?;
            self.painted.insert(self.pos, color);

            self.direction = match rotation {
                0 => self.direction.counterclockwise(),
                1 => self.direction.clockwise(),
                _ => return Err("invalid rotation"),
            };
            self.pos = self.direction.move_along(self.pos);
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let program = emulator::Program::new(input).expect("failed to parse input");
    let mut robot = Robot::new();
    robot.run(program).expect("failed running program");
    println!("Part 1: result = {}", robot.painted.len());
}
