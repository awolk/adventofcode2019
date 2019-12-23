use std::ops::Index;

struct Deck {
    cards: Vec<u16>,
}

impl Deck {
    fn new() -> Deck {
        Deck {
            cards: (0..10007).collect(),
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

    fn deal_with_increment(&self, n: usize) -> Deck {
        let mut cards = vec![0; 10007];
        let mut pos = 0;
        for &card in &self.cards {
            cards[pos] = card;
            pos = (pos + n) % 10007;
        }
        Deck { cards }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut deck = Deck::new();
    for line in input.lines() {
        if line == "deal into new stack" {
            deck = deck.deal_into_new_stack();
        } else if line.starts_with("cut ") {
            let n: i64 = line.trim_start_matches("cut ").parse().unwrap();
            deck = deck.cut(n);
        } else if line.starts_with("deal with increment ") {
            let n: usize = line
                .trim_start_matches("deal with increment ")
                .parse()
                .unwrap();
            deck = deck.deal_with_increment(n);
        } else {
            panic!("invalid line: {}", line)
        }
    }
    println!(
        "Part 1: position = {}",
        deck.cards.iter().position(|&card| card == 2019).unwrap()
    );
}
