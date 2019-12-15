mod emulator;

use std::cmp::Ordering;
use std::collections::HashMap;

struct Screen {
    grid: HashMap<(i64, i64), i64>,
    score: i64,

    ball_pos: (i64, i64),
    paddle_pos: (i64, i64),
}

impl Screen {
    fn new() -> Self {
        Screen {
            grid: HashMap::new(),
            score: 0,
            ball_pos: (0, 0),
            paddle_pos: (0, 0),
        }
    }

    fn process_triple(&mut self, x: i64, y: i64, val: i64) {
        if x == -1 && y == 0 {
            self.score = val;
        } else {
            self.grid.insert((x, y), val);
            if val == 4 {
                self.ball_pos = (x, y);
            } else if val == 3 {
                self.paddle_pos = (x, y);
            }
        }
    }

    fn process_output(
        &mut self,
        mut output: impl Iterator<Item = i64>,
    ) -> Result<(), &'static str> {
        while let Some(x) = output.next() {
            let y = output.next().ok_or("invalid output")?;
            let val = output.next().ok_or("invalid output")?;
            self.process_triple(x, y, val);
        }

        Ok(())
    }

    fn count(&self, tile_type: i64) -> usize {
        self.grid.values().filter(|t| **t == tile_type).count()
    }
}

fn part1(program: emulator::Program) {
    let mut emu = emulator::Emulator::new(program);
    let mut output_buffer = Vec::new();
    loop {
        let halted = emu
            .step(
                || Err("no input"),
                |out| {
                    output_buffer.push(out);
                    Ok(())
                },
            )
            .expect("emulator failed");
        if halted {
            break;
        }
    }

    let mut screen = Screen::new();
    screen
        .process_output(output_buffer.into_iter())
        .expect("failed to parse screen");

    let num_block_tile = screen.count(2);
    println!("Part 1: number of block tiles = {}", num_block_tile);
}

fn part2(program: emulator::Program) {
    let mut screen = Screen::new();
    let mut emu = emulator::Emulator::new(program);
    emu.store(0, 2);

    let mut output_buffer = Vec::with_capacity(3);

    loop {
        let paddle_cmp_ball = screen.paddle_pos.0.cmp(&screen.ball_pos.0);

        let halted = emu
            .step(
                || {
                    Ok(match paddle_cmp_ball {
                        Ordering::Equal => 0,
                        Ordering::Less => 1,
                        Ordering::Greater => -1,
                    })
                },
                |output| {
                    output_buffer.push(output);
                    if output_buffer.len() == 3 {
                        screen.process_triple(output_buffer[0], output_buffer[1], output_buffer[2]);
                        output_buffer.clear();
                    }
                    Ok(())
                },
            )
            .expect("emulator failed");

        if halted {
            break;
        };
    }

    println!("Part 2: score = {}", screen.score);
}

fn main() {
    let input = include_str!("input.txt");
    let program = emulator::Program::new(input).expect("failed to parse program");

    part1(program.clone());
    part2(program);
}
