enum ParameterMode {
    Position,
    Immediate,
}

enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals
}

struct Instruction {
    opcode: Opcode,
    p1_mode: ParameterMode,
    p2_mode: ParameterMode,
    p3_mode: ParameterMode,
}

impl Instruction {
    fn parse_parameter_mode(mode: char) -> Result<ParameterMode, &'static str> {
        match mode {
            '0' => Ok(ParameterMode::Position),
            '1' => Ok(ParameterMode::Immediate),
            _ => Err("invalid parameter mode"),
        }
    }

    fn parse_opcode(opcode: &str) -> Result<Opcode, &'static str> {
        Ok(match opcode {
            "01" => Opcode::Add,
            "02" => Opcode::Multiply,
            "03" => Opcode::Input,
            "04" => Opcode::Output,
            "05" => Opcode::JumpIfTrue,
            "06" => Opcode::JumpIfFalse,
            "07" => Opcode::LessThan,
            "08" => Opcode::Equals,
            "99" => Opcode::Halt,
            _ => return Err("invalid opcode"),
        })
    }

    fn parse(instr: i32) -> Result<Instruction, &'static str> {
        let instr_str = format!("{:0>5}", instr);
        let mut chars = instr_str.chars();
        let p3_mode = Instruction::parse_parameter_mode(chars.next().unwrap())?;
        let p2_mode = Instruction::parse_parameter_mode(chars.next().unwrap())?;
        let p1_mode = Instruction::parse_parameter_mode(chars.next().unwrap())?;
        let opcode = Instruction::parse_opcode(&chars.take(2).collect::<String>())?;

        Ok(Instruction {
            opcode,
            p1_mode,
            p2_mode,
            p3_mode,
        })
    }
}

struct Emulator<'a> {
    memory: Vec<i32>,
    input: &'a [i32],
    ip: i32,
    halted: bool,
}

impl<'a> Emulator<'a> {
    fn new(code: &str, input: &'a [i32]) -> Result<Emulator<'a>, &'static str> {
        let memory = code
            .split(',')
            .map(|item| item.parse().map_err(|_| "failed to parse integer"))
            .collect::<Result<Vec<i32>, &'static str>>()?;

        Ok(Emulator {
            memory,
            input,
            ip: 0,
            halted: false,
        })
    }

    fn get(&self, address: i32) -> i32 {
        self.memory[address as usize]
    }

    fn store(&mut self, address: i32, value: i32) {
        self.memory[address as usize] = value;
    }

    fn get_arg_val(&self, n: i32, mode: ParameterMode) -> i32 {
        let arg = self.get(self.ip + n);
        match mode {
            ParameterMode::Immediate => arg,
            ParameterMode::Position => self.get(arg)
        }
    }

    fn step(&mut self) -> Result<(), &'static str> {
        let instr_code = self.get(self.ip);
        let instr = Instruction::parse(instr_code)?;

        match instr.opcode {
            Opcode::Add => {
                let arg1 = self.get_arg_val(1, instr.p1_mode);
                let arg2 = self.get_arg_val(2, instr.p2_mode);
                let res_addr = self.get(self.ip + 3);
                self.store(res_addr, arg1 + arg2);
                self.ip += 4;
            }
            Opcode::Multiply => {
                let arg1 = self.get_arg_val(1, instr.p1_mode);
                let arg2 = self.get_arg_val(2, instr.p2_mode);
                let res_addr = self.get(self.ip + 3);
                self.store(res_addr, arg1 * arg2);
                self.ip += 4;
            }
            Opcode::Input => {
                let (first, rest) = self.input.split_first().ok_or("input too short")?;
                self.input = rest;
                let res_addr = self.get(self.ip + 1);
                self.store(res_addr, *first);
                self.ip += 2;
            }
            Opcode::Output => {
                let arg = self.get_arg_val(1, instr.p1_mode);
                println!("Output: {}", arg);
                self.ip += 2;
            }
            Opcode::JumpIfTrue => {
                let cond = self.get_arg_val(1, instr.p1_mode);
                let dest = self.get_arg_val(2, instr.p2_mode);
                if cond != 0 {
                    self.ip = dest;
                } else {
                    self.ip += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let cond = self.get_arg_val(1, instr.p1_mode);
                let dest = self.get_arg_val(2, instr.p2_mode);
                if cond == 0 {
                    self.ip = dest;
                } else {
                    self.ip += 3;
                }
            }
            Opcode::LessThan => {
                let arg1 = self.get_arg_val(1, instr.p1_mode);
                let arg2 = self.get_arg_val(2, instr.p2_mode);
                let res_addr = self.get(self.ip + 3);
                self.store(res_addr, (arg1 < arg2) as i32);
                self.ip += 4;
            }
            Opcode::Equals => {
                let arg1 = self.get_arg_val(1, instr.p1_mode);
                let arg2 = self.get_arg_val(2, instr.p2_mode);
                let res_addr = self.get(self.ip + 3);
                self.store(res_addr, (arg1 == arg2) as i32);
                self.ip += 4;
            }
            Opcode::Halt => {
                self.halted = true;
            }
        }

        Ok(())
    }

    fn run(&mut self) -> Result<(), &'static str> {
        while !self.halted {
            self.step()?
        }

        Ok(())
    }
}

fn part1(program: &str) {
    let input = [1];
    let mut emulator = Emulator::new(program, &input).expect("failed to parse program");
    emulator.run().expect("failed to run program");
}

fn part2(program: &str) {
    let input = [5];
    let mut emulator = Emulator::new(program, &input).expect("failed to parse program");
    emulator.run().expect("failed to run program");
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1:");
    println!("-------");
    part1(input);
    println!("Part 2:");
    println!("-------");
    part2(input);
}
