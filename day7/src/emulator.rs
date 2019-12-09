use std::sync::mpsc::{Receiver, SyncSender};

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
    Equals,
}

struct Instruction {
    opcode: Opcode,
    p1_mode: ParameterMode,
    p2_mode: ParameterMode,
    p3_mode: ParameterMode,
}

impl Instruction {
    fn parse_parameter_mode(mode: i32) -> Result<ParameterMode, &'static str> {
        match mode {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err("invalid parameter mode"),
        }
    }

    fn parse_opcode(opcode: i32) -> Result<Opcode, &'static str> {
        Ok(match opcode {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            _ => return Err("invalid opcode"),
        })
    }

    fn parse(instr: i32) -> Result<Instruction, &'static str> {
        let p3_mode = Instruction::parse_parameter_mode((instr / 10000) % 10)?;
        let p2_mode = Instruction::parse_parameter_mode((instr / 1000) % 10)?;
        let p1_mode = Instruction::parse_parameter_mode((instr / 100) % 10)?;
        let opcode = Instruction::parse_opcode(instr % 100)?;

        Ok(Instruction {
            opcode,
            p1_mode,
            p2_mode,
            p3_mode,
        })
    }
}

pub struct Emulator {
    memory: Vec<i32>,
    ip: i32,
    halted: bool,

    // use options to allow the channel half to be dropped
    input: Option<Receiver<i32>>,
    output: Option<SyncSender<i32>>,
}

impl Emulator {
    pub fn new(
        code: &str,
        input: Receiver<i32>,
        output: SyncSender<i32>,
    ) -> Result<Emulator, &'static str> {
        let memory = code
            .split(',')
            .map(|item| item.parse().map_err(|_| "failed to parse integer"))
            .collect::<Result<Vec<i32>, &'static str>>()?;

        Ok(Emulator {
            memory,
            ip: 0,
            halted: false,
            input: Some(input),
            output: Some(output),
        })
    }

    pub fn dup_memory(&self, input: Receiver<i32>, output: SyncSender<i32>) -> Emulator {
        let memory = self.memory.clone();
        Emulator {
            memory,
            ip: 0,
            halted: false,
            input: Some(input),
            output: Some(output),
        }
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
            ParameterMode::Position => self.get(arg),
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
                // safe to unwrap because input will only be None when the emulator is halted
                let input = self
                    .input
                    .as_ref()
                    .unwrap()
                    .recv()
                    .map_err(|_| "input failed")?;
                let res_addr = self.get(self.ip + 1);
                self.store(res_addr, input);
                self.ip += 2;
            }
            Opcode::Output => {
                let arg = self.get_arg_val(1, instr.p1_mode);
                // safe to unwrap because output will only be None when the emulator is halted
                self.output
                    .as_ref()
                    .unwrap()
                    .send(arg)
                    .map_err(|_| "output failed")?;
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
                // drop input and output channel halves
                self.input = None;
                self.output = None;
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        while !self.halted {
            self.step()?
        }

        Ok(())
    }
}
