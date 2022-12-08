// use std::cmp::{Ordering, PartialOrd};

// #[derive(Debug, Copy, Clone, PartialEq)]
// enum RPS {
//     Rock,
//     Paper,
//     Scissors,
// }
// impl PartialOrd for RPS {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match (self, other) {
//             (x, y) if x == y => Some(Ordering::Equal),
//             (RPS::Rock, RPS::Scissors) | (RPS::Scissors, RPS::Paper) | (RPS::Paper, RPS::Rock) => {
//                 Some(Ordering::Greater)
//             }
//             (x, y) => y.partial_cmp(x).map(|x| x.reverse()),
//         }
//     }
// }
// // yolo lol
// fn parse_theirs(x: u8) -> RPS {
//     unsafe { std::mem::transmute(x - b'A') }
// }
// fn parse_ours(x: u8) -> RPS {
//     unsafe { std::mem::transmute(x - b'X') }
// }

fn main() {
    // let p1: i32 = REAL
    //     .lines()
    //     .map(|line| {
    //         let chars = line.as_bytes();
    //         let theirs = parse_theirs(chars[0]);
    //         let ours = parse_ours(chars[2]);
    //         (match theirs.partial_cmp(&ours) {
    //             Some(Ordering::Equal) => 3,
    //             Some(Ordering::Less) => 6,
    //             _ => 0,
    //         }) + ours as i32
    //             + 1
    //     })
    //     .sum();

    let p1: u32 = REAL
        .as_bytes()
        .chunks(4)
        .map(|chars| {
            let theirs = chars[0] - b'A'; // a=0,b=1,c=2
            let ours = chars[2] - b'X'; // x=0,y=1,z=2
            let goal = (4 + ours - theirs) % 3; // l=0,d=1,w=2
            (goal * 3 + ours + 1) as u32
        })
        .sum();

    let p2: u32 = REAL
        .as_bytes()
        .chunks(4)
        .map(|chars| {
            let theirs = chars[0] - b'A'; // a=0,b=1,c=2
            let goal = chars[2] - b'X'; // x=0,y=1,z=2
            let modifier = goal + 2; // x=-1,y=0,z=1 modulo 3
            let ours = (theirs + modifier) % 3; // rot for win or lose
            (goal * 3 + ours + 1) as u32
        })
        .sum();

    println!("{} {}", p1, p2);
}

const TEST: &str = include_str!("test.txt");
const REAL: &str = include_str!("real.txt");
