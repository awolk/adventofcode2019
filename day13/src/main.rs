use std::collections::HashMap;
use std::iter;

mod emulator;

struct Screen {
    grid: HashMap<(i64, i64), i64>,
}

impl Screen {
    fn from_output(mut output: impl Iterator<Item = i64>) -> Result<Self, &'static str> {
        let mut grid = HashMap::new();

        while let Some(x) = output.next() {
            let y = output.next().ok_or("invalid output")?;
            let tile = output.next().ok_or("invalid output")?;
            grid.insert((x, y), tile);
        }

        Ok(Screen { grid })
    }

    fn count(&self, tile_type: i64) -> usize {
        self.grid.values().filter(|t| **t == tile_type).count()
    }
}

fn main() {
    let input = include_str!("input.txt");
    let program = emulator::Program::new(input).expect("failed to parse program");
    let output = emulator::Emulator::run_program_with_input(program, iter::empty())
        .expect("failed to run program");
    let screen = Screen::from_output(output.into_iter()).expect("failed to parse screen");

    let num_block_tile = screen.count(2);
    println!("Part 1: number of block tiles = {}", num_block_tile);
}
