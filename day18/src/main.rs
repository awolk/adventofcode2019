use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

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
                b'A'..=b'Z' => Item::Door(chr.to_ascii_lowercase()),
                _ => panic!("invalid entry"),
            };

            map.insert((row, col), item);
        }
    }

    Maze {
        map,
        entrances,
        keys,
    }
}

pub trait Node: Sized {
    fn neighbors(&self) -> Vec<Self>;
}

fn dijkstra<T: Node + Eq + Hash + Ord>(start: T, mut done: impl FnMut(&T) -> bool) -> Option<T> {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(start);

    while let Some(node) = queue.pop() {
        if seen.contains(&node) {
            continue;
        }

        if done(&node) {
            return Some(node);
        }

        for neighbor in node.neighbors() {
            queue.push(neighbor);
        }

        seen.insert(node);
    }

    None
}

#[derive(Clone)]
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
impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // reverse order so that BinaryHeap behaves as a min-heap
        Some(other.steps.cmp(&self.steps))
    }
}
impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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

    fn with_key(&self, robot: usize, key: u8, dist: usize) -> Self {
        let mut positions = self.positions.clone();
        positions[robot] = self.maze.keys[&key];
        Self {
            positions,
            inventory: self.inventory | (1 << (key - b'a') as u32),
            steps: self.steps + dist,
            maze: self.maze,
        }
    }

    fn has_key(&self, key: u8) -> bool {
        (self.inventory & (1 << (key - b'a') as u32)) != 0
    }

    fn is_final(&self) -> bool {
        self.inventory == 0b11_1111_1111_1111_1111_1111_1111
    }

    fn find_reachable_from_pos(&self, pos: (usize, usize)) -> HashMap<u8, usize> {
        let mut res: HashMap<u8, usize> = HashMap::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
        queue.push_back((pos, 0));
        while let Some((pos, dist)) = queue.pop_front() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);

            let item = self.maze.map.get(&pos).unwrap_or(&Item::Wall);
            match item {
                Item::Wall => continue,
                Item::Door(key) if !self.has_key(*key) => continue,
                Item::Key(key) if !self.has_key(*key) => {
                    res.entry(*key).or_insert(dist);
                }
                _ => {}
            }

            let (r, c) = pos;
            for neighbor in &[(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
                queue.push_back((*neighbor, dist + 1));
            }
        }

        res
    }
}

impl<'a> Node for State<'a> {
    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        for (robot, pos) in self.positions.iter().enumerate() {
            let reachable_keys = self.find_reachable_from_pos(*pos);
            for (key, dist) in reachable_keys.into_iter() {
                neighbors.push(self.with_key(robot, key, dist));
            }
        }
        neighbors
    }
}

fn time<T>(func: impl FnOnce() -> T) -> (Duration, T) {
    let start = Instant::now();
    let res = func();
    let duration = start.elapsed();
    (duration, res)
}

fn main() {
    let input = include_str!("input.txt");
    let mut maze = parse_input(input);

    let start_state = State::new(&maze);
    let (duration, final_state) =
        time(|| dijkstra(start_state, State::is_final).expect("failed to find result"));
    println!(
        "Part 1: steps = {} in {}s",
        final_state.steps,
        duration.as_secs()
    );

    maze.convert_entrance();
    let start_state = State::new(&maze);
    let (duration, final_state) =
        time(|| dijkstra(start_state, State::is_final).expect("failed to find result"));
    println!(
        "Part 2: steps = {} in {}s",
        final_state.steps,
        duration.as_secs()
    );
}
