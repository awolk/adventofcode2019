mod emulator;
use crate::emulator::{Emulator, Program, Status};
use std::sync::mpsc::{channel, sync_channel};
use std::thread::spawn;

fn build_network(program: Program) {
    let mut senders = Vec::new();
    let mut emulators = Vec::new();
    for addr in 0..50 {
        let (send, recv) = channel::<i64>();
        senders.push(send);

        let mut emu = Emulator::new(program.clone());
        emu.add_input(addr);
        emulators.push((emu, recv));
    }

    let (res_send, res_recv) = channel();

    for (mut emu, recv) in emulators {
        let senders = senders.clone();
        let res_send = res_send.clone();
        spawn(move || loop {
            let status = emu.run().expect("emulator failed");
            match status {
                Status::Halted => return,
                Status::NeedsInput => {
                    if let Ok(x) = recv.try_recv() {
                        let y = recv.recv().unwrap();
                        emu.add_input(x);
                        emu.add_input(y);
                    } else {
                        emu.add_input(-1);
                    }
                }
                Status::Output(dest) => {
                    let x = match emu.run().expect("emulator failed") {
                        Status::Output(i) => i,
                        _ => panic!("expected output from emulator"),
                    };
                    let y = match emu.run().expect("emulator failed") {
                        Status::Output(i) => i,
                        _ => panic!("expected output from emulator"),
                    };
                    let chan = if dest == 255 {
                        &res_send
                    } else {
                        &senders[dest as usize]
                    };
                    chan.send(x).expect("send failed");
                    chan.send(y).expect("send failed");
                }
            }
        });
    }

    let x = res_recv.recv().expect("recv failed");
    let y = res_recv.recv().expect("recv failed");
    println!("Part 1: [255] Y = {}", y);
}

fn main() {
    let input = include_str!("input.txt");
    let program = Program::new(input).expect("failed to parse program");
    build_network(program)
}
