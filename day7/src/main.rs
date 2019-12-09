use std::sync::mpsc::sync_channel;
use std::thread::spawn;

mod emulator;

fn test_amps(
    amp_control_software: &str,
    phase_setting: [i32; 5],
    use_feedback_loop: bool,
) -> Result<i32, &'static str> {
    let (in1tx, in1rx) = sync_channel(0);
    let (out1tx, out1rx) = sync_channel(0);
    let mut em1 = emulator::Emulator::new(amp_control_software, in1rx, out1tx)?;

    let (in2tx, in2rx) = sync_channel(0);
    let (out2tx, out2rx) = sync_channel(0);
    let mut em2 = em1.dup_memory(in2rx, out2tx);

    let (in3tx, in3rx) = sync_channel(0);
    let (out3tx, out3rx) = sync_channel(0);
    let mut em3 = em1.dup_memory(in3rx, out3tx);

    let (in4tx, in4rx) = sync_channel(0);
    let (out4tx, out4rx) = sync_channel(0);
    let mut em4 = em1.dup_memory(in4rx, out4tx);

    let (in5tx, in5rx) = sync_channel(0);
    let (out5tx, out5rx) = sync_channel(0);
    let mut em5 = em1.dup_memory(in5rx, out5tx);

    // start amplifiers
    spawn(move || em1.run());
    spawn(move || em2.run());
    spawn(move || em3.run());
    spawn(move || em4.run());
    spawn(move || em5.run());

    // send phase settings
    in1tx.send(phase_setting[0]).map_err(|_| "send failed")?;
    in2tx.send(phase_setting[1]).map_err(|_| "send failed")?;
    in3tx.send(phase_setting[2]).map_err(|_| "send failed")?;
    in4tx.send(phase_setting[3]).map_err(|_| "send failed")?;
    in5tx.send(phase_setting[4]).map_err(|_| "send failed")?;

    // connect pipeline
    if !use_feedback_loop {
        in1tx.send(0).map_err(|_| "send failed")?;
        in2tx
            .send(out1rx.recv().map_err(|_| "receive failed")?)
            .map_err(|_| "send failed")?;
        in3tx
            .send(out2rx.recv().map_err(|_| "receive failed")?)
            .map_err(|_| "send failed")?;
        in4tx
            .send(out3rx.recv().map_err(|_| "receive failed")?)
            .map_err(|_| "send failed")?;
        in5tx
            .send(out4rx.recv().map_err(|_| "receive failed")?)
            .map_err(|_| "send failed")?;

        out5rx.recv().map_err(|_| "receive failed")
    } else {
        in1tx.send(0).map_err(|_| "send failed")?;
        let mut last_output = 0; // placeholder

        loop {
            in2tx
                .send(out1rx.recv().map_err(|_| "receive failed")?)
                .map_err(|_| "send failed")?;
            in3tx
                .send(out2rx.recv().map_err(|_| "receive failed")?)
                .map_err(|_| "send failed")?;
            in4tx
                .send(out3rx.recv().map_err(|_| "receive failed")?)
                .map_err(|_| "send failed")?;
            in5tx
                .send(out4rx.recv().map_err(|_| "receive failed")?)
                .map_err(|_| "send failed")?;

            last_output = out5rx.recv().map_err(|_| "receive failed")?;
            if let Err(_) = in1tx.send(last_output) {
                // if error, the channel is closed and we have received the last output
                return Ok(last_output);
            }
        }
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

fn part1(amp_control_software: &str) {
    let best_output = permutations5([0, 1, 2, 3, 4])
        .map(|phase_setting| {
            test_amps(amp_control_software, phase_setting, false).expect("failed test")
        })
        .max()
        .expect("failed to find max");

    println!("Part 1: highest thruster signal = {}", best_output);
}

fn part2(amp_control_software: &str) {
    let best_output = permutations5([5, 6, 7, 8, 9])
        .map(|phase_setting| {
            test_amps(amp_control_software, phase_setting, true).expect("failed test")
        })
        .max()
        .expect("failed to find max");

    println!("Part 1: highest thruster signal = {}", best_output);
}

fn main() {
    let amp_control_software = include_str!("input.txt");
    part1(amp_control_software);
    part2(amp_control_software);
}
