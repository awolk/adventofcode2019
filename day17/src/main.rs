use crate::emulator::Emulator;
use crate::Item::{Empty, Scaffold};

mod emulator;

#[derive(PartialEq)]
enum Item {
    Empty,
    Scaffold,
}

fn get_image(emu: &mut Emulator) -> Result<Vec<Vec<Item>>, &'static str> {
    let mut image = Vec::new();
    let mut row = Box::new(Vec::new());

    loop {
        let halted = emu.step(
            || Err("no input"),
            |i| {
                match (i as u8) as char {
                    '.' => row.push(Empty),
                    '#' | '<' | '>' | '^' | 'v' => row.push(Scaffold),
                    '\n' => {
                        if row.len() > 0 {
                            image.push(std::mem::replace(&mut *row, Vec::new()));
                        }
                    }
                    _ => return Err("invalid pixel"),
                }
                Ok(())
            },
        )?;

        if halted {
            break;
        }
    }

    Ok(image)
}

fn main() {
    let input = include_str!("input.txt");
    let program = emulator::Program::new(input).expect("parsing failed");
    let mut emu = emulator::Emulator::new(program);
    let image = get_image(&mut emu).expect("failed to get image");

    let mut res = 0;

    let height = image.len();
    let width = image[0].len();
    for row in 1..height - 1 {
        for col in 1..width - 1 {
            if image[row][col] == Scaffold
                && image[row - 1][col] == Scaffold
                && image[row + 1][col] == Scaffold
                && image[row][col - 1] == Scaffold
                && image[row][col + 1] == Scaffold
            {
                res += row * col;
            }
        }
    }

    println!("Part 1: sum of alignment params = {}", res);
}
