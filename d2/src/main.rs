use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, Copy, Clone, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (x, y) if x == y => Some(Ordering::Equal),
            (RPS::Rock, RPS::Scissors) | (RPS::Scissors, RPS::Paper) | (RPS::Paper, RPS::Rock) => {
                Some(Ordering::Greater)
            }
            (x, y) => y.partial_cmp(x).map(|x| x.reverse()),
        }
    }
}
// yolo lol
fn parse_theirs(x: u8) -> RPS {
    unsafe { std::mem::transmute(x - b'A') }
}
fn parse_ours(x: u8) -> RPS {
    unsafe { std::mem::transmute(x - b'X') }
}

fn main() {
    let p1: i32 = REAL
        .lines()
        .map(|line| {
            let mut chars = line.bytes();
            let theirs = parse_theirs(chars.nth(0).unwrap());
            let ours = parse_ours(chars.nth(1).unwrap());
            (match theirs.partial_cmp(&ours) {
                Some(Ordering::Equal) => 3,
                Some(Ordering::Less) => 6,
                _ => 0,
            }) + ours as i32
                + 1
        })
        .sum();

    let p2: i32 = REAL
        .lines()
        .map(|line| {
            let mut chars = line.bytes();
            let theirs_discrim = chars.nth(0).unwrap() - b'A';
            let modifier = chars.nth(1).unwrap() as i32 - 'Y' as i32; // x=-1,y=0,z=1
            let ours_discrim = (theirs_discrim as i32 + modifier).rem_euclid(3); // rot for win or lose
            let score = (modifier + 1) * 3; // x=0,y=3,z=6
            score + ours_discrim + 1
        })
        .sum();

    println!("{} {}", p1, p2);
}

const TEST: &str = r#"A Y
B X
C Z"#;

const REAL: &str = r#"B Z
B Z
C Z
C Z
B X
C Y
A Y
B Z
A Z
A Y
C Z
C X
C Z
C X
B X
B Z
B Z
A Z
C Z
C Z
C Y
C Z
B Z
B Z
A X
A Y
B Z
B Z
C Z
B Z
B Z
C X
C Z
A Y
A X
A X
C Z
C Z
B X
C X
B Z
A Z
B X
B Z
B Z
A Z
B Z
A Y
C Z
C Z
B X
C Z
A Z
C Z
C Z
C Z
C Z
A Z
B Z
C Z
A Z
C Z
C Z
C Y
A Y
C X
C Z
C Z
C Z
C Z
A Y
C X
A Y
A Y
C X
B Z
A Y
C Y
C Z
B Z
C Z
B Z
B Z
B Z
C Y
B Z
A Y
C Z
C Z
A X
C X
C Z
A X
C Z
C Z
C X
A Z
C Z
C Z
B Z
C Y
A Y
C Y
C Y
A Z
B Z
C Z
C Z
C Z
A Z
C Z
B X
C Z
A Y
C Z
C X
C Y
C Y
B Z
C Z
C Z
A X
C X
C Z
C Z
A Z
B Z
B Z
C X
C Z
B Z
C X
B Z
C Z
B Z
B Z
B Z
B Z
C Y
C Z
B Z
C Z
C Z
C Z
C Z
B Z
C Z
B Z
C Z
B Z
C Z
C Y
C Z
C Z
C X
C Z
C Z
C Y
C Z
C Y
C X
B Z
B Z
C Z
A Z
C Z
C Z
C Z
B Z
C Z
B Z
C Z
B Z
C Z
C Z
B X
B Z
C Z
C Z
A Z
A Z
B Z
C Z
C Z
A Z
B X
C X
A Z
C Z
C Z
B Y
B Z
B Z
A Y
B Z
C Z
C Z
A Y
B Z
C Y
A Z
C X
A Z
A Y
C X
C Y
C Z
B Z
C Z
A Y
B Z
B Z
C Z
C Z
C Z
C Z
C Z
B Z
C Z
A X
C Z
C Z
A Y
C Z
A Y
C Z
B Z
C Z
A Y
C Z
C Y
C Z
B Z
C X
A Y
C X
A Y
B X
C Y
B X
C Z
C Y
C Z
C Y
C Y
B Z
C Z
A Y
C X
C Z
C Z
B Z
A Z
C Z
C Z
C Y
C Z
C Y
B X
C Z
C Z
C Z
C X
A Y
B Z
C Z
C Z
C Z
B Z
A Y
A Z
A Y
B Z
A Z
A Y
B Z
C Z
A Y
A Z
B Z
B Z
B Z
C Z
C Z
C Z
C Z
C Z
C Z
B Z
C Z
C Z
C Z
A Z
A Y
C X
C Z
A Z
C Z
A Y
C Z
A Y
C Z
C Z
A X
C Y
C Z
A X
C X
A X
B Z
C Z
A X
C Z
B Z
A X
C Z
A Y
A Z
C Y
A Y
C Y
A Z
B Z
A Z
B Z
C X
C X
C Z
C Z
B Z
A Y
C Z
A Y
C Z
B Z
B X
B X
B X
A Z
B Z
C X
C Z
A Y
C Z
B X
A Y
C X
B Z
C Z
C Z
C Z
C Z
A Z
B Z
B Z
B X
A Y
C Z
C X
A Y
C Z
C Z
C Z
A Y
A X
B Z
C Z
A Z
C Y
A Z
B Z
A Y
C Z
C Z
B Z
A Y
B Z
A Z
A Y
C Z
A X
C Z
C Z
B Z
C Z
C X
A Y
A Y
B Z
C Z
B X
A Y
C Z
C Z
A X
A Z
C Z
C Z
A Y
C Z
C Y
A Y
A Y
B Z
C Z
C Z
C X
C Z
A Y
B Z
C Z
C Z
A Y
B Z
C Z
C Z
B Z
A Y
B Z
C Z
C Z
C Z
C X
C Z
C Z
C Z
C Y
C X
C Y
C Z
B Z
A Y
C X
B Z
C X
C X
C Z
A Z
C Z
B X
A Y
C Z
B Z
A X
B X
C X
C Z
C Z
B Z
B Z
C Z
C Z
A Y
B Z
A Z
B Z
A Z
A X
C Z
A Z
A Z
B X
A Z
B Z
B Z
A Y
C Y
B Z
B Z
B Z
B Z
C Z
B Z
C Z
B Z
C Y
C Z
C Z
C X
C Z
C Y
C Y
C X
A Y
C Y
B Z
C Z
A Y
A Z
B X
C Z
C Z
A Y
A Y
A Y
C Z
A Z
C Z
B Y
A X
A Y
C Z
A X
B Z
C Z
C Z
B Z
C Z
A Z
C X
C Z
B Z
B Z
C Z
B Z
A Z
C Z
C Z
A Y
B Z
A Y
B Z
C Z
B Z
C Z
B Z
B Z
A Y
C Y
B Z
C Z
A Z
B Z
C Z
C Z
A Z
B Z
C Z
A Y
A Z
C X
B Z
A Y
A Y
C Z
A Z
B Z
B Z
C Z
C Z
C Z
C Z
A Y
A X
C Z
B Z
C Z
C Z
C Y
C Z
A X
A Z
C Z
B X
A Z
B X
B X
A Y
A Y
A Y
C Z
C Z
C Z
A Z
C X
A Y
C Z
B Z
C Z
A X
B Z
A X
A Z
C Z
C Z
C Z
B X
B Z
C X
B Z
C Z
C Z
C Z
A Y
B Z
B Z
C Z
A X
B Z
A Y
C Z
A Y
A Y
C Z
C Z
A Y
C X
C Z
A Y
C Z
B Z
B Z
C Z
C Y
B Z
B Z
C Z
A Y
C Z
B Z
B X
A Y
A Z
C Z
B Z
C Z
C X
A Y
A Z
A Y
B Z
A Z
B Z
A Y
A Y
B Z
C Z
C Z
C Z
A Y
A X
B Z
A Y
C Z
B X
C Z
C Z
A Y
C Z
C Z
C Y
C Z
C Y
B Z
B Z
C Z
C Z
B Z
B Z
B Z
C Z
B Z
B Z
C Z
C Z
C X
C Z
C Z
B Z
B Z
C X
A Y
C Y
C X
B Z
C X
B Z
C Z
A Z
C Y
C Z
C X
B X
A Y
C Y
A X
C Z
C Z
C Z
C Z
A Y
C Y
C Z
A Y
C Z
C Z
B Z
B Z
B Z
C Z
C Z
C Z
C Z
C Z
C Z
B Z
C Z
B Z
C Z
B Z
C Z
C Z
B Z
C Z
C Z
C Z
A Y
C Z
C Z
A X
C Z
C Z
A X
B Z
C Z
A Y
C X
C Z
A Z
C Y
A Y
C Z
B Z
C Z
B Z
C X
C Y
B Z
C Z
C Z
A Y
C X
C Z
C Z
B X
C Z
C Z
C Z
C Z
A Z
A Y
C Z
B Z
C Z
B Z
A Y
B Z
C Z
B Z
B Y
C Z
B Z
A Y
C Z
C Y
C Z
A X
B Z
C Z
A Z
A Z
B Z
A X
B Z
B Z
C Y
C Z
C Z
B Y
B Z
C Z
A Y
B X
B Z
C Z
C Z
C Z
B X
A Y
A Y
B Z
B Z
C Z
B Z
C Z
C Y
B Z
B Z
B X
A Y
A Y
A Y
B X
C Z
C Z
B Z
C Z
C X
B Z
C Z
C Z
B Z
C Z
C Z
A Y
B Z
A Y
C Y
B Z
B Z
B Z
C X
C Z
C Z
C Z
C X
C Z
A X
B Z
C Z
C Z
A Y
B Z
C Z
A Y
C Z
B Z
C Z
C Z
B Z
B Z
C Z
C Z
C Y
C Z
C Y
C X
C Z
C Y
C Z
A Y
B Z
B Z
A Y
B Z
B X
B X
A Z
C Y
C Z
C Z
B Z
C Z
C Z
C Z
C Z
C Y
C Z
B Z
C Z
C Z
C Z
B Z
B Z
C Z
B X
C Y
A X
C Z
A Z
C Y
C Z
C Z
B Z
C Z
C Z
C Z
B Z
A Y
A Z
B X
C X
A Y
A Y
C Z
C Z
C Z
C Z
C Z
C Z
C Y
C Z
C Z
C Z
A Y
A Z
C Z
A Y
C Z
C Z
B Z
B Z
B X
A Y
B Z
C Y
C Z
B Z
A Z
A Y
A X
C Z
B Z
C Z
B Z
B Z
C Z
C Z
C Z
C Z
A Y
A Z
C Z
C Z
A Y
C Y
C Z
A X
B X
B X
C Z
C Z
B Z
A Y
A Y
A Y
A X
C Z
B Z
B Z
B Z
A Y
C Y
B Z
A Z
C Z
B X
C Z
C Z
C X
C X
C Z
A Y
B Z
C Z
A Y
C Z
A X
C Z
B Z
B Z
B Z
C Z
B Z
C Z
A Z
C Z
B Z
C Z
A Y
C Z
A Z
C Z
C Y
B Z
A Z
C Z
B Y
A Z
C Z
A X
C Z
B Z
A X
C Z
B Z
C Z
C X
A Y
C Z
B Z
A X
C Z
B Z
C Z
A Y
C Z
C Z
C Z
B Z
C Y
C Z
C X
A Z
C Z
C Z
B Z
C Z
C Z
C Z
A Z
C Z
A Y
A X
C Z
A Y
C X
C Z
B Z
C Z
B Z
B Z
A Y
B Z
A Y
A Y
C X
B Z
C Z
C Z
C Z
C Z
C Z
A Y
B Z
C Z
A Y
B Z
C Z
C Z
A Y
B Z
C Z
C Y
B Z
C Z
B Z
A Z
C X
B Z
C Y
C Z
C Z
C X
C X
A X
C Z
B Z
A Z
C Z
B Z
B X
C Z
C Z
C Z
C Z
B Z
B X
C Z
C Z
C Z
B Z
C Y
B Z
B X
C Y
C Z
C Z
B Z
A Z
C Y
C X
C Z
B Z
C Z
C Z
C Z
B Z
B X
C Z
C Z
A Y
C Z
C Y
A Y
A Z
B Y
C Z
A Y
C X
C X
C X
C Z
B Z
C Z
C Z
B Z
C Z
A Y
C Z
C Z
B Z
C X
C Z
B Z
C Z
A Y
C Z
C Z
C X
C Z
B Z
C Y
C Z
A Z
C Z
B Z
A Z
B Z
C Z
A Y
B Z
C Z
C Z
B Z
A Z
C Z
C X
C Z
C Z
A Y
A Y
A Z
C Z
A Y
A X
C Z
C Z
C Z
C X
A Z
C Z
A Y
C Z
C Z
C Y
C Z
C Z
B Z
C Z
C Z
B Z
B Z
C Z
C Z
C Z
B Z
C Z
C Z
C Y
B Z
C Y
B Z
B X
C Z
C Z
A Y
B Z
C Y
B Z
B Z
B Z
B Z
C Z
C Y
A X
C Z
C Z
C Z
C Z
C Z
B Z
C Z
A Y
C Z
C Z
A Y
A Y
C Z
C Z
B Z
B Z
C X
C Z
C Z
A X
C Z
C Y
C Z
B Z
C Z
A Y
C Z
C Z
C Z
C X
A Y
C Z
C X
B Z
C Z
C Z
C Y
C Z
A Y
B Z
B Z
A X
C Y
C Z
B X
A X
C Z
C Z
C Z
C Z
C Y
A X
B Z
C Z
C Z
A Y
B X
A Z
A Y
B Z
C Z
C X
A X
C X
C Y
C Z
B X
A X
C Z
C Z
B X
A Y
A Y
A Y
C Z
C Z
A Y
C Z
A Z
A Y
C Z
B X
C Z
C Z
C X
C Z
C Z
A Z
B Z
A Z
C Y
C Y
C Y
C Z
C X
C Z
A Y
B Z
C Z
B Z
C Z
A Y
C X
C Z
C Z
B Z
C Z
C Y
A Y
B Z
B Z
B X
B Z
C Z
A Y
C Z
B Z
A Y
C Z
C Z
B Z
A Y
C Z
C Z
C Z
C Z
A Z
A Y
C Z
C X
A Z
A Z
C Z
C Z
C Z
A Y
B Z
B Z
B Z
B Z
C Z
C X
C Z
C Z
B Z
B Z
C Z
C X
A Z
A Y
C Z
B Z
C X
C X
A Y
A Y
C Z
B X
C Z
C Y
C X
A Y
C Y
C Z
C Z
B Z
A Z
C Z
C Z
C Z
C X
C Z
B Z
A Y
C Z
B Z
C Z
A Y
A Y
B X
B X
B Z
C Z
C Z
C Z
C Z
C X
C Z
B Z
C X
C X
A Z
B Y
C Z
C Z
C X
B Z
C Z
C Z
A Z
C Z
C X
A Y
A Z
C Z
B Z
C Z
B X
A Y
C Z
C Z
B Z
C X
B Z
B Z
C Z
C Z
C Z
B Z
A Z
C Z
A X
C Z
C Z
C Z
C X
C X
B Z
C Y
C Z
C Y
A Z
A Y
A X
B Z
A Y
C Z
C Y
B Z
B Z
B Z
B Y
B Z
A X
C Z
A Y
B Z
A Y
C Z
C Z
C Z
A Y
C Z
B Z
C Z
A Z
C Y
C X
A Y
A Y
A Y
B Z
C Y
C Z
C Z
C X
B Z
A Y
C Z
C Z
B Z
A Z
C Z
B Z
C X
C Z
B Z
C Z
C Z
B Y
B Z
C Y
B Z
A X
C Y
B Z
C Z
A Y
C Z
A Z
C Z
B Z
A X
C Y
C Y
A Y
A Z
B Z
A Z
C Z
C Y
C X
C Z
A Y
C Z
A Y
C Z
C Z
C Z
C Y
C X
C X
B Z
A Y
C Z
A X
C Z
A Z
C X
A X
A Z
A Y
C Z
C Z
C Z
B Z
B Z
C Z
C Z
C Z
A Y
C Z
A Z
C Z
C Z
C X
C X
C Z
C Z
C Z
C Z
C Z
A Y
B Z
B Z
C Z
B X
C Y
C Z
C Z
C Z
A Y
B X
C Z
C X
B Z
A Z
B X
C Z
C Z
B Z
C X
B Z
C Z
A Y
C X
A Y
A Z
C Y
C Z
C Y
C Z
C X
C Z
C Z
C Z
C Z
C Z
C Z
B Z
C Z
B Z
C X
C Z
C X
B Z
A Z
A X
C Z
A Y
C Y
C Z
C Z
C Z
B Z
B Z
A X
C Z
B Z
C Z
C Z
A X
C Z
B Z
C X
C Z
C Z
C Z
C Z
B X
C X
B Z
C Z
B Z
A Y
C Z
A Y
B X
C Z
C Z
C Z
C Z
A Y
C Z
C Z
B Z
C Z
C Z
B Z
A Y
C X
C Z
C Z
A Y
C X
B Z
B Z
A Y
C Z
B Z
C Z
A Y
B Z
A Z
C Z
B Z
C Z
B Z
C X
B Z
C Z
C Z
C Z
A Y
A Y
C Z
B Z
A Y
B X
C Z
B Z
C X
C Z
B Z
B Z
C Y
C Y
A Z
C X
C Z
B Z
A Y
C Z
C Z
B Z
A Y
B Z
A Z
C Z
B Z
B Z
B Z
C Z
C X
B Z
C Z
B Z
C Z
C Z
A Y
A Y
C Z
C X
A Y
B Z
C Z
A Z
C Z
B Z
A Y
C Z
C X
C Z
C Z
C Z
C Z
A Y
C Z
C X
B Z
B X
C Z
A Y
C Y
C Z
C Z
C X
C Z
C X
C X
C Z
B Z
B X
A Z
A Z
C Z
C Y
A Y
C Z
A Y
C Z
C Z
C Z
B Z
B Z
C Z
B Z
C Z
C Z
C Y
B Z
B Z
C Z
C Z
B Z
C X
A Z
C Z
C Z
A Y
A Y
A Y
C Z
C Z
B Z
A Z
A X
A Y
C X
C Z
B Z
C X
C X
B Z
B Z
B X
C Z
C Y
B Y
C Z
C Z
C Z
B Y
A Y
A Y
C Z
C Z
C Z
C Z
C Z
C Z
C Z
C Z
B X
A Z
A X
C Z
A Y
A Z
C X
C Z
C Y
C Z
C Z
C Z
B Z
B Z
A Y
A Z
C X
B X
B X
C Z
C Y
C Z
C Z
C Y
C Z
C Z
A Z
B Z
C Z
A Y
C Z
C Z
C Z
A Y
C Z
B Y
B X
C Z
A Y
B X
C Y
A Y
C Z
C Z
C Y
A Z
B Z
C Z
C Z
B Z
C Y
C X
C Y
C Y
B Z
C Z
B Z
C Z
A Y
A Y
A Y
B Z
C Z
A Z
C Z
C Y
B Z
B Z
C Y
C Z
C Z
A Y
C Z
C Z
B Z
C Z
C X
A X
A Y
A Y
B Z
C Z
B X
A Y
B Z
C Z
C Z
B Z
A Y
C Z
C Z
B X
A Y
C Z
B X
A Y
C Z
C X
A Z
B Z
B Z
C Z
C X
A Y
C Z
C Z
A Y
B Z
C Z
A Y
C Z
A Y
C Z
B Z
C Z
A X
B Z
C Z
C Z
C X
B Y
A Y
A X
C X
B Z
C Z
C X
B Z
C Z
C Z
C Z
C Z
A Z
C Z
A Y
B X
C Z
C Z
B Z
B Z
C X
B Z
B X
C Z
C Z
C Z
A Y
C Z
C Z
C Z
C Z
A Y
A Z
B Z
C Y
A Z
C Z
C Z
C Z
B X
B Z
C Z
B Z
C Z
C X
C Z
A X
C Z
C Z
C Z
C Z
C Z
A Y
A Y
B Z
B Z
B Z
B Z
C Z
C Z
C Z
C Z
A Z
A Y
C Z
A Z
A Y
C Z
C Z
C Y
A Y
B Z
C X
B Z
A Y
C Z
A Y
C Z
B Z
B Z
B Z
C Z
C Z
A Z
A Y
C X
A Y
C Z
A Y
C Z
C Z
A Y
B Z
C Z
C Y
C Z
B Z
B Z
B Z
C Y
C Z
B Z
C X
B Z
A Y
B Z
C Z
A Y
C Z
C Y
C Z
C Z
C Z
C Z
B X
B Z
C Z
C Z
A Y
C Y
C Z
C Z
C Z
C Z
C Z
C X
B Z
C X
A Y
B X
A Y
C Z
B Z
A Y
B Z
C Z
A Y
C Z
C Z
C X
C X
C Z
B Z
C X
C Z
A X
A Z
C Z
C Y
A Y
A X
B Z
C Z
C Z
B Z
A Y
C Z
A X
B Z
B Z
A X
C Y
C Z
A Y
B Z
C Z
C Z
C Z
B Z
C Z
B Z
C Z
B X
B Z
C Z
A X
C Z
A X
C Z
A Y
C Z
A Z
C Z
C Z
C Z
B X
A Y
A Y
A X
B Z
B Z
C Y
C Z
B Z
A X
C Z
A Y
C Z
C Z
B Z
C Z
C X
B X
B Z
B X
C Z
C Z
B Z
A Z
C Z
C Y
C Y
C Z
C Z
C Z
C Z
C Z
C Z
A X
A Z
C Z
B Z
B Z
B Z
C X
A Z
C Z
C Z
B Z
B Z
C Z
B X
C Z
B Z
C Z
C Z
C Z
C Z
A X
A Z
A Y
C Z
A Z
A Y
C Z
A Z
C Z
A Y
C Z
B Z
C Z
A Y
C Y
A Y
B Z
A Y
B Z
C Z
B X
C X
C Z
B Z
A Z
C Z
B X
C Y
C Z
C Y
A X
C X
C X
B Z
A Y
C Z
A Z
B Z
C Z
C X
C Z
C Z
C Z
C Z
C Z
B Z
B Z
C Z
C Z
C Z
C Z
C X
A Z
C Z
A Y
A Y
C Z
C Z
C Y
C Z
A Y
C Z
B Z
B X
A Z
A Y
A Y
C Z
A Y
C Z
A X
A Y
B X
C Z
C Z
A Z
C Z
A Y
B Z
A Y
C Z
C Z
B Z
C X
C Z
C Z
C Z
C Y
C Z
B X
C Z
C Z
B X
A Z
C Z
C X
C X
B Z
C Z
C Z
B Z
C Z
C Z
B Z
B Z
B Z
B Z
B Z
A Z
C X
C Y
C Z
A Y
C Z
B Z
A Y
C Z
C Z
B Z
B Z
C Z
A Z
A X
A Y
C Z
A Z
C Z
C X
B Z
C Z
A X
B Z
B X
C Z
C Z
C Z
B X
C Z
B Z
B Z
A Y
A Y
A Z
C Z
C Z
C Z
B Z
C Z
B Z
C Z
A Z
C Z
C X
C Z
C Z
C Z
B Z
A Y
C Y
C Z
C Z
C Z
C Z
B Z
C Y
C Z
A Z
C Y
C Z
B X
A Y
A Y
A X
B Z
B Z
C Z
A Y
B X
A Y
C X
C Z
A Y
C X
C Z
C Z
A Z
C X
C Z
C X
C Z
A Y
C Z
C Z
B Z
C Z
C Y
A Z
C X
C Y
C Z
C Z
C Y
B Z
B Z
C X
C Z
C Z
C Z
C Z
B Z
C Z
C Z
C Z
B Y
C Y
C Z
B Z
C Z
C Z
C Y
B Z
A Y
A Z
C Y
C Y
C Z
C X
C Z
A Z
C Z
C Z
A Y
A Z
C Z
A Z
C Z
B Z
B X
B Z
B Z
A Z
C Z
A Y
C Z
C Y
C Z
A Z
B Z
B Z
B Z
C Z
B Z
A Z
A Y
C Z
C Z
C Z
C Z
C Z
A Z
C Z
A Z
A X
B Z
A Y
C X
A Z
B X
B Y
A Y
B Z
A Z
B Z
B Z
C Z
A Z
B Z
B Z
A Y
C Y
C Y
A X
C Z
C Z
C Z
C X
C Z
C Z
C Z
C Z
B Z
A Y
C Z
C Z
C Z
A Z
C X
C Z
B Z
C Y
B X
B Y
C Z
C Z
A X
C Y
C X
B Z
B Z
C Z
B Z
C Y
C Y
C Y
C X
C Z
C Z
C Z
C X
A Y
C Z
C Z
B Z
A Z
C Z
A Y
B X
C Z
C X
C Z
A Z
B Z
A Y
C Z
C Z
C Z
A Z
C Z
C Z
C X
C Z
C Z
B Z
C Z
B Z
C X
C Z
A Y
A Y
C Z
C Z
B Z
C Z
C Z
C X
C Z
A Y
C Z
C Z
C Z
B Z
C Z
A Y"#;
