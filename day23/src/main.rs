mod emulator;
use crate::emulator::{Emulator, Program, Status};
use std::cmp::min;
use std::collections::VecDeque;

fn run(program: Program) {
    let mut queues: Vec<VecDeque<(i64, i64)>> = vec![VecDeque::new(); 50];
    let mut nat_queue: VecDeque<(i64, i64)> = VecDeque::new();
    let mut idle = vec![0; 50];
    let mut emus = Vec::new();
    for addr in 0..50 {
        let mut emu = Emulator::new(program.clone());
        emu.add_input(addr);
        emus.push(emu);
    }

    let mut first_nat_packet = true;
    let mut last_sent = None;

    loop {
        for (addr, emu) in emus.iter_mut().enumerate() {
            match emu.run().unwrap() {
                Status::Halted => panic!("emulator halted"),
                Status::Output(dest) => {
                    let x = match emu.run().unwrap() {
                        Status::Output(i) => i,
                        _ => panic!("expected output"),
                    };
                    let y = match emu.run().unwrap() {
                        Status::Output(i) => i,
                        _ => panic!("expected output"),
                    };
                    let queue = if dest == 255 {
                        if first_nat_packet {
                            println!("Part 1: Y = {}", y);
                            first_nat_packet = false;
                        }
                        &mut nat_queue
                    } else {
                        &mut queues[dest as usize]
                    };
                    queue.push_back((x, y));
                    idle[addr] = 0;
                }
                Status::NeedsInput => {
                    if let Some((x, y)) = queues[addr].pop_front() {
                        emu.add_input(x);
                        emu.add_input(y);
                        idle[addr] = 0;
                    } else {
                        emu.add_input(-1);
                        idle[addr] = min(idle[addr] + 1, 100);
                    }
                }
            }
        }
        if idle.iter().all(|&i| i == 100) {
            let packet = *nat_queue.back().unwrap();
            if Some(packet) == last_sent {
                println!("Part 2: Y = {}", packet.1);
                return;
            } else {
                last_sent = Some(packet);
            }
            queues[0].push_back(packet);
            idle = vec![0; 50];
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let program = Program::new(input).expect("failed to parse program");
    run(program);
}
