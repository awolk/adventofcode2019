use crate::emulator::Emulator;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::iter;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

mod emulator;

mod terminal {
    const CSI: &str = "\x1b[";

    fn clear_screen() {
        print!("{}2J", CSI);
    }

    fn reset_cursor() {
        print!("{};H", CSI);
    }

    pub fn reset() {
        clear_screen();
        reset_cursor();
    }
}

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

    fn dim(&self) -> (i64, i64) {
        let x_max = self.grid.keys().map(|&(x, _y)| x).max().unwrap_or(0);
        let y_max = self.grid.keys().map(|&(_x, y)| y).max().unwrap_or(0);
        (x_max, y_max)
    }

    fn display(&self) {
        terminal::reset();
        println!("Score: {}", self.score);
        let (cols, rows) = self.dim();
        for row in 0..=rows {
            for col in 0..=cols {
                print!(
                    "{}",
                    match self.grid.get(&(col, row)) {
                        Some(0) => ' ',
                        Some(1) => '#',
                        Some(2) => '%',
                        Some(3) => '-',
                        Some(4) => 'o',
                        _ => ' ',
                    }
                );
            }
            println!();
        }
    }
}

fn part1(program: emulator::Program) {
    let mut emu = emulator::Emulator::new(program);
    let (tx_out, rx_out) = sync_channel(0);
    let run_handle: JoinHandle<Result<(), &'static str>> = thread::spawn(move || loop {
        let halted = emu.step(
            || Err("no input"),
            |out| tx_out.send(out).map_err(|_| "write failed"),
        )?;
        if halted {
            return Ok(());
        }
    });

    let mut screen = Screen::new();
    screen
        .process_output(rx_out.into_iter())
        .expect("failed to parse screen");

    run_handle
        .join()
        .expect("failed to join thread")
        .expect("emulator failed");

    let num_block_tile = screen.count(2);
    println!("Part 1: number of block tiles = {}", num_block_tile);
}

fn part2(program: emulator::Program) {
    let mut screen = Screen::new();
    let mut emu = Emulator::new(program);
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
                        screen.display();
                    }
                    Ok(())
                },
            )
            .expect("emulator failed");

        if halted {
            break;
        };
    }
}

fn main() {
    let input = include_str!("input.txt");
    let program = emulator::Program::new(input).expect("failed to parse program");
    part1(program.clone());

    println!("Press enter to start part 2");
    io::stdin()
        .read_line(&mut String::new())
        .expect("read failed");
    part2(program);
}
