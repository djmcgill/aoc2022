use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::{
    cmp::{max, min},
    time::Instant,
};

// const INPUT: &str = TEST;
const INPUT: &str = REAL;
type Elf = (isize, isize);

fn p1() -> usize {
    // todo: try a grid based approach? square input is nice
    let mut elves = HashSet::default();
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }

    for i in 0..10 {
        let new_elves = do_shuffle(i, &elves);
        elves = new_elves;
    }

    let (mut min_x, mut max_x) = (isize::MAX, 0);
    let (mut min_y, mut max_y) = (isize::MAX, 0);
    for &(x, y) in &elves {
        min_x = min(min_x, x);
        max_x = max(max_x, x);

        min_y = min(min_y, y);
        max_y = max(max_y, y);
    }
    (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize - elves.len()
}

fn p2() -> usize {
    let mut elves = HashSet::default();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }

    for i in 0.. {
        let new_elves = do_shuffle(i, &elves);
        // todo: work out a better way to see if anybody has moved or not
        if elves == new_elves {
            return i + 1;
        } else {
            elves = new_elves;
        }
    }
    unreachable!()
}

fn do_shuffle(i: usize, elves: &HashSet<Elf>) -> HashSet<Elf> {
    let mut new_positions = HashMap::default();
    let mut prospective_moves = vec![];

    for &elf in elves {
        let prospective_move = prospective_move(i, elf, &elves);
        // this is where each elf is proposing to move
        new_positions
            .entry(prospective_move)
            .and_modify(|x| *x += 1)
            .or_insert(1);

        // keeping track of the generated move so we don't have to calculate twice
        prospective_moves.push((elf, prospective_move));
    }

    // now actually try to move
    let mut new_elves = HashSet::default();
    for &(elf, prospective_move1) in &prospective_moves {
        if new_positions[&prospective_move1] == 1 {
            // we're good
            new_elves.insert(prospective_move1);
        } else {
            // nope
            new_elves.insert(elf);
        }
    }
    new_elves
}

fn prospective_move(i: usize, elf: Elf, elves: &HashSet<Elf>) -> Elf {
    let n = elves.contains(&(elf.0, elf.1 - 1));
    let ne = elves.contains(&(elf.0 + 1, elf.1 - 1));
    let nw = elves.contains(&(elf.0 - 1, elf.1 - 1));
    let s = elves.contains(&(elf.0, elf.1 + 1));
    let se = elves.contains(&(elf.0 + 1, elf.1 + 1));
    let sw = elves.contains(&(elf.0 - 1, elf.1 + 1));
    let e = elves.contains(&(elf.0 + 1, elf.1));
    let w = elves.contains(&(elf.0 - 1, elf.1));

    let count_neighbours = [n, ne, nw, s, se, sw, e, w].iter().filter(|x| **x).count();

    let choices = [
        (!(n || ne || nw), (elf.0, elf.1 - 1)),
        (!(s || se || sw), (elf.0, elf.1 + 1)),
        (!(w || nw || sw), (elf.0 - 1, elf.1)),
        (!(e || ne || se), (elf.0 + 1, elf.1)),
    ];
    if count_neighbours == 0 {
        // don't move
        elf
    } else if choices[i % 4].0 {
        choices[i % 4].1
    } else if choices[(i + 1) % 4].0 {
        choices[(i + 1) % 4].1
    } else if choices[(i + 2) % 4].0 {
        choices[(i + 2) % 4].1
    } else if choices[(i + 3) % 4].0 {
        choices[(i + 3) % 4].1
    } else {
        elf // fixme: shouldn't this be unreachable?
    }
}

fn main() {
    let start_time = Instant::now();
    let p1 = p1();
    let p1_time = Instant::now();
    let p2 = p2();
    let p2_time = Instant::now();
    println!("p1: {}", p1);
    println!("p2: {}", p2);
    println!("{:?}", p1_time - start_time);
    println!("{:?}", p2_time - p1_time);
}

const NEIGHBOURS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const REAL: &str = include_str!("real.txt");

const TEST: &str = r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";
