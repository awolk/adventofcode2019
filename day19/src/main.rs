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

fn main() {
    let input = include_str!("input.txt");
    let program = Program::new(input).expect("failed to parse program");
    let mut image = [false; 50 * 50];
    for x in 0..50 {
        for y in 0..50 {
            image[x * 50 + y] = check_pos(program.clone(), x, y).expect("failed to check position");
        }
    }

    let points = image.iter().filter(|&&p| p).count();

    println!("Part 1: points affected by beam = {}", points);
}
