mod emulator;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }

    // returns (row, col)
    fn to_delta(self) -> (i64, i64) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

// image handling

struct Image {
    map: Vec<Vec<bool>>, // true = scaffolding
    vacuum_row: usize,
    vacuum_col: usize,
    vacuum_dir: Direction,
}

impl Image {
    fn scaffold(&self, row: i64, col: i64) -> bool {
        if row < 0 || col < 0 {
            return false;
        }
        self.map
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
            .cloned()
            .unwrap_or(false)
    }
}

fn get_image(emu: &mut emulator::Emulator) -> Result<Image, &'static str> {
    let mut map = Vec::new();
    let mut row = Box::new(Vec::new());

    let mut vacuum_row = 0;
    let mut vacuum_col = 0;
    let mut vacuum_dir = Direction::Left;

    loop {
        let halted = emu.step(
            || Err("no input"),
            |i| {
                match (i as u8) as char {
                    '.' => row.push(false),
                    '#' => row.push(true),
                    '^' | 'v' | '<' | '>' => {
                        vacuum_col = row.len();
                        vacuum_row = map.len();
                        vacuum_dir = match (i as u8) as char {
                            '^' => Direction::Up,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            '>' => Direction::Right,
                            _ => unreachable!(),
                        };
                        row.push(true);
                    }
                    '\n' => {
                        if row.len() > 0 {
                            map.push(std::mem::replace(&mut *row, Vec::new()));
                        }
                    }
                    _ => return Err("invalid pixel"),
                }
                Ok(())
            },
        )?;

        if halted {
            break;
        }
    }

    Ok(Image {
        map,
        vacuum_row,
        vacuum_col,
        vacuum_dir,
    })
}

fn sum_intersection_alignment_params(map: &[Vec<bool>]) -> usize {
    let mut res = 0;

    let height = map.len();
    let width = map[0].len();
    for row in 1..height - 1 {
        for col in 1..width - 1 {
            if map[row][col]
                && map[row - 1][col]
                && map[row + 1][col]
                && map[row][col - 1]
                && map[row][col + 1]
            {
                res += row * col;
            }
        }
    }

    res
}

// path finding

#[derive(Debug, PartialEq)]
enum Instruction {
    RotateLeft,
    RotateRight,
    Move(usize),
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::RotateLeft => "L".to_string(),
            Instruction::RotateRight => "R".to_string(),
            Instruction::Move(steps) => steps.to_string(),
        }
    }
}

// returns (instruction, new_direction)
fn find_rotation(
    image: &Image,
    dir: Direction,
    row: i64,
    col: i64,
) -> (Option<Instruction>, Direction) {
    // try left
    let (dr, dc) = dir.left().to_delta();
    if image.scaffold(row + dr, col + dc) {
        return (Some(Instruction::RotateLeft), dir.left());
    }

    // try right
    let (dr, dc) = dir.right().to_delta();
    if image.scaffold(row + dr, col + dc) {
        return (Some(Instruction::RotateRight), dir.right());
    }

    (None, dir)
}

// returns (steps, final_row, final_col)
fn steps_foward(image: &Image, dir: Direction, mut row: i64, mut col: i64) -> (usize, i64, i64) {
    let mut steps = 0;
    loop {
        let (dr, dc) = dir.to_delta();
        let (nr, nc) = (row + dr, col + dc);
        if !image.scaffold(nr, nc) {
            return (steps, row, col);
        }

        steps += 1;
        row = nr;
        col = nc;
    }
}

fn find_path(image: &Image) -> Vec<Instruction> {
    let mut res = Vec::new();
    // find initial rotation:
    let (initial_rot, mut dir) = find_rotation(
        image,
        image.vacuum_dir,
        image.vacuum_row as i64,
        image.vacuum_col as i64,
    );

    res.push(initial_rot.expect("failed to find initial rotation"));

    // move forward then rotate until done
    let mut row = image.vacuum_row as i64;
    let mut col = image.vacuum_col as i64;
    loop {
        let (steps, final_row, final_col) = steps_foward(image, dir, row, col);
        res.push(Instruction::Move(steps));
        row = final_row;
        col = final_col;

        let (rot, new_dir) = find_rotation(image, dir, row, col);
        match rot {
            Some(i) => res.push(i),
            None => return res,
        }
        dir = new_dir;
    }
}

// movement routine generation

struct PathProgram {
    main: String,
    a: String,
    b: String,
    c: String,
}

fn split<'a, T: PartialEq>(mut path: &'a [T], delim: &[T]) -> Vec<&'a [T]> {
    let mut res = Vec::new();
    let mut current_index = 0;
    while path.len() - current_index >= delim.len() {
        if &path[current_index..current_index + delim.len()] == delim {
            if current_index > 0 {
                res.push(&path[..current_index]);
            }
            path = &path[current_index + delim.len()..];
            current_index = 0;
        } else {
            current_index += 1;
        }
    }

    if !path.is_empty() {
        res.push(path);
    }

    res
}

fn compress_path(path: &[Instruction], max_chars: usize) -> Option<PathProgram> {
    for a_start in 0..path.len() {
        for a_end in a_start + 1..=path.len() {
            let a = &path[a_start..a_end];
            if str_len(a) > max_chars {
                break;
            }

            let remaining_chunks = split(path, a);

            for chunk_i in 0..remaining_chunks.len() {
                let chunk = remaining_chunks[chunk_i];
                let prev_chunks = &remaining_chunks[..chunk_i];
                let later_chunks = &remaining_chunks[chunk_i + 1..];
                for b_start in 0..chunk.len() {
                    for b_end in b_start + 1..=chunk.len() {
                        let b = &chunk[b_start..b_end];

                        if str_len(b) > max_chars {
                            break;
                        }

                        let mut remaining_chunks = split(chunk, b);
                        remaining_chunks
                            .extend(prev_chunks.iter().flat_map(|chunk| split(chunk, b)));
                        remaining_chunks
                            .extend(later_chunks.iter().flat_map(|chunk| split(chunk, b)));

                        let first_remaining = remaining_chunks[0];
                        for c_start in 0..first_remaining.len() {
                            for c_end in c_start + 1..=first_remaining.len() {
                                let c = &first_remaining[c_start..c_end];

                                if str_len(c) > max_chars {
                                    break;
                                }

                                if remaining_chunks
                                    .iter()
                                    .all(|chunk| split(chunk, c).is_empty())
                                {
                                    let main_str = to_ascii(path);
                                    let a_str = to_ascii(a);
                                    let b_str = to_ascii(b);
                                    let c_str = to_ascii(c);

                                    let main_str = main_str
                                        .replace(&a_str, "A")
                                        .replace(&b_str, "B")
                                        .replace(&c_str, "C");

                                    if main_str.len() <= max_chars {
                                        return Some(PathProgram {
                                            main: main_str,
                                            a: a_str,
                                            b: b_str,
                                            c: c_str,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn to_ascii<T: ToString>(path: &[T]) -> String {
    path.iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn str_len<T: ToString>(path: &[T]) -> usize {
    to_ascii(path).len()
}

fn exec_path_program(
    emu: &mut emulator::Emulator,
    path_program: PathProgram,
) -> Result<i64, &'static str> {
    let mut res = Box::new(None);
    let input = format!(
        "{}\n{}\n{}\n{}\nn\n",
        path_program.main, path_program.a, path_program.b, path_program.c
    );
    let mut chars = input.chars();
    while !emu.step(
        || chars.next().ok_or("need more input").map(|c| c as i64),
        |output| {
            if output > 128 {
                *res = Some(output);
            }
            Ok(())
        },
    )? {}

    res.ok_or("no non-ASCII output")
}

fn main() {
    let input = include_str!("input.txt");
    let program = emulator::Program::new(input).expect("parsing failed");

    // part 1
    let mut emu = emulator::Emulator::new(program.clone());
    let image = get_image(&mut emu).expect("failed to get image");
    let sum = sum_intersection_alignment_params(&image.map);
    println!("Part 1: sum of alignment params = {}", sum);

    // part 2
    let mut emu = emulator::Emulator::new(program);
    emu.store(0, 2);
    let path = find_path(&image);
    let path_program = compress_path(&path, 20).expect("failed to compress path");
    let dust_collected =
        exec_path_program(&mut emu, path_program).expect("failed to count dust collected");
    println!("Part 2: dust collected = {}", dust_collected);
}

#[cfg(test)]
mod tests {
    use super::split;

    #[test]
    fn test_split() {
        let instrs = [1, 2, 3, 4, 5, 3, 2, 1, 4, 5, 1];
        let delim = [4, 5];
        let res = split(&instrs, &delim);
        let expected: Vec<&[i32]> = vec![&[1, 2, 3], &[3, 2, 1], &[1]];
        assert_eq!(res, expected);

        let instrs = [1, 1, 1];
        let delim = [1];
        let res = split(&instrs, &delim);
        let expected: Vec<&[i32]> = vec![];
        assert_eq!(res, expected);
    }
}
