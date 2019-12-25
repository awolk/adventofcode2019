use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

struct Layout {
    grid: [bool; 25],
}

impl FromStr for Layout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [false; 25];
        for (row, line) in s.lines().enumerate() {
            for (col, byte) in line.bytes().enumerate() {
                if row > 4 || col > 4 || (byte != b'#' && byte != b'.') {
                    return Err(());
                }

                grid[row * 5 + col] = byte == b'#';
            }
        }
        Ok(Layout { grid })
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in 0..5 {
            for col in 0..5 {
                write!(f, "{}", if self.grid[row * 5 + col] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Layout {
    fn biodiversity(&self) -> u32 {
        self.grid.iter().enumerate().fold(0, |acc, (index, &val)| {
            acc + if val { 1 << index as u32 } else { 0 }
        })
    }

    fn next(&self) -> Self {
        let mut grid = self.grid;
        for row in 0..5i32 {
            for col in 0..5i32 {
                let neighbors = [
                    (row + 1, col),
                    (row - 1, col),
                    (row, col + 1),
                    (row, col - 1),
                ];
                let adjacent_bugs: u8 = neighbors
                    .iter()
                    .map(|&(row, col)| {
                        let index = row * 5 + col;
                        if row < 0 || row >= 5 || col < 0 || col >= 5 {
                            0
                        } else {
                            self.grid[index as usize] as u8
                        }
                    })
                    .sum();

                let index = row as usize * 5 + col as usize;
                if self.grid[index] {
                    if adjacent_bugs != 1 {
                        grid[index] = false;
                    }
                } else if adjacent_bugs == 1 || adjacent_bugs == 2 {
                    grid[index] = true;
                }
            }
        }

        Layout { grid }
    }
}

fn part1(mut layout: Layout) -> u32 {
    let mut seen = HashSet::new();
    loop {
        let biodiversity = layout.biodiversity();
        if !seen.insert(biodiversity) {
            return biodiversity;
        }
        layout = layout.next();
    }
}

fn main() {
    let input = include_str!("input.txt");
    let layout: Layout = input.parse().unwrap();
    let biodiversity = part1(layout);
    println!("Part 1: biodiversity = {}", biodiversity);
}
