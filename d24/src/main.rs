use pathfinding::directed::astar::astar;
use std::{
    cmp::{max, min},
    time::Instant,
};

// const INPUT: &str = TEST;
const INPUT: &str = REAL;
type Grid = Vec<Vec<Vec<char>>>;

fn main() {
    let mut start_time = Instant::now();
    let mut grid = vec![];
    for line in INPUT.lines() {
        let mut row = vec![];
        for c in line.chars() {
            if c == '.' {
                row.push(vec![]);
            } else {
                row.push(vec![c]);
            }
        }
        grid.push(row);
    }

    let x_len = grid[0].len();
    let cycle_w = x_len - 2;
    let y_len = grid.len();
    let cycle_h = y_len - 2;

    let start_x = grid[0]
        .iter()
        .enumerate()
        .find(|(_, x)| x.is_empty())
        .unwrap()
        .0;
    let end_x = grid[y_len - 1]
        .iter()
        .enumerate()
        .find(|(_, x)| x.is_empty())
        .unwrap()
        .0;

    let lcm = lcm(cycle_h, cycle_w);
    let grids = prepare_grids(grid, cycle_w, cycle_h, lcm);
    // for grid in &grids {
    //     print_grid(grid);
    // }

    let prep_time = Instant::now();

    let neighbours = |&(x, y, time): &(usize, usize, usize)| {
        let time = time + 1;
        let mut neighbours = Vec::with_capacity(5);
        if is_okay_move(&grids[time % lcm][y][x]) {
            neighbours.push(((x, y, time), 1));
        }

        if x > 1 && is_okay_move(&grids[time % lcm][y][x - 1]) {
            neighbours.push(((x - 1, y, time), 1));
        }
        if y > 0 && is_okay_move(&grids[time % lcm][y - 1][x]) {
            neighbours.push(((x, y - 1, time), 1));
        }
        if x < cycle_w && is_okay_move(&grids[time % lcm][y][x + 1]) {
            neighbours.push(((x + 1, y, time), 1));
        }
        // this is the only time we want to move into the edge
        if y < cycle_h + 1 && is_okay_move(&grids[time % lcm][y + 1][x]) {
            neighbours.push(((x, y + 1, time), 1));
        }
        neighbours
    };

    let path_1_len = astar(
        &(start_x, 0, 0),
        neighbours,
        move |&(x, y, _): &(usize, usize, usize)| y_len - 1 - y + max(x, end_x) - min(x, end_x),
        move |&(x, y, _): &(usize, usize, usize)| x == end_x && y == y_len - 1,
    )
    .unwrap()
    .1;
    let p1_time = Instant::now();

    let path_2_len = astar(
        &(end_x, y_len - 1, path_1_len),
        neighbours,
        move |&(x, y, _): &(usize, usize, usize)| y + max(x, start_x) - min(x, start_x),
        move |&(x, y, _): &(usize, usize, usize)| x == start_x && y == 0,
    )
    .unwrap()
    .1;

    let path_3_len = astar(
        &(start_x, 0, path_1_len + path_2_len),
        neighbours,
        move |&(x, y, _): &(usize, usize, usize)| y_len - 1 - y + max(x, end_x) - min(x, end_x),
        move |&(x, y, _): &(usize, usize, usize)| x == end_x && y == y_len - 1,
    )
    .unwrap()
    .1;
    let p2_time = Instant::now();

    println!("p2: {}", path_1_len + path_2_len + path_3_len);
    println!("setup: {:?}", prep_time - start_time);
    println!("p1: {:?}", p1_time - start_time);
    println!("p2: {:?}", p2_time - start_time);
}

// we can only move into a space if it's empty
fn is_okay_move(c: &Vec<char>) -> bool {
    c.is_empty()
}

fn prepare_grids(grid: Grid, cycle_w: usize, cycle_h: usize, lcm: usize) -> Vec<Grid> {
    let mut grids = vec![];
    grids.push(grid);
    // could instead split up hori and verti grids
    for i in 1..lcm {
        let old_grid = &grids[i - 1];
        let mut new_grid = old_grid.clone();
        // print_grid(&new_grid);

        // clear old tornadoes
        for row in &mut new_grid[1..1 + cycle_h] {
            for c in &mut row[1..1 + cycle_w] {
                *c = vec![];
            }
        }

        for (y, row) in old_grid[1..1 + cycle_h].iter().enumerate() {
            let y = y + 1;
            for (x, cell) in row[1..1 + cycle_w].iter().enumerate() {
                let x = x + 1;
                for c in cell {
                    let (new_x, new_y) = match c {
                        '>' => {
                            let new_x = ((x - 1 + 1) % cycle_w) + 1;
                            (new_x, y)
                        }
                        '<' => {
                            let new_x = ((x - 1 + cycle_w - 1) % cycle_w) + 1;
                            (new_x, y)
                        }
                        'v' => {
                            let new_y = ((y - 1 + 1) % cycle_h) + 1;
                            (x, new_y)
                        }
                        '^' => {
                            let new_y = ((y - 1 + cycle_h - 1) % cycle_h) + 1;
                            (x, new_y)
                        }
                        c => unreachable!("{}", c),
                    };
                    new_grid[new_y][new_x].push(*c);
                }
            }
        }
        grids.push(new_grid);
    }
    grids
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: usize, y: usize) -> usize {
    x / gcd(x, y) * y
}

fn print_grid(grid: &Grid) {
    for row in grid {
        for cell in row {
            let s = match cell.len() {
                0 => '.',
                1 => cell[0],
                n => (min(9, n) as u8 + b'0') as char,
            };
            print!("{}", s);
        }
        println!("");
    }
    println!("");
    println!("");
}

const REAL: &str = include_str!("real.txt");

const TEST: &str = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
