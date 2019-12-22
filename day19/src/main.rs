mod emulator;
use self::emulator::{Emulator, Program, Status};

fn check_pos(program: Program, x: usize, y: usize) -> Result<bool, &'static str> {
    let mut emu = Emulator::new(program);
    emu.add_input(x as i64);
    emu.add_input(y as i64);
    match emu.run()? {
        Status::Output(1) => Ok(true),
        Status::Output(0) => Ok(false),
        _ => Err("unexpected emulator output"),
    }
}

fn gen_image(
    program: Program,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Result<Vec<bool>, &'static str> {
    let mut res = Vec::with_capacity(width * height);
    for y in y..y + height {
        for x in x..x + width {
            res.push(check_pos(program.clone(), x, y)?);
        }
    }
    Ok(res)
}

#[allow(dead_code)]
fn print_image(image: &[bool], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", if image[y * width + x] { '#' } else { '.' });
        }
        println!();
    }
}

fn ship_fits(program: Program, x: usize, y: usize, size: usize) -> Result<bool, &'static str> {
    for y in (y + 1 - size)..=y {
        for x in x..x + size {
            if !check_pos(program.clone(), x, y)? {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

// returns bottom left corner
fn find_pos_for_ship(
    program: Program,
    mut x: usize,
    mut y: usize,
    size: usize,
) -> Result<(usize, usize), &'static str> {
    while !(ship_fits(program.clone(), x, y, size)?) {
        y += 1;
        while !check_pos(program.clone(), x, y)? {
            x += 1;
        }
    }

    Ok((x, y))
}

fn main() {
    let input = include_str!("input.txt");
    let program = Program::new(input).expect("failed to parse program");

    let image = gen_image(program.clone(), 0, 0, 50, 50).expect("failed to generate image");
    let points = image.iter().filter(|&&p| p).count();
    println!("Part 1: points affected by beam = {}", points);

    let size = 100;
    let (x, y) = find_pos_for_ship(program, 0, 99, size).expect("failed to find position for ship");
    let y = y - size + 1; // use top-left corner

    let res = x * 10_000 + y;
    println!("Part 2: result = {}", res);
}
