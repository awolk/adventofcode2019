use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

macro_rules! ind {
    ($grid:expr, $r:expr, $c:expr) => {
        $grid[$r * 5 + $c]
    };
}

#[derive(Clone, Copy)]
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

                ind!(grid, row, col) = byte == b'#';
            }
        }
        Ok(Layout { grid })
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in 0..5 {
            for col in 0..5 {
                write!(f, "{}", if ind!(self.grid, row, col) { '#' } else { '.' })?;
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
                        if row < 0 || row >= 5 || col < 0 || col >= 5 {
                            0
                        } else {
                            ind!(self.grid, row as usize, col as usize) as u8
                        }
                    })
                    .sum();

                if ind!(self.grid, row as usize, col as usize) {
                    if adjacent_bugs != 1 {
                        ind!(grid, row as usize, col as usize) = false;
                    }
                } else if adjacent_bugs == 1 || adjacent_bugs == 2 {
                    ind!(grid, row as usize, col as usize) = true;
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

#[derive(Clone)]
struct RecursiveLayout {
    layouts: HashMap<i32, [bool; 25]>, // depth -> layout
    max_height: i32,
    min_height: i32,
}

impl From<Layout> for RecursiveLayout {
    fn from(layout: Layout) -> Self {
        let mut layouts = HashMap::new();
        layouts.insert(0, layout.grid);
        Self {
            layouts,
            max_height: 0,
            min_height: 0,
        }
    }
}

enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

fn get_side(grid: &[bool; 25], side: Side) -> Vec<bool> {
    match side {
        Side::Left => vec![
            ind!(grid, 0, 0),
            ind!(grid, 1, 0),
            ind!(grid, 2, 0),
            ind!(grid, 3, 0),
            ind!(grid, 4, 0),
        ],
        Side::Right => vec![
            ind!(grid, 0, 4),
            ind!(grid, 1, 4),
            ind!(grid, 2, 4),
            ind!(grid, 3, 4),
            ind!(grid, 4, 4),
        ],
        Side::Top => vec![
            ind!(grid, 0, 0),
            ind!(grid, 0, 1),
            ind!(grid, 0, 2),
            ind!(grid, 0, 3),
            ind!(grid, 0, 4),
        ],
        Side::Bottom => vec![
            ind!(grid, 4, 0),
            ind!(grid, 4, 1),
            ind!(grid, 4, 2),
            ind!(grid, 4, 3),
            ind!(grid, 4, 4),
        ],
    }
}

fn neighbors(
    grid: &[bool; 25],
    outer: &[bool; 25],
    inner: &[bool; 25],
    row: usize,
    col: usize,
) -> Vec<bool> {
    let mut neighbors = Vec::new();
    // get left neighbors
    neighbors.extend(match (row, col) {
        (_, 0) => vec![ind!(outer, 2, 1)],
        (2, 3) => get_side(inner, Side::Right),
        _ => vec![ind!(grid, row, col - 1)],
    });
    // get right neighbors
    neighbors.extend(match (row, col) {
        (_, 4) => vec![ind!(outer, 2, 3)],
        (2, 1) => get_side(inner, Side::Left),
        _ => vec![ind!(grid, row, (col + 1))],
    });
    // get upper neighbors
    neighbors.extend(match (row, col) {
        (0, _) => vec![ind!(outer, 1, 2)],
        (3, 2) => get_side(inner, Side::Bottom),
        _ => vec![ind!(grid, row - 1, col)],
    });
    // get lower neighbors
    neighbors.extend(match (row, col) {
        (4, _) => vec![ind!(outer, 3, 2)],
        (1, 2) => get_side(inner, Side::Top),
        _ => vec![ind!(grid, row + 1, col)],
    });

    neighbors
}

impl RecursiveLayout {
    fn count_bugs(&self) -> u32 {
        let mut res = 0;
        for height in self.min_height..=self.max_height {
            res += self.layouts[&height].iter().map(|&b| b as u32).sum::<u32>();
        }
        res
    }

    fn next(self) -> Self {
        let mut new = self.clone();
        // expand up 1 on each end, and chop it off if empty
        new.max_height += 1;
        new.layouts.insert(new.max_height, [false; 25]);
        new.min_height -= 1;
        new.layouts.insert(new.min_height, [false; 25]);

        for height in new.min_height..=new.max_height {
            let outer = self.layouts.get(&(height + 1)).unwrap_or(&[false; 25]);
            let inner = self.layouts.get(&(height - 1)).unwrap_or(&[false; 25]);
            let grid = self.layouts.get(&height).unwrap_or(&[false; 25]);
            let new_grid = new.layouts.get_mut(&height).unwrap();

            for row in 0..5 {
                for col in 0..5 {
                    if (row, col) == (2, 2) {
                        continue;
                    }

                    let adjacent_bugs: u8 = neighbors(grid, outer, inner, row, col)
                        .iter()
                        .map(|b| *b as u8)
                        .sum();

                    if ind!(grid, row, col) {
                        if adjacent_bugs != 1 {
                            ind!(new_grid, row, col) = false;
                        }
                    } else if adjacent_bugs == 1 || adjacent_bugs == 2 {
                        ind!(new_grid, row, col) = true;
                    }
                }
            }
        }

        if new.layouts[&new.max_height].iter().all(|&b| !b) {
            new.layouts.remove(&new.max_height);
            new.max_height -= 1;
        }

        if new.layouts[&new.min_height].iter().all(|&b| !b) {
            new.layouts.remove(&new.min_height);
            new.min_height += 1;
        }
        new
    }
}

fn part2(mut rec_layout: RecursiveLayout) -> u32 {
    for _ in 0..200 {
        rec_layout = rec_layout.next();
    }
    rec_layout.count_bugs()
}

fn main() {
    let input = include_str!("input.txt");
    // part 1
    let layout: Layout = input.parse().unwrap();
    let biodiversity = part1(layout);
    println!("Part 1: biodiversity = {}", biodiversity);
    // part 2
    let rec_layout = RecursiveLayout::from(layout);
    let bug_count = part2(rec_layout);
    println!("Part 2: bugs = {}", bug_count);
}
