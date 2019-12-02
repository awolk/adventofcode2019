struct Program(Vec<usize>);

impl From<&str> for Program {
    fn from(source: &str) -> Program {
        Program(
            source
                .split(',')
                .map(|item| item.parse().expect("expected positive integer"))
                .collect(),
        )
    }
}

impl Program {
    fn restore(&mut self, noun: usize, verb: usize) {
        self.0[1] = noun;
        self.0[2] = verb;
    }

    fn eval(&mut self) {
        let mem: &mut [usize] = &mut self.0;
        let mut ic = 0;

        loop {
            match mem[ic] {
                1 => mem[mem[ic + 3]] = mem[mem[ic + 1]] + mem[mem[ic + 2]],
                2 => mem[mem[ic + 3]] = mem[mem[ic + 1]] * mem[mem[ic + 2]],
                99 => return,
                _ => panic!("invalid opcode"),
            }

            ic += 4;
        }
    }

    fn output(&self) -> usize {
        self.0[0]
    }
}

fn part1(input: &str) {
    let mut program = Program::from(input);
    program.restore(12, 2);
    program.eval();
    println!("Part 1 result: {}", program.output());
}

fn part2(input: &str) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = Program::from(input);
            program.restore(noun, verb);
            program.eval();

            if program.output() == 19690720 {
                println!("Part 2 result: {}", 100 * noun + verb);
                return;
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
