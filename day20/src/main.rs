mod vector;
use self::vector::Vec2;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Eq, PartialEq)]
enum Entry {
    Wall,
    Passage,
    Portal(u8, u8, PortalType),
}

#[derive(Eq, PartialEq, Debug)]
enum PortalType {
    Inner,
    Outer,
}

struct Maze {
    grid: HashMap<Vec2, Entry>,
    portal_dests: HashMap<(u8, u8), Vec<Vec2>>,
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let byte_grid: HashMap<Vec2, u8> = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.bytes()
                    .enumerate()
                    .map(move |(col, byte)| (Vec2(row as i64, col as i64), byte))
            })
            .collect();

        let mut portal_dests = HashMap::new();
        let mut grid = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            for (col, byte) in line.bytes().enumerate() {
                let pos = Vec2(row as i64, col as i64);
                match byte {
                    b'#' | b' ' => {
                        grid.insert(pos, Entry::Wall);
                    }
                    b'.' => {
                        grid.insert(pos, Entry::Passage);
                    }
                    b'A'..=b'Z' => {
                        // try lower entry
                        let under_pos = pos + Vec2(1, 0);
                        let under = byte_grid.get(&under_pos).unwrap_or(&b'#');
                        if (b'A'..=b'Z').contains(under) {
                            let portals_entry =
                                portal_dests.entry((byte, *under)).or_insert_with(Vec::new);
                            // find dest (either above pos or below under)
                            let above_pos = pos - Vec2(1, 0);
                            let above_val = *byte_grid.get(&above_pos).unwrap_or(&b'#');
                            if above_val == b'.' {
                                // dest is above pos
                                portals_entry.push(above_pos);
                                let pt = if byte_grid.get(&(under_pos + Vec2(1, 0))).is_none() {
                                    PortalType::Outer
                                } else {
                                    PortalType::Inner
                                };
                                grid.insert(pos, Entry::Portal(byte, *under, pt));
                            } else {
                                // dest should be below under
                                let below_under_pos = under_pos + Vec2(1, 0);
                                let below_under_val =
                                    *byte_grid.get(&below_under_pos).unwrap_or(&b'#');
                                assert_eq!(below_under_val, b'.');
                                portals_entry.push(below_under_pos);
                                let pt = if byte_grid.get(&(pos - Vec2(1, 0))).is_none() {
                                    PortalType::Outer
                                } else {
                                    PortalType::Inner
                                };
                                grid.insert(under_pos, Entry::Portal(byte, *under, pt));
                            }
                        } else {
                            // try right
                            let right_pos = pos + Vec2(0, 1);
                            let right = byte_grid.get(&right_pos).unwrap_or(&b' ');
                            if (b'A'..=b'Z').contains(right) {
                                let portals_entry =
                                    portal_dests.entry((byte, *right)).or_insert_with(Vec::new);
                                // find dest (either left of pos or right of right)
                                let left_pos = pos - Vec2(0, 1);
                                let left_val = *byte_grid.get(&left_pos).unwrap_or(&b'#');
                                if left_val == b'.' {
                                    // dest is left of pos
                                    portals_entry.push(left_pos);
                                    let pt = if byte_grid.get(&(right_pos + Vec2(0, 1))).is_none() {
                                        PortalType::Outer
                                    } else {
                                        PortalType::Inner
                                    };
                                    grid.insert(pos, Entry::Portal(byte, *right, pt));
                                } else {
                                    // dest should be right of right
                                    let right_right_pos = right_pos + Vec2(0, 1);
                                    let right_right_val =
                                        *byte_grid.get(&right_right_pos).unwrap_or(&b'#');
                                    assert_eq!(right_right_val, b'.');
                                    portals_entry.push(right_right_pos);
                                    let pt = if byte_grid.get(&(pos - Vec2(0, 1))).is_none() {
                                        PortalType::Outer
                                    } else {
                                        PortalType::Inner
                                    };
                                    grid.insert(right_pos, Entry::Portal(byte, *right, pt));
                                }
                            }
                        }
                    }
                    _ => panic!("invalid entry"),
                };
            }
        }
        Maze { grid, portal_dests }
    }
}

fn bfs(maze: &Maze) -> usize {
    let start = maze.portal_dests[&(b'A', b'A')][0];
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos);

        for npos in pos.neighbors() {
            let entry = maze.grid.get(&npos).unwrap_or(&Entry::Wall);
            match entry {
                Entry::Wall => {}
                Entry::Passage => queue.push_back((npos, dist + 1)),
                Entry::Portal(from, to, _pt) => {
                    if *from == b'Z' && *to == b'Z' {
                        return dist;
                    }
                    if let Some(dest) = maze
                        .portal_dests
                        .get(&(*from, *to))
                        .and_then(|dests| dests.iter().find(|&&dest| dest != pos))
                    {
                        queue.push_back((*dest, dist + 1));
                    }
                }
            }
        }
    }
    panic!("could not find destination");
}

fn bfs_with_levels(maze: &Maze) -> usize {
    let start = maze.portal_dests[&(b'A', b'A')][0];
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0, 0));

    while let Some((pos, level, dist)) = queue.pop_front() {
        if seen.contains(&(pos, level)) {
            continue;
        }
        seen.insert((pos, level));

        for npos in pos.neighbors() {
            let entry = maze.grid.get(&npos).unwrap_or(&Entry::Wall);
            match entry {
                Entry::Wall => {}
                Entry::Passage => queue.push_back((npos, level, dist + 1)),
                Entry::Portal(from, to, pt) => {
                    if *from == b'Z' && *to == b'Z' && level == 0 {
                        return dist;
                    }
                    if let Some(dest) = maze
                        .portal_dests
                        .get(&(*from, *to))
                        .and_then(|dests| dests.iter().find(|&&dest| dest != pos))
                    {
                        let new_level = match pt {
                            PortalType::Inner => level + 1,
                            PortalType::Outer => level - 1,
                        };
                        if new_level >= 0 {
                            queue.push_back((*dest, new_level, dist + 1));
                        }
                    }
                }
            }
        }
    }
    panic!("could not find destination");
}

fn main() {
    let input = include_str!("input.txt");
    let maze: Maze = input.into();
    let dist = bfs(&maze);
    println!("Part 1: distance = {}", dist);
    let dist = bfs_with_levels(&maze);
    println!("Part 2: distance = {}", dist);
}
