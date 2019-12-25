// efficiently simulate springscript programs

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InputRegister {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
}
const INPUT_REGISTERS: [InputRegister; 9] = [
    InputRegister::A,
    InputRegister::B,
    InputRegister::C,
    InputRegister::D,
    InputRegister::E,
    InputRegister::F,
    InputRegister::G,
    InputRegister::H,
    InputRegister::I,
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum IORegister {
    T = 0,
    J = 1,
}
const IO_REGISTERS: [IORegister; 2] = [IORegister::T, IORegister::J];

#[derive(Copy, Clone, Eq, PartialEq)]
enum Register {
    I(InputRegister),
    IO(IORegister),
}
impl ToString for Register {
    fn to_string(&self) -> String {
        match self {
            Register::I(i) => format!("{:?}", i),
            Register::IO(io) => format!("{:?}", io),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operation {
    NOT,
    AND,
    OR,
}

const OPERATIONS: [Operation; 3] = [Operation::NOT, Operation::AND, Operation::OR];

#[derive(Copy, Clone, Eq, PartialEq)]
struct Instruction {
    op: Operation,
    r1: Register,
    r2: IORegister,
}
impl ToString for Instruction {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.op, self.r1.to_string(), self.r2)
    }
}

fn sim_program(program: &[Instruction], inputs: &[bool]) -> bool {
    let mut io_registers = [false; 2];
    for instr in program {
        let input = match instr.r1 {
            Register::I(ir) => inputs[ir as usize],
            Register::IO(ior) => io_registers[ior as usize],
        };
        let output = &mut io_registers[instr.r2 as usize];
        *output = match instr.op {
            Operation::NOT => !input,
            Operation::AND => input && *output,
            Operation::OR => input || *output,
        };
    }
    io_registers[IORegister::J as usize]
}

// returns true if successful
fn test_program(program: &[Instruction], map: &[bool]) -> bool {
    let mut pos = 0;
    let mut map = Vec::from(map);
    map.extend(&[true, true, true, true, true, true, true, true, true]);

    while pos < map.len() - 9 {
        if !map[pos] {
            return false;
        }
        let jump = sim_program(program, &map[pos + 1..pos + 10]);
        pos += if jump { 4 } else { 1 };
    }
    true
}

fn all_instructions(limit_inputs: bool) -> Vec<Instruction> {
    let input_registers = if limit_inputs {
        &INPUT_REGISTERS[..4]
    } else {
        &INPUT_REGISTERS[..]
    };

    let mut registers = Vec::with_capacity(input_registers.len() + IO_REGISTERS.len());
    for ir in input_registers {
        registers.push(Register::I(*ir));
    }
    for ior in &IO_REGISTERS {
        registers.push(Register::IO(*ior));
    }

    let mut res = Vec::new();
    for operation in &OPERATIONS {
        for r1 in &registers {
            for r2 in &IO_REGISTERS {
                if operation != &Operation::NOT && *r1 == Register::IO(*r2) {
                    // don't use noop instructions: AND <X> <X> / OR <X> <X>
                    continue;
                }

                res.push(Instruction {
                    op: *operation,
                    r1: *r1,
                    r2: *r2,
                })
            }
        }
    }

    res
}

fn gen_instrs<'a>(
    len: usize,
    all_instrs: &'a [Instruction],
) -> Box<dyn Iterator<Item = Vec<Instruction>> + 'a> {
    if len == 1 {
        Box::new(all_instrs.iter().map(|&instr| vec![instr]))
    } else {
        Box::new(all_instrs.iter().flat_map(move |&last_instr| {
            gen_instrs(len - 1, all_instrs).filter_map(move |mut vec| {
                if last_instr != *all_instrs.last().unwrap() {
                    vec.push(last_instr);
                    Some(vec)
                } else {
                    None
                }
            })
        }))
    }
}

pub fn find_program<T: AsRef<[bool]>>(limit_inputs: bool, tests: &[T]) -> String {
    let all_instrs = all_instructions(limit_inputs);
    for len in 1..=15 {
        println!("Testing programs of length {}", len);
        for program in gen_instrs(len, &all_instrs) {
            if tests
                .iter()
                .all(|test| test_program(&program, test.as_ref()))
            {
                return program
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");
            }
        }
    }
    panic!("could not find valid program")
}
