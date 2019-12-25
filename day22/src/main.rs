#[derive(Copy, Clone, Debug)]
enum Shuffle {
    DealIntoNewStack,
    Cut(i64),
    DealWithIncrement(u64),
}

struct Deck {
    cards: Vec<u16>,
}

impl Deck {
    fn new() -> Deck {
        Deck {
            cards: (0..10007).collect(),
        }
    }

    fn apply_shuffle(&self, shuffle: Shuffle) -> Deck {
        match shuffle {
            Shuffle::DealIntoNewStack => self.deal_into_new_stack(),
            Shuffle::Cut(n) => self.cut(n),
            Shuffle::DealWithIncrement(n) => self.deal_with_increment(n),
        }
    }

    fn deal_into_new_stack(&self) -> Deck {
        let mut cards = self.cards.clone();
        cards.reverse();
        Deck { cards }
    }

    fn cut(&self, n: i64) -> Deck {
        if n >= 0 {
            let n = n as usize;
            let old_start = &self.cards[0..n];
            let mut cards = self.cards[n..].to_owned();
            cards.extend(old_start);
            Deck { cards }
        } else {
            let n = n.abs() as usize;
            let mut cards = self.cards[10007 - n..].to_owned();
            cards.extend(&self.cards[..10007 - n]);
            Deck { cards }
        }
    }

    fn deal_with_increment(&self, n: u64) -> Deck {
        let mut cards = vec![0; 10007];
        let mut pos = 0;
        for &card in &self.cards {
            cards[pos as usize] = card;
            pos = (pos + n) % 10007;
        }
        Deck { cards }
    }
}

fn parse_shuffles(input: &str) -> Vec<Shuffle> {
    input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                Shuffle::DealIntoNewStack
            } else if line.starts_with("cut ") {
                let n: i64 = line.trim_start_matches("cut ").parse().unwrap();
                Shuffle::Cut(n)
            } else if line.starts_with("deal with increment ") {
                let n: u64 = line
                    .trim_start_matches("deal with increment ")
                    .parse()
                    .unwrap();
                Shuffle::DealWithIncrement(n)
            } else {
                panic!("invalid line: {}", line)
            }
        })
        .collect()
}

fn part1(shuffles: &[Shuffle]) {
    let mut deck = Deck::new();

    for &shuffle in shuffles {
        deck = deck.apply_shuffle(shuffle);
    }

    println!(
        "Part 1: position = {}",
        deck.cards.iter().position(|&card| card == 2019).unwrap()
    );
}

// Reasoning for part 2:
// for new stack
//  cardAt(pos) = pos
// after deal into new stack
//   cardAt'(pos) = cardAt(len - 1 - pos)
// after cut(n)
//   cardAt'(pos) = cardAt((len + pos + n) mod len)
// after deal with increment(n)
//   pos'(card) = (pos(card) * n) mod len
//   pos(card) = pos'(card) * n^-1 mod len
//   cardAt'(pos') = cardAt(pos(card))
//   cardAt'(pos') = cardAt(pos' * n^-1 mod len)
//   cardAt'(pos) = cardAt(pos * n^-1 mod len)

fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    // algorithm taken from https://en.wikipedia.org/wiki/Modular_exponentiation
    if modulus == 1 {
        return 0;
    }

    let mut res = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % modulus
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    res
}

fn mod_mult_inv(a: i128, m: i128) -> i128 {
    // m must be prime!
    mod_pow(a, m - 2, m)
}

#[derive(Debug, Clone)]
struct Transformation {
    // ax + b mod m
    a: i128,
    b: i128,
    m: i128,
}

impl Transformation {
    fn then(self, next: Transformation) -> Transformation {
        // apply self then next to some x
        let Transformation {
            a: a1,
            b: b1,
            m: m1,
        } = self;
        let Transformation {
            a: a2,
            b: b2,
            m: m2,
        } = next;
        assert_eq!(m1, m2);
        // a2*(a1*x + b1) + b2 = (a2*a1)x + (a2*b1 + b2) mod m
        Transformation {
            a: (a2 * a1) % m1,
            b: (a2 * b1 + b2) % m1,
            m: m1,
        }
    }

    fn repeat(self, times: u64) -> Transformation {
        // (ax +b)^n
        if times == 1 {
            return self;
        }

        if times % 2 == 0 {
            let half = times / 2;
            let half_transform = self.repeat(half);
            half_transform.clone().then(half_transform)
        } else {
            let half = (times - 1) / 2;
            let half_transform = self.clone().repeat(half);
            half_transform.clone().then(half_transform).then(self)
        }
    }

    fn apply(self, x: i128) -> i128 {
        let res = (self.a * x + self.b) % self.m;
        if res < 0 {
            res + self.m
        } else {
            res
        }
    }
}

fn deal_into_new_stack(m: i128) -> Transformation {
    Transformation { a: -1, b: m - 1, m }
}
fn cut(n: i128, m: i128) -> Transformation {
    Transformation {
        a: 1,
        b: (m + n) % m,
        m,
    }
}
fn deal_with_increment(n: i128, m: i128) -> Transformation {
    Transformation {
        a: mod_mult_inv(n, m),
        b: 0,
        m,
    }
}

fn shuffles_transformation(shuffles: &[Shuffle], len: i128) -> Transformation {
    shuffles.iter().fold(
        Transformation { a: 1, b: 0, m: len },
        |transformation, shuffle| {
            match shuffle {
                Shuffle::DealIntoNewStack => deal_into_new_stack(len),
                Shuffle::Cut(n) => cut(*n as i128, len),
                Shuffle::DealWithIncrement(n) => deal_with_increment(*n as i128, len),
            }
            .then(transformation)
        },
    )
}

const PT2_LEN: i128 = 119_315_717_514_047;
const PT2_REPETITIONS: u64 = 101_741_582_076_661;

fn part2(shuffles: &[Shuffle]) {
    let transformation = shuffles_transformation(shuffles, PT2_LEN);
    let transformation = transformation.repeat(PT2_REPETITIONS);
    let pos2020 = transformation.apply(2020);
    println!("Part 2: card = {}", pos2020);
}

fn main() {
    let input = include_str!("input.txt");
    let shuffles = parse_shuffles(input);
    part1(&shuffles);
    part2(&shuffles);
}
