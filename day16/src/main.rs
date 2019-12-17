fn gen_pattern(i: usize, len: usize) -> Vec<i8> {
    let mut res = Vec::with_capacity(len);

    let mut stage = 0;
    let mut step = 0;

    while res.len() < len {
        if step == i {
            step = 0;
            stage += 1;
        } else {
            step += 1;
        }

        res.push([0, 1, 0, -1][stage % 4])
    }

    res
}

fn apply_pattern(signal: &[u8], pattern: &[i8]) -> u8 {
    let sum: i64 = signal
        .iter()
        .zip(pattern.iter())
        .map(|(&a, &b)| a as i64 * b as i64)
        .sum();
    (sum.abs() % 10) as u8
}

fn fft(mut signal: Vec<u8>, iterations: u64) -> Vec<u8> {
    let mut patterns: Vec<Vec<i8>> = Vec::with_capacity(signal.len());
    for i in 0..signal.len() {
        patterns.push(gen_pattern(i, signal.len()))
    }

    for _ in 0..iterations {
        let new_signal = patterns
            .iter()
            .map(|pattern| apply_pattern(&signal, pattern))
            .collect();
        signal = new_signal;
    }

    signal
}

fn fft_part2(signal: &[u8], offset: usize, iterations: u64) -> Vec<u8> {
    // The fft process is multiplication by an upper triangular matrix.
    // Sample matrix for signal length 7:
    // [[1  0 -1  0  1  0 -1]
    //  [0  1  1  0  0 -1 -1]
    //  [0  0  1  1  1  0  0]
    //  [0  0  0  1  1  1  1]
    //  [0  0  0  0  1  1  1]
    //  [0  0  0  0  0  1  1]
    //  [0  0  0  0  0  0  1]]
    // Therefore, any parts of the signal before the offset don't affect anything after the offset:
    // the value of any index is only affected by other parts of the signal after that index.
    // After halfway through the matrix, the nth row is (n-1) zeros and then only ones.
    assert!(offset > signal.len() / 2);
    // Therefore, for indices greater than halfway through the signal, for each step:
    //   new_signal[n] = | sum(signal[n..]) | % 10

    let mut signal = signal[offset..].to_vec();
    for _ in 0..iterations {
        let mut sum: i64 = signal.iter().map(|b| *b as i64).sum();
        for entry in signal.iter_mut() {
            let old_val = *entry as i64;
            *entry = (sum.abs() % 10) as u8;
            sum -= old_val;
        }
    }

    signal[..8].to_vec()
}

fn main() {
    let input = include_bytes!("input.txt");
    let signal: Vec<u8> = input.iter().map(|byte| byte - b'0').collect();

    // part 1
    let signal1 = fft(signal.clone(), 100);
    let final_signal = String::from_utf8(signal1[..8].iter().map(|b| *b + b'0').collect()).unwrap();
    println!("Part 1: Final signal = {}", final_signal);

    // part 2
    let signal2 = signal.repeat(10_000);
    let offset = signal[..7]
        .iter()
        .fold(0, |accum, digit| accum * 10 + *digit as usize);
    let res = fft_part2(&signal2, offset, 100);
    let final_signal = String::from_utf8(res[..8].iter().map(|b| *b + b'0').collect()).unwrap();
    println!("Part 2: Final signal = {}", final_signal);
}
