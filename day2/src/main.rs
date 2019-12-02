fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.parse().expect("expected integer"))
        .collect()
}

fn eval(program: &mut [usize]) {
    let mut pc = 0;
    loop {
        match program[pc] {
            1 => program[program[pc + 3]] = program[program[pc + 1]] + program[program[pc + 2]],
            2 => program[program[pc + 3]] = program[program[pc + 1]] * program[program[pc + 2]],
            99 => return,
            _ => panic!("invalid opcode")
        }
        pc += 4;
    }
}

fn restore(program: &mut [usize]) {
    program[1] = 12;
    program[2] = 2;
}

fn main() {
    let input = include_str!("input.txt");
    let mut program = parse(input);
    restore(&mut program);
    eval(&mut program);
    println!("Position 0: {}", program[0]);
}
