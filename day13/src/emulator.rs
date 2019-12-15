use std::{
    io::{self, Write},
    iter,
};

enum ParameterMode {
    Position,
    Immediate,
    Relative,
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
    AdjustRelativeBase,
}

struct Instruction {
    opcode: Opcode,
    p1_mode: ParameterMode,
    p2_mode: ParameterMode,
    p3_mode: ParameterMode,
}

impl Instruction {
    fn parse_parameter_mode(mode: i64) -> Result<ParameterMode, &'static str> {
        Ok(match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => return Err("invalid parameter mode"),
        })
    }

    fn parse_opcode(opcode: i64) -> Result<Opcode, &'static str> {
        Ok(match opcode {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            9 => Opcode::AdjustRelativeBase,
            99 => Opcode::Halt,
            _ => return Err("invalid opcode"),
        })
    }

    fn parse(instr: i64) -> Result<Instruction, &'static str> {
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

#[derive(Clone)]
pub struct Program {
    memory: Vec<i64>,
}

impl Program {
    pub fn new(code: &str) -> Option<Program> {
        let memory = code
            .split(',')
            .map(|item| item.parse().ok())
            .collect::<Option<Vec<i64>>>()?;

        Some(Program { memory })
    }
}

pub struct Emulator {
    memory: Vec<i64>,
    ip: i64,
    relative_base: i64,
    halted: bool,
}

impl Emulator {
    pub fn new(program: Program) -> Emulator {
        Emulator {
            memory: program.memory,
            ip: 0,
            relative_base: 0,
            halted: false,
        }
    }

    fn get(&self, address: i64) -> i64 {
        let address = address as usize;
        if address >= self.memory.len() {
            0
        } else {
            self.memory[address]
        }
    }

    pub fn store(&mut self, address: i64, value: i64) {
        let address = address as usize;
        if address >= self.memory.len() {
            self.memory
                .extend(iter::repeat(0).take(address - self.memory.len() + 1))
        }
        self.memory[address] = value;
    }

    fn get_arg_val(&self, n: i64, mode: ParameterMode) -> i64 {
        let arg = self.get(self.ip + n);
        match mode {
            ParameterMode::Immediate => arg,
            ParameterMode::Position => self.get(arg),
            ParameterMode::Relative => self.get(arg + self.relative_base),
        }
    }

    fn get_arg_dest(&self, n: i64, mode: ParameterMode) -> Result<i64, &'static str> {
        let arg = self.get(self.ip + n);
        Ok(match mode {
            ParameterMode::Position => arg,
            ParameterMode::Relative => arg + self.relative_base,
            ParameterMode::Immediate => return Err("destination cannot be in immediate mode"),
        })
    }

    /// returns Ok(true) if halted
    pub fn step(
        &mut self,
        get_input: impl FnOnce() -> Result<i64, &'static str>,
        handle_output: impl FnOnce(i64) -> Result<(), &'static str>,
    ) -> Result<bool, &'static str> {
        let instr_code = self.get(self.ip);
        let instr = Instruction::parse(instr_code)?;

        match instr.opcode {
            Opcode::Add => {
                let arg1 = self.get_arg_val(1, instr.p1_mode);
                let arg2 = self.get_arg_val(2, instr.p2_mode);
                let res_addr = self.get_arg_dest(3, instr.p3_mode)?;
                self.store(res_addr, arg1 + arg2);
                self.ip += 4;
            }
            Opcode::Multiply => {
                let arg1 = self.get_arg_val(1, instr.p1_mode);
                let arg2 = self.get_arg_val(2, instr.p2_mode);
                let res_addr = self.get_arg_dest(3, instr.p3_mode)?;
                self.store(res_addr, arg1 * arg2);
                self.ip += 4;
            }
            Opcode::Input => {
                let input = get_input()?;
                let res_addr = self.get_arg_dest(1, instr.p1_mode)?;
                self.store(res_addr, input);
                self.ip += 2;
            }
            Opcode::Output => {
                let arg = self.get_arg_val(1, instr.p1_mode);
                handle_output(arg)?;
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
                let res_addr = self.get_arg_dest(3, instr.p3_mode)?;
                self.store(res_addr, (arg1 < arg2) as i64);
                self.ip += 4;
            }
            Opcode::Equals => {
                let arg1 = self.get_arg_val(1, instr.p1_mode);
                let arg2 = self.get_arg_val(2, instr.p2_mode);
                let res_addr = self.get_arg_dest(3, instr.p3_mode)?;
                self.store(res_addr, (arg1 == arg2) as i64);
                self.ip += 4;
            }
            Opcode::AdjustRelativeBase => {
                let arg = self.get_arg_val(1, instr.p1_mode);
                self.relative_base += arg;
                self.ip += 2;
            }
            Opcode::Halt => {
                self.halted = true;
                return Ok(true);
            }
        }

        Ok(false)
    }
}
