use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone)]
enum Item {
    Entrance,
    Passage,
    Wall,
    Key(u8),
    Door(u8),
}

struct Maze {
    map: HashMap<(usize, usize), Item>,
    entrances: Vec<(usize, usize)>,
    keys: HashMap<u8, (usize, usize)>,
    doors: HashMap<u8, (usize, usize)>,
}

impl Maze {
    fn convert_entrance(&mut self) {
        assert_eq!(self.entrances.len(), 1);
        let (sr, sc) = self.entrances[0];
        self.map.insert((sr, sc), Item::Wall);
        self.map.insert((sr + 1, sc), Item::Wall);
        self.map.insert((sr - 1, sc), Item::Wall);
        self.map.insert((sr, sc + 1), Item::Wall);
        self.map.insert((sr, sc - 1), Item::Wall);
        self.map.insert((sr + 1, sc + 1), Item::Entrance);
        self.map.insert((sr + 1, sc - 1), Item::Entrance);
        self.map.insert((sr - 1, sc + 1), Item::Entrance);
        self.map.insert((sr - 1, sc - 1), Item::Entrance);

        self.entrances = vec![
            (sr + 1, sc + 1),
            (sr + 1, sc - 1),
            (sr - 1, sc + 1),
            (sr - 1, sc - 1),
        ];
    }
}

fn parse_input(input: &str) -> Maze {
    let mut entrances = Vec::new();
    let mut map = HashMap::new();
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, chr) in line.bytes().enumerate() {
            let item = match chr {
                b'@' => {
                    entrances.push((row, col));
                    Item::Entrance
                }
                b'.' => Item::Passage,
                b'#' => Item::Wall,
                b'a'..=b'z' => {
                    keys.insert(chr, (row, col));
                    Item::Key(chr)
                }
                b'A'..=b'Z' => {
                    doors.insert(chr.to_ascii_lowercase(), (row, col));
                    Item::Door(chr.to_ascii_lowercase())
                }
                _ => panic!("invalid entry"),
            };

            map.insert((row, col), item);
        }
    }

    Maze {
        map,
        entrances,
        keys,
        doors,
    }
}

pub trait Node: Sized {
    fn neighbors(&self) -> Vec<Self>;
}

fn breadth_first_search<T: Node + Eq + Hash>(
    start: T,
    mut done: impl FnMut(&T) -> bool,
) -> Option<T> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if seen.contains(&node) {
            continue;
        }

        for neighbor in node.neighbors() {
            if done(&neighbor) {
                return Some(neighbor);
            }

            queue.push_back(neighbor);
        }

        seen.insert(node);
    }

    None
}

struct State<'a> {
    positions: Vec<(usize, usize)>,
    inventory: u32,
    steps: usize,
    maze: &'a Maze,
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions && self.inventory == other.inventory
    }
}

impl<'a> Eq for State<'a> {}

impl<'a> Hash for State<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.positions.hash(state);
        self.inventory.hash(state);
    }
}

impl<'a> State<'a> {
    fn new(maze: &'a Maze) -> Self {
        Self {
            positions: maze.entrances.clone(),
            inventory: 0,
            steps: 0,
            maze,
        }
    }

    fn with_pos(&self, robot: usize, position: (usize, usize)) -> Self {
        let mut positions = self.positions.clone();
        positions[robot] = position;
        Self {
            positions,
            inventory: self.inventory,
            steps: self.steps + 1,
            maze: self.maze,
        }
    }

    fn with_pos_and_key(&self, robot: usize, position: (usize, usize), key: u8) -> Self {
        let mut positions = self.positions.clone();
        positions[robot] = position;
        Self {
            positions,
            inventory: self.inventory | (1 << (key - b'a') as u32),
            steps: self.steps + 1,
            maze: self.maze,
        }
    }

    fn has_key(&self, key: u8) -> bool {
        (self.inventory & (1 << (key - b'a') as u32)) != 0
    }

    fn is_final(&self) -> bool {
        self.inventory == 0b11_1111_1111_1111_1111_1111_1111
    }
}

impl<'a> Node for State<'a> {
    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::with_capacity(4);
        for (robot, &(r, c)) in self.positions.iter().enumerate() {
            for &(pr, pc) in &[(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
                let item = self.maze.map.get(&(pr, pc)).unwrap_or(&Item::Wall);
                let can_move_to = match item {
                    Item::Wall => false,
                    Item::Door(chr) => self.has_key(*chr),
                    _ => true,
                };

                if can_move_to {
                    neighbors.push(if let Item::Key(chr) = item {
                        self.with_pos_and_key(robot, (pr, pc), *chr)
                    } else {
                        self.with_pos(robot, (pr, pc))
                    });
                }
            }
        }
        neighbors
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut maze = parse_input(input);
    let start_state = State::new(&maze);
    let final_state =
        breadth_first_search(start_state, State::is_final).expect("failed to find result");
    println!("Part 1: steps = {}", final_state.steps);

    maze.convert_entrance();
    let start_state = State::new(&maze);
    let final_state =
        breadth_first_search(start_state, State::is_final).expect("failed to find result");
    println!("Part 2: steps = {}", final_state.steps);
}
