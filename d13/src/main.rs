#![feature(iter_array_chunks)]
use json::JsonValue;
use std::{cmp::Ordering, time::Instant};

const INPUT: &str = REAL;

#[derive(PartialEq, Eq)]
struct CommPacketJson(JsonValue);
impl PartialOrd for CommPacketJson {
    fn partial_cmp(&self, rhs: &CommPacketJson) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for CommPacketJson {
    fn cmp(&self, rhs: &CommPacketJson) -> Ordering {
        let xs = &self.0;
        let ys = &rhs.0;
        match (xs, ys) {
            (JsonValue::Array(xs), JsonValue::Array(ys)) => {
                let value_check = xs
                    .iter()
                    .zip(ys.iter())
                    .map(|(x, y)| CommPacketJson(x.clone()).cmp(&CommPacketJson(y.clone())))
                    .fold(Ordering::Equal, |acc, x| acc.then(x));
                value_check.then_with(|| xs.len().cmp(&ys.len()))
            }
            (JsonValue::Number(_), JsonValue::Array(_)) => {
                CommPacketJson(JsonValue::Array(vec![xs.clone()])).cmp(rhs)
            }
            (JsonValue::Array(_), JsonValue::Number(_)) => {
                self.cmp(&CommPacketJson(JsonValue::Array(vec![ys.clone()])))
            }
            (JsonValue::Number(_), JsonValue::Number(_)) => {
                xs.as_u8().unwrap().cmp(&ys.as_u8().unwrap())
            }
            _ => unreachable!(),
        }
    }
}

// no integers are more than 2 digits
#[derive(PartialEq, Eq)]
struct CommPacket<'a>(&'a [u8]);
impl<'a> Ord for CommPacket<'a> {
    fn cmp(&self, rhs: &CommPacket<'a>) -> Ordering {
        // println!(
        //     "{} < {}: ",
        //     std::str::from_utf8(&self.0).unwrap(),
        //     std::str::from_utf8(&rhs.0).unwrap()
        // );
        match (self.0[0], rhs.0[0]) {
            (b',', b',') | (b']', b']') | (b'[', b'[') => {
                let lhs = CommPacket(&self.0[1..]);
                let rhs = CommPacket(&rhs.0[1..]);
                lhs.cmp(&rhs)
            }

            // whichever side runs out first is the loser
            (b']', _) => Ordering::Less,
            (_, b']') => Ordering::Greater,

            (b'[', _) => {
                let lhs = CommPacket(&self.0[1..]);
                lhs.cmp(rhs)
            }
            (_, b'[') => {
                let rhs = CommPacket(&rhs.0[1..]);
                self.cmp(&rhs)
            }
            _ => {
                // both digits
                let (lhs_digit, lhs_advance) = take_int(self.0);
                let (rhs_digit, rhs_advance) = take_int(rhs.0);
                lhs_digit.cmp(&rhs_digit).then_with(|| {
                    let lhs = CommPacket(&self.0[lhs_advance..]);
                    let rhs = CommPacket(&rhs.0[rhs_advance..]);
                    lhs.cmp(&rhs)
                })
            }
        }
    }
}
impl<'a> PartialOrd for CommPacket<'a> {
    fn partial_cmp(&self, rhs: &CommPacket<'a>) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

fn take_int(bytes: &[u8]) -> (u8, usize) {
    let mut i = 0;
    let mut x = 0;

    for byte in bytes {
        match byte {
            b']' => {
                // leave the end bracket
                return (x, i);
            }
            b',' => {
                // consume the comma
                return (x, i + 1);
            }
            digit => {
                // found a digit
                debug_assert!((b'0'..=b'9').contains(&digit));
                i += 1;
                x *= 10;
                x += digit - b'0';
            }
        }
    }
    unreachable!()
}

fn main() {
    let start = Instant::now();
    let mut p1 = 0;
    for (ix, pair) in INPUT.split("\n\n").enumerate() {
        let mut lines = pair.lines();
        let lhs = CommPacket(lines.next().unwrap().as_bytes());
        let rhs = CommPacket(lines.next().unwrap().as_bytes());

        let lhs_json = CommPacketJson(json::parse(std::str::from_utf8(lhs.0).unwrap()).unwrap());
        let rhs_json = CommPacketJson(json::parse(std::str::from_utf8(rhs.0).unwrap()).unwrap());
        // assert_eq!(
        //     json_cmp(&lhs_json, &rhs_json),
        //     lhs.cmp(&rhs),
        //     "{}: {} < {}",
        //     ix + 1,
        //     std::str::from_utf8(lhs.0).unwrap(),
        //     std::str::from_utf8(rhs.0).unwrap()
        // );

        // println!(
        //     "{}: {} < {}: {}\n",
        //     ix + 1,
        //     std::str::from_utf8(lhs.0).unwrap(),
        //     std::str::from_utf8(rhs.0).unwrap(),
        //     lhs < rhs
        // );
        // if lhs < rhs {
        //     p1 += ix + 1;
        // }
        if lhs_json < rhs_json {
            p1 += ix + 1;
        }
    }
    let p1_time = Instant::now();

    let mut p2_inputs = INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| CommPacketJson(json::parse(line).unwrap()))
        .collect::<Vec<_>>();
    p2_inputs.push(CommPacketJson(json::parse("[[2]]").unwrap()));
    p2_inputs.push(CommPacketJson(json::parse("[[6]]").unwrap()));
    p2_inputs.sort();

    let mut p2 = 1;
    for (ix, line) in p2_inputs.into_iter().enumerate() {
        if line.0 == json::parse("[[2]]").unwrap() || line.0 == json::parse("[[6]]").unwrap() {
            p2 *= ix + 1;
        }
    }
    let p2_time = Instant::now();

    println!("{} {}", p1, p2);
    println!("{:?} {:?}", p1_time - start, p2_time - p1_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_int_tests() {
        // all ints will be followed by either a comma or an end
        assert_eq!(take_int(b"23,"), (23, 3));
        assert_eq!(take_int(b"23]"), (23, 2));
        assert_eq!(take_int(b"2,"), (2, 2));
        assert_eq!(take_int(b"2]"), (2, 1));
    }

    #[test]
    fn t7() {
        let x = b"[[],[10,6,3,[[2,2],4,2,[7,5,10,5]],[9,3]],[[],[7,7,6,[10,6,2,0],[7,10,5,2,7]]]]";
        let y = b"[[],[3,[10,[3,3,8,2,4],6,[10,3,10,10],3]]]";
        assert!(CommPacket(x) > CommPacket(y));
    }
}
const REAL: &str = include_str!("real.txt");
const TEST: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;
