use pathfinding::directed::astar::astar;
use std::{
    cmp::{max, min},
    time::Instant,
};

// const INPUT: &str = TEST;
const INPUT: &str = REAL;

fn main() {
    let start_time = Instant::now();
    let mut grid = vec![];
    // TODO: can remove the lines() call with more math
    for line in INPUT.lines() {
        let mut row = vec![];
        for c in line.bytes() {
            row.push(c);
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
        .find(|(_, x)| **x == b'.')
        .unwrap()
        .0;
    let end_x = grid[y_len - 1]
        .iter()
        .enumerate()
        .find(|(_, x)| **x == b'.')
        .unwrap()
        .0;

    // precache the valid moves at a given time
    // todo: snip off the edges tbh
    let mut okay_x_moves = Vec::with_capacity(cycle_w * (cycle_h + 2) * (cycle_w + 2));
    for time in 0..cycle_w {
        for y in 0..cycle_h + 2 {
            for x in 0..cycle_w + 2 {
                let okay = if y == 0 || x == 0 || y == cycle_h + 1 || x == cycle_w + 1 {
                    // edges don't move
                    grid[y][x] == b'.'
                } else {
                    // right moving
                    let r_candidate_x = (x - 1 + cycle_w - (time % cycle_w)) % cycle_w + 1;
                    // left moving
                    let l_candidate_x = (x - 1 + (time % cycle_w)) % cycle_w + 1;

                    grid[y][l_candidate_x] != b'<' && grid[y][r_candidate_x] != b'>'
                };
                okay_x_moves.push(okay);
            }
        }
    }
    let mut okay_y_moves = Vec::with_capacity(cycle_h * (cycle_h + 2) * (cycle_w + 2));
    for time in 0..cycle_h {
        for y in 0..cycle_h + 2 {
            for x in 0..cycle_w + 2 {
                let okay = if y == 0 || x == 0 || y == cycle_h + 1 || x == cycle_w + 1 {
                    // edges don't move
                    grid[y][x] == b'.'
                } else {
                    // down moving
                    let d_candidate_y = (y - 1 + cycle_h - (time % cycle_h)) % cycle_h + 1;
                    // up moving
                    let u_candidate_y = (y - 1 + (time % cycle_h)) % cycle_h + 1;

                    grid[d_candidate_y][x] != b'v' && grid[u_candidate_y][x] != b'^'
                };
                okay_y_moves.push(okay);
            }
        }
    }

    let prep_time = Instant::now();

    let neighbours = |&(x, y, time): &(usize, usize, usize)| {
        let time = time + 1;
        let mut neighbours = Vec::with_capacity(5);
        if is_okay_move(cycle_w, cycle_h, &okay_x_moves, &okay_y_moves, (x, y, time)) {
            neighbours.push(((x, y, time), 1));
        }
        if x > 1
            && is_okay_move(
                cycle_w,
                cycle_h,
                &okay_x_moves,
                &okay_y_moves,
                (x - 1, y, time),
            )
        {
            neighbours.push(((x - 1, y, time), 1));
        }
        // this is the only time we want to move into the top edge
        if y > 0
            && is_okay_move(
                cycle_w,
                cycle_h,
                &okay_x_moves,
                &okay_y_moves,
                (x, y - 1, time),
            )
        {
            neighbours.push(((x, y - 1, time), 1));
        }
        if x < cycle_w
            && is_okay_move(
                cycle_w,
                cycle_h,
                &okay_x_moves,
                &okay_y_moves,
                (x + 1, y, time),
            )
        {
            neighbours.push(((x + 1, y, time), 1));
        }
        // this is the only time we want to move into the bottom edge
        if y < cycle_h + 1
            && is_okay_move(
                cycle_w,
                cycle_h,
                &okay_x_moves,
                &okay_y_moves,
                (x, y + 1, time),
            )
        {
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
fn is_okay_move(
    cycle_w: usize,
    cycle_h: usize,
    okay_x_moves: &Vec<bool>,
    okay_y_moves: &Vec<bool>,
    (x, y, time): (usize, usize, usize),
) -> bool {
    let x_ix = (x, y, time % cycle_w);
    let y_ix = (x, y, time % cycle_h);
    okay_x_moves[to_ix(x_ix, cycle_w, cycle_h)] && okay_y_moves[to_ix(y_ix, cycle_w, cycle_h)]
}

fn to_ix((x, y, time): (usize, usize, usize), cycle_w: usize, cycle_h: usize) -> usize {
    x + y * (cycle_w + 2) + time * (cycle_h + 2) * (cycle_w + 2)
}

const REAL: &str = include_str!("real.txt");

const TEST: &str = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
