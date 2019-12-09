mod emulator;

fn part1(source: &str) {
    let p = emulator::Program::new(source).expect("failed to parse program");
    emulator::Emulator::run_program_with_input(p, vec![1]).expect("failed to run program");
}

fn main() {
    let source = include_str!("input.txt");
    part1(source);
}
