mod emulator;
use emulator::{Emulator, Program, Status};
use std::io::{self, Read, Write};

fn run_ascii_program(program: Program) {
    let mut emu = Emulator::new(program);
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
            Status::NeedsInput => {
                let mut buf = [0];
                io::stdin().read_exact(&mut buf).expect("read failed");
                emu.add_input(buf[0] as i64);
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let program = Program::new(input).unwrap();
    run_ascii_program(program);
    // dont take:
    // - photons
    // - giant electromagnet
    // - infinite loop
    // - escape pod
    // - molten lava

    // to get password need:
    // - ornament
    // - astrolabe
    // - sand
    // - shell

    // password: 134807554
}
