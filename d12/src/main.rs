use pathfinding::prelude::*;
// use smallvec::SmallVec;
use std::{
    cmp::{max, min},
    time::Instant,
};

// yes this is cheating slightly no I don't care
const W: usize = 67;
const H: usize = 41;
const INPUT: &[u8] = REAL;

// const W: usize = 8;
// const H: usize = 5;
// const INPUT: &[u8] = TEST;

fn main() {
    let start_time = Instant::now();
    let (start, target) = p1_init();
    let heuristic = |node: &usize| {
        let (x, y) = to_xy(*node);
        let (tx, ty) = to_xy(target);
        (max(tx, x) - min(tx, x)) + (max(ty, y) - min(ty, y))
    };
    let success = |node: &usize| *node == target;
    let successors = |node: &usize| p1_successors(node, start, target);
    let (_, dist) = astar(&start, successors, heuristic, success).unwrap();
    let p1_time = Instant::now();

    // P2
    // we swap it around so we start at 'E' and then look at all the 'a' we can visit
    let start = p2_init();
    let success = |node: &usize| INPUT[*node] == b'a' || INPUT[*node] == b'S';
    let successors = |node: &usize| p2_successors(node, start);
    let (_, dist2) = yen(&start, successors, success, 1)[0];
    let p2_time = Instant::now();

    println!(
        "{} {:?}\n{} {:?}",
        dist,
        p1_time - start_time, // 336µs
        dist2,
        p2_time - p1_time // 170µs
    );

    // let mut grid = [['.'; W]; H];
    // for i in 0..path.len() - 1 {
    //     let (x0, y0) = to_xy(path[i]);
    //     let (x1, y1) = to_xy(path[i + 1]);
    //     let c = match (x0 as isize - x1 as isize, y0 as isize - y1 as isize) {
    //         (-1, 0) => '>',
    //         (1, 0) => '<',
    //         (0, -1) => 'v',
    //         (0, 1) => '^',
    //         _ => unreachable!(),
    //     };
    //     grid[y0][x0] = c;
    // }
    // let end = path[path.len() - 1];
    // let (ex, ey) = to_xy(end);
    // grid[ey][ex] = 'E';
    // grid[0][0] = 'S';

    // for row in grid {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!("");
    // }
}

fn p1_init() -> (usize, usize) {
    let mut start = 0;
    let mut start_set = false;
    let mut target = 0;
    let mut target_set = false;

    for (ix, c) in INPUT.iter().enumerate() {
        if *c == b'S' {
            start = ix;
            if target_set {
                break;
            } else {
                start_set = true
            }
        } else if *c == b'E' {
            target = ix;
            if start_set {
                break;
            } else {
                target_set = true
            }
        }
    }
    (start, target)
}

fn p2_init() -> usize {
    for (ix, c) in INPUT.iter().enumerate() {
        if *c == b'E' {
            return ix;
        }
    }
    unreachable!()
}

fn successor_candidates(node: usize) -> impl Iterator<Item = usize> {
    // how the fuck is this slower than just `Vec`
    // let mut candidates = SmallVec::<[usize; 4]>::new();

    let mut candidates = Vec::with_capacity(4);

    let (x, y) = to_xy(node);
    if x != 0 {
        candidates.push(node - 1);
    }
    if x != W - 1 {
        candidates.push(node + 1);
    }
    if y != 0 {
        candidates.push(node - (W + 1));
    }
    if y != H - 1 {
        candidates.push(node + W + 1);
    }
    candidates.into_iter()
}

fn p1_successors(
    node: &usize,
    start: usize,
    target: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let node = *node;

    let p1_height = move |node| {
        if node == start {
            b'a'
        } else if node == target {
            b'z'
        } else {
            INPUT[node]
        }
    };

    successor_candidates(node)
        .filter(move |&candidate_ix| p1_height(candidate_ix) <= p1_height(node) + 1)
        .map(|ix| (ix, 1))
}

fn p2_successors(node: &usize, start: usize) -> impl Iterator<Item = (usize, usize)> {
    let node = *node;
    let p2_height = move |node| {
        if node == start {
            b'z'
        } else {
            INPUT[node]
        }
    };

    successor_candidates(node)
        .filter(move |&candidate_ix| {
            // we can step from 'c' to 'b' but not 'c' to 'a'
            p2_height(node) <= p2_height(candidate_ix) + 1
        })
        .map(|ix| (ix, 1))
}

// don't forget about line ends lol
#[inline(always)]
fn to_xy(ix: usize) -> (usize, usize) {
    (ix % (W + 1), ix / (W + 1))
}
#[inline(always)]
fn from_xy(ix: (usize, usize)) -> usize {
    ix.1 * (W + 1) + ix.0
}

const REAL: &[u8] = include_bytes!("real.txt");
const TEST: &[u8] = b"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
