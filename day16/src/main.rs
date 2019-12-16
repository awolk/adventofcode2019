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

fn main() {
    let input = include_bytes!("input.txt");
    let mut signal: Vec<u8> = input.iter().map(|byte| byte - b'0').collect();

    let mut patterns: Vec<Vec<i8>> = Vec::with_capacity(signal.len());
    for i in 0..signal.len() {
        patterns.push(gen_pattern(i, signal.len()))
    }

    for _ in 0..100 {
        let new_signal = patterns
            .iter()
            .map(|pattern| apply_pattern(&signal, pattern))
            .collect();
        signal = new_signal;
    }

    let final_signal = String::from_utf8(signal[0..8].iter().map(|b| *b + b'0').collect()).unwrap();

    println!("Part 1: Final signal =  {}", final_signal);
}
