use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone)]
enum Item {
    Entrance,
    Passage,
    Wall,
    Key(char),
    Door(char),
}

struct State {
    position: (usize, usize),
    inventory: u32,
    steps: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.inventory == other.inventory
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.inventory.hash(state);
    }
}

impl State {
    fn with_key(&self, key: char) -> Self {
        Self {
            position: self.position,
            inventory: self.inventory | (1 << (key as u8 - b'a') as u32),
            steps: self.steps,
        }
    }

    fn has_key(&self, key: char) -> bool {
        (self.inventory & (1 << (key as u8 - b'a') as u32)) != 0
    }

    fn new(position: (usize, usize)) -> Self {
        Self {
            position,
            inventory: 0,
            steps: 0,
        }
    }

    fn move_to(&self, position: (usize, usize)) -> Self {
        Self {
            position,
            inventory: self.inventory,
            steps: self.steps + 1,
        }
    }

    fn is_final(&self) -> bool {
        self.inventory == 0b11_1111_1111_1111_1111_1111_1111
    }
}

fn get(map: &HashMap<(usize, usize), Item>, row: usize, col: usize) -> Item {
    map.get(&(row, col)).cloned().unwrap_or(Item::Wall)
}

fn main() {
    let input = include_str!("input.txt");
    let mut start = (0, 0);
    let mut map = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            let item = match chr {
                '@' => Item::Entrance,
                '.' => Item::Passage,
                '#' => Item::Wall,
                'a'..='z' => Item::Key(chr),
                'A'..='Z' => Item::Door(chr.to_ascii_lowercase()),
                _ => panic!("invalid entry"),
            };

            if item == Item::Entrance {
                start = (row, col);
            }

            map.insert((row, col), item);
        }
    }

    // depth first search
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(State::new(start));

    while let Some(state) = queue.pop_front() {
        if seen.contains(&state) {
            continue;
        }

        let (r, c) = state.position;
        for &(pr, pc) in &[(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
            let item = get(&map, pr, pc);
            let can_move_to = match item {
                Item::Wall => false,
                Item::Door(chr) => state.has_key(chr),
                _ => true,
            };

            if can_move_to {
                let new_state = if let Item::Key(chr) = item {
                    state.move_to((pr, pc)).with_key(chr)
                } else {
                    state.move_to((pr, pc))
                };

                if new_state.is_final() {
                    println!("Part 1: steps = {}", new_state.steps);
                    return;
                }

                queue.push_back(new_state);
            }
        }

        seen.insert(state);
    }
}
