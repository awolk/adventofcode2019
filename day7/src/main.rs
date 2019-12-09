use crate::emulator::Program;
use std::sync::mpsc::sync_channel;
use std::thread::spawn;

mod emulator;

fn test_amps(
    amp_control_program: emulator::Program,
    phase_setting: [i32; 5],
    use_feedback_loop: bool,
) -> Result<i32, &'static str> {
    let (send_in_to_1, recv_in_to_1) = sync_channel(0);
    let (send_1_to_2, recv_1_to_2) = sync_channel(0);
    let (send_2_to_3, recv_2_to_3) = sync_channel(0);
    let (send_3_to_4, recv_3_to_4) = sync_channel(0);
    let (send_4_to_5, recv_4_to_5) = sync_channel(0);
    let (send_5_to_out, recv_5_to_out) = sync_channel(0);

    let mut em1 = emulator::Emulator::new(
        amp_control_program.clone(),
        recv_in_to_1,
        send_1_to_2.clone(),
    )?;
    let mut em2 = emulator::Emulator::new(
        amp_control_program.clone(),
        recv_1_to_2,
        send_2_to_3.clone(),
    )?;
    let mut em3 = emulator::Emulator::new(
        amp_control_program.clone(),
        recv_2_to_3,
        send_3_to_4.clone(),
    )?;
    let mut em4 = emulator::Emulator::new(
        amp_control_program.clone(),
        recv_3_to_4,
        send_4_to_5.clone(),
    )?;
    let mut em5 = emulator::Emulator::new(amp_control_program, recv_4_to_5, send_5_to_out)?;

    // start amplifiers
    spawn(move || em1.run());
    spawn(move || em2.run());
    spawn(move || em3.run());
    spawn(move || em4.run());
    spawn(move || em5.run());

    // send phase settings
    send_in_to_1
        .send(phase_setting[0])
        .map_err(|_| "send failed")?;
    send_1_to_2
        .send(phase_setting[1])
        .map_err(|_| "send failed")?;
    send_2_to_3
        .send(phase_setting[2])
        .map_err(|_| "send failed")?;
    send_3_to_4
        .send(phase_setting[3])
        .map_err(|_| "send failed")?;
    send_4_to_5
        .send(phase_setting[4])
        .map_err(|_| "send failed")?;

    // connect pipeline
    if !use_feedback_loop {
        send_in_to_1.send(0).map_err(|_| "send failed")?;

        loop {
            let last_output = recv_5_to_out.recv().map_err(|_| "receive failed")?;
            if send_in_to_1.send(last_output).is_err() {
                // if error, the channel is closed and we have received the last output
                return Ok(last_output);
            }
        }
    } else {
        send_in_to_1.send(0).map_err(|_| "send failed")?;
        recv_5_to_out.recv().map_err(|_| "receive failed")
    }
}

fn permutations5(values: [i32; 5]) -> impl Iterator<Item = [i32; 5]> {
    // uses Heap's algorithm, adapted from pseudocode on Wikipedia
    let mut res: Vec<[i32; 5]> = Vec::with_capacity(120 /* 5! */);

    let n = 5;
    let mut a = values;
    let mut c = [0, 0, 0, 0, 0];

    res.push(a);

    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                a.swap(0, i);
            } else {
                a.swap(c[i], i);
            }
            res.push(a);
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    res.into_iter()
}

fn part1(amp_control_program: Program) {
    let best_output = permutations5([0, 1, 2, 3, 4])
        .map(|phase_setting| {
            test_amps(amp_control_program.clone(), phase_setting, false).expect("failed test")
        })
        .max()
        .expect("failed to find max");

    println!("Part 1: highest thruster signal = {}", best_output);
}

fn part2(amp_control_program: Program) {
    let best_output = permutations5([5, 6, 7, 8, 9])
        .map(|phase_setting| {
            test_amps(amp_control_program.clone(), phase_setting, true).expect("failed test")
        })
        .max()
        .expect("failed to find max");

    println!("Part 1: highest thruster signal = {}", best_output);
}

fn main() {
    let amp_control_code = include_str!("input.txt");
    let amp_control_program = Program::new(amp_control_code).expect("failed to parse program");
    part1(amp_control_program.clone());
    part2(amp_control_program);
}
