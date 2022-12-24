use std::time::Instant;

// const INPUT: &str = TEST;
// const N: usize = 4;
// const fn lookup(grid_x: usize, grid_y: usize, dir_ix: usize) -> (usize, usize, usize) {
//     test_lookup(grid_x, grid_y, dir_ix)
// }

const INPUT: &str = REAL;
// this is massively cheating, but what can you do
const N: usize = 50;
const fn lookup(grid_x: usize, grid_y: usize, dir_ix: usize) -> (usize, usize, usize) {
    real_lookup(grid_x, grid_y, dir_ix)
}

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)]; // R D L U

fn main() {
    let start_time = Instant::now();
    let p1 = p1();
    let p1_time = Instant::now();
    let p2 = p2();
    let p2_time = Instant::now();

    println!("p1: {}", p1);
    println!("{:?}", p1_time - start_time); // 388.8µs
    println!("p2: {}", p2);
    println!("{:?}", p2_time - p1_time); // 359.4µs
}

fn p1() -> usize {
    let (tiles, rows, cols, mut pos, instrs) = parse_input_p1();

    let mut dir_ix = 0; // right

    let mut j = 0;
    for instr in &instrs[..instrs.len() - 1] {
        // lose end linebreak
        let digit = instr - b'0';
        if (0..=9).contains(&digit) {
            j *= 10;
            j += digit;
        } else {
            // L or R
            // move x spaces
            // then change dir
            // then x = 0
            p1_move(j, &mut pos, dir_ix, &rows, &cols, &tiles);
            j = 0;
            if *instr == b'L' {
                dir_ix = (dir_ix + DIRS.len() - 1) % DIRS.len();
            } else {
                // if *instr == b'R' {
                dir_ix = (dir_ix + 1) % DIRS.len();
            }
        }
    }

    // if we end with a number, still gotta move
    p1_move(j, &mut pos, dir_ix, &rows, &cols, &tiles);

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir_ix
}

fn p1_move(
    j: u8,
    pos: &mut (usize, usize),
    dir_ix: usize,
    rows: &Vec<(usize, usize)>,
    cols: &Vec<(usize, usize)>,
    tiles: &Vec<Vec<char>>,
) {
    for _ in 0..j {
        let (new_x, new_y) = p1_new_coord(*pos, dir_ix, rows, cols);
        if tiles[new_y][new_x] == '.' {
            *pos = (new_x, new_y);
        } else {
            // can't move any more
            break;
        }
    }
}

// FIXME: do we want to do a vec approach instead?

fn p2() -> usize {
    let (tiles, mut pos, instrs) = parse_input_p2();

    let mut dir_ix = 0; // right

    let mut j = 0;
    for instr in &instrs[..instrs.len() - 1] {
        // lose end linebreak
        let digit = instr - b'0';
        if (0..=9).contains(&digit) {
            j *= 10;
            j += digit;
        } else {
            p2_move(j, &mut pos, &mut dir_ix, &tiles);
            j = 0;
            if *instr == b'L' {
                dir_ix = (dir_ix + DIRS.len() - 1) % DIRS.len();
            } else {
                // if *instr == b'R' {
                dir_ix = (dir_ix + 1) % DIRS.len();
            }
        }
    }

    // if we end with a number, still gotta move
    p2_move(j, &mut pos, &mut dir_ix, &tiles);

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir_ix
}

fn p2_move(j: u8, pos: &mut (usize, usize), dir_ix: &mut usize, tiles: &Vec<Vec<char>>) {
    for _ in 0..j {
        let (new_x, new_y, new_dir_ix) = p2_new_coord_and_dir_ix(*pos, *dir_ix);
        if tiles[new_y][new_x] == '.' {
            *pos = (new_x, new_y);
            *dir_ix = new_dir_ix;
        } else {
            // can't move any more
            break;
        }
    }
}

fn p1_new_coord(
    pos: (usize, usize),
    dir_ix: usize,
    rows: &Vec<(usize, usize)>,
    cols: &Vec<(usize, usize)>,
) -> (usize, usize) {
    // this relies on the fact that you can't move diag, so
    // if you wrap x then your y isn't changing and vice versa
    let (x_start, x_len) = rows[pos.1];
    let (y_start, y_len) = cols[pos.0];

    let relative_x = pos.0 - x_start + x_len;
    let relative_y = pos.1 - y_start + y_len;

    let new_relative_x = relative_x.saturating_add_signed(DIRS[dir_ix].0) % x_len;
    let new_relative_y = relative_y.saturating_add_signed(DIRS[dir_ix].1) % y_len;

    let new_x = new_relative_x + x_start;
    let new_y = new_relative_y + y_start;
    (new_x, new_y)
}

fn p2_new_coord_and_dir_ix(pos: (usize, usize), dir_ix: usize) -> (usize, usize, usize) {
    let relative_x = pos.0 % N;
    let grid_x = pos.0 / N;
    let relative_y = pos.1 % N;
    let grid_y = pos.1 / N;

    if (dir_ix == 0 && relative_x == N - 1)
        || (dir_ix == 1 && relative_y == N - 1)
        || (dir_ix == 2 && relative_x == 0)
        || (dir_ix == 3 && relative_y == 0)
    {
        // we're leaving a grid

        let (new_grid_x, new_grid_y, new_dir_ix) = lookup(grid_x, grid_y, dir_ix);

        let rots = (new_dir_ix + DIRS.len() - dir_ix) % DIRS.len();
        let (new_relative_x, new_relative_y) = match rots {
            0 => (relative_x, relative_y),
            // rot 180
            2 => (N - 1 - relative_x, N - 1 - relative_y),
            // rot 90
            1 => (N - 1 - relative_y, relative_x),
            // rot -90
            3 => (relative_y, N - 1 - relative_x),
            _ => unreachable!(),
        };
        // which edge do we enter from?
        let (new_relative_x, new_relative_y) = match new_dir_ix {
            0 => (0, new_relative_y),
            1 => (new_relative_x, 0),
            2 => (N - 1, new_relative_y),
            3 => (new_relative_x, N - 1),
            _ => unreachable!(),
        };

        let new_x = new_relative_x + new_grid_x * N;
        let new_y = new_relative_y + new_grid_y * N;
        (new_x, new_y, new_dir_ix)
    } else {
        // phew we're not leaving a grid so we can just continue as normal
        let new_relative_x = relative_x.saturating_add_signed(DIRS[dir_ix].0) % N;
        let new_relative_y = relative_y.saturating_add_signed(DIRS[dir_ix].1) % N;
        let new_x = new_relative_x + grid_x * N;
        let new_y = new_relative_y + grid_y * N;
        (new_x, new_y, dir_ix)
    }
}

// test:
//                0,0,1
//
// 0,-1,0  1,0,0  0,1,0
//
//                0,0,-1  -1,0,0
const fn test_lookup(grid_x: usize, grid_y: usize, dir_ix: usize) -> (usize, usize, usize) {
    match (grid_x, grid_y, dir_ix) {
        (2, 0, 0) => (3, 2, 2),
        (2, 0, 1) => (2, 1, 1),
        (2, 0, 2) => (1, 1, 1),
        (2, 0, 3) => (2, 2, 2),
        (0, 1, 0) => (1, 1, 0),
        (0, 1, 1) => (2, 2, 3),
        (0, 1, 2) => (3, 2, 3),
        (0, 1, 3) => (2, 0, 1),
        (1, 1, 0) => (2, 1, 0),
        (1, 1, 1) => (2, 2, 0),
        (1, 1, 2) => (0, 1, 2),
        (1, 1, 3) => (2, 0, 0),
        (2, 1, 0) => (3, 2, 1),
        (2, 1, 1) => (2, 2, 1),
        (2, 1, 2) => (1, 1, 2),
        (2, 1, 3) => (2, 0, 3),
        (2, 2, 0) => (3, 2, 0),
        (2, 2, 1) => (0, 1, 3),
        (2, 2, 2) => (1, 1, 3),
        (2, 2, 3) => (2, 1, 3),
        (3, 2, 0) => (2, 0, 2),
        (3, 2, 1) => (0, 1, 0),
        (3, 2, 2) => (2, 2, 2),
        (3, 2, 3) => (2, 1, 2),
        _ => unreachable!(),
    }
}

// real:
//          0,0,1  0,1,0
//          1,0,0
//  0,-1,0  0,0,-1
//  -1,0,0
const fn real_lookup(grid_x: usize, grid_y: usize, dir_ix: usize) -> (usize, usize, usize) {
    match (grid_x, grid_y, dir_ix) {
        (1, 0, 0) => (2, 0, 0),
        (1, 0, 1) => (1, 1, 1),
        (1, 0, 2) => (0, 2, 0),
        (1, 0, 3) => (0, 3, 0),
        (2, 0, 0) => (1, 2, 2),
        (2, 0, 1) => (1, 1, 2),
        (2, 0, 2) => (1, 0, 2),
        (2, 0, 3) => (0, 3, 3),
        (1, 1, 0) => (2, 0, 3),
        (1, 1, 1) => (1, 2, 1),
        (1, 1, 2) => (0, 2, 1),
        (1, 1, 3) => (1, 0, 3),
        (0, 2, 0) => (1, 2, 0),
        (0, 2, 1) => (0, 3, 1),
        (0, 2, 2) => (1, 0, 0),
        (0, 2, 3) => (1, 1, 0),
        (1, 2, 0) => (2, 0, 2),
        (1, 2, 1) => (0, 3, 2),
        (1, 2, 2) => (0, 2, 2),
        (1, 2, 3) => (1, 1, 3),
        (0, 3, 0) => (1, 2, 3),
        (0, 3, 1) => (2, 0, 1),
        (0, 3, 2) => (1, 0, 1),
        (0, 3, 3) => (0, 2, 3),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loops() {
        for dir_ix in 0..4 {
            for (grid_x, grid_y) in [(1, 0), (2, 0), (1, 1), (0, 2), (1, 2), (0, 3)] {
                let (grid_x_1, grid_y_1, dir_ix_1) = real_lookup(grid_x, grid_y, dir_ix);
                let (grid_x_2, grid_y_2, dir_ix_2) = real_lookup(grid_x_1, grid_y_1, dir_ix_1);
                let (grid_x_3, grid_y_3, dir_ix_3) = real_lookup(grid_x_2, grid_y_2, dir_ix_2);
                let (grid_x_4, grid_y_4, dir_ix_4) = real_lookup(grid_x_3, grid_y_3, dir_ix_3);
                assert_eq!((grid_x, grid_y, dir_ix), (grid_x_4, grid_y_4, dir_ix_4));
            }
        }
    }
}

fn parse_input_p1() -> (
    Vec<Vec<char>>,
    Vec<(usize, usize)>,
    Vec<(usize, usize)>,
    (usize, usize),
    &'static [u8],
) {
    // 0,0 is top left
    let mut tiles: Vec<Vec<char>> = vec![];
    // for each x, what ix does it start and where does it end
    // todo: could we use slices or overlapping vecs here?
    let mut rows: Vec<(usize, usize)> = vec![];
    let mut cols: Vec<(usize, usize)> = vec![];
    let mut pos = (0usize, 0usize);
    let mut sections = INPUT.split("\n\n");
    let section_1 = sections.next().unwrap();
    let section_2 = sections.next().unwrap();
    for (y, line) in section_1.lines().enumerate() {
        tiles.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            // found some new cols, catch up.
            // need to do this because the input
            // got end-line blank space trimmed lol
            while cols.len() < x {
                cols.push((0, 0))
            }
            tiles.last_mut().unwrap().push(c);
            if c != ' ' {
                if pos.0 == 0 && pos.1 == 0 && c == '.' {
                    pos = (x, y);
                }
                // extend x_range and y_range
                if let Some((_x_start, x_len)) = rows.get_mut(y) {
                    *x_len += 1;
                } else {
                    rows.push((x, 1));
                }
                if x == cols.len() {
                    cols.push((y, 1));
                } else {
                    let (y_start, y_len) = &mut cols[x];
                    if *y_start + *y_len == y {
                        *y_len += 1;
                    } else {
                        *y_start = y;
                        *y_len = 1;
                    }
                }
            }
        }
    }
    (tiles, rows, cols, pos, section_2.as_bytes())
}

fn parse_input_p2() -> (Vec<Vec<char>>, (usize, usize), &'static [u8]) {
    // 0,0 is top left
    let mut tiles: Vec<Vec<char>> = vec![];
    // for each x, what ix does it start and where does it end
    // todo: could we use slices or overlapping vecs here?
    let mut pos = (0usize, 0usize);
    let mut sections = INPUT.split("\n\n");
    let section_1 = sections.next().unwrap();
    let section_2 = sections.next().unwrap();
    for (y, line) in section_1.lines().enumerate() {
        tiles.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            // found some new cols, catch up.
            // need to do this because the input
            // got end-line blank space trimmed lol
            tiles.last_mut().unwrap().push(c);
            if c != ' ' {
                if pos.0 == 0 && pos.1 == 0 && c == '.' {
                    pos = (x, y);
                }
            }
        }
    }
    (tiles, pos, section_2.as_bytes())
}

const REAL: &str = include_str!("real.txt");

const TEST: &str = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";
