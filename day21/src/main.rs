mod emulator;
mod springscript_sim;
use crate::emulator::{Emulator, Program, Status};
use std::io;
use std::io::Write;

fn run_ascii_program_with_input(program: Program, input: &str) {
    let mut emu = Emulator::new(program);
    emu.add_inputs(input.bytes().map(|b| b as i64));
    loop {
        let status = emu.run().expect("emulator error");
        match status {
            Status::Halted => return,
            Status::Output(out) => {
                if out < 255 {
                    io::stdout().write_all(&[out as u8]).expect("write failed");
                } else {
                    println!("Non-ASCII Output: {}", out);
                }
            }
            Status::NeedsInput => panic!("input is short"),
        }
    }
}

fn parse_test(test: &str) -> Vec<bool> {
    test.bytes().map(|b| b == b'#').collect()
}

// I added tests here when the searcher would find a springscript program that would fail on the
// real input. I used this method, with a springscript simulator, because simulating the
// springscript programs is a lot faster than testing them with the input intcode and an intcode
// emulator. Part 2 still took a very long time to run.
#[allow(dead_code)]
fn find_programs(program: Program) {
    // part 1
    let tests = [
        parse_test("####.####"),
        parse_test("#####..#.####"),
        parse_test("#####...####"),
        parse_test("#####.#..####"),
    ];
    let springscript = springscript_sim::find_program(true, &tests) + "\nWALK\n";
    println!("Program:\n{}", springscript);
    run_ascii_program_with_input(program.clone(), &(springscript));

    // part 2
    let tests = [
        parse_test("#.#"),
        parse_test("#####..#.#"),
        parse_test("####...#"),
        parse_test("####.#..#"),
        parse_test("####.#.##.#.#"),
        parse_test("####.###.#..#"),
        parse_test("####..#..##.#"),
        parse_test("#####.###.#..#"),
        parse_test("####.###..#..#"),
        parse_test("#####.#.#.#.#.###"),
        parse_test("#####.####.#..###"),
    ];
    let springscript = springscript_sim::find_program(false, &tests) + "\nRUN\n";
    println!("Program:\n{}", springscript);
    run_ascii_program_with_input(program, &(springscript));
}

fn main() {
    let input = include_str!("input.txt");
    let program = Program::new(input).expect("failed to parse program");

    // programs were generated automatically, but this is slow for part 2
    println!("Part 1:\n------");
    let part1_input = "NOT D T
OR C T
AND A T
NOT T J
WALK
";
    run_ascii_program_with_input(program.clone(), &part1_input);
    println!("\nPart 2:\n------");
    let part2_input = "NOT H T
OR C T
AND B T
AND A T
NOT T J
AND D J
RUN
";
    run_ascii_program_with_input(program, &part2_input);
}
