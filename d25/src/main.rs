use std::{str::from_utf8, time::Instant};
const INPUT: &str = REAL;

fn main() {
    let start_time = Instant::now();
    let sum: isize = INPUT.lines().map(|line| parse_snafu(line.as_bytes())).sum();
    let p1_time = Instant::now();
    println!("{}", print_snafu(sum));
    println!("{:?}", p1_time - start_time);
}

fn parse_snafu(x: &[u8]) -> isize {
    let mut ans = 0;
    for c in x {
        ans *= 5;
        ans += snafu_digit_to_int(*c);
    }
    ans
}

fn print_snafu(mut x: isize) -> String {
    // todo: use ilog(5) to work from the front instead of from the backs
    // let mut modulo = 1; // this is better than dividing
    let mut ans = Vec::new();
    while {
        let (digit, carry) = snafu_int_to_digit_and_carry(x % 5);
        ans.push(digit);
        x /= 5;
        x += carry;
        x != 0
    } {}

    ans.reverse();
    from_utf8(&ans).unwrap().to_string()
}

fn snafu_digit_to_int(x: u8) -> isize {
    match x {
        b'1' => 1,
        b'2' => 2,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => unreachable!(),
    }
}

fn snafu_int_to_digit_and_carry(x: isize) -> (u8, isize) {
    match x {
        0 => (b'0', 0),
        1 => (b'1', 0),
        2 => (b'2', 0),
        3 => (b'=', 1),
        4 => (b'-', 1),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_snafu_tests() {
        assert_eq!(1, parse_snafu(b"1"));
        assert_eq!(2, parse_snafu(b"2"));
        assert_eq!(3, parse_snafu(b"1="));
        assert_eq!(4, parse_snafu(b"1-"));
        assert_eq!(5, parse_snafu(b"10"));
        assert_eq!(6, parse_snafu(b"11"));
        assert_eq!(7, parse_snafu(b"12"));
        assert_eq!(8, parse_snafu(b"2="));
        assert_eq!(9, parse_snafu(b"2-"));
        assert_eq!(10, parse_snafu(b"20"));
        assert_eq!(15, parse_snafu(b"1=0"));
        assert_eq!(20, parse_snafu(b"1-0"));
        assert_eq!(2022, parse_snafu(b"1=11-2"));
        assert_eq!(12345, parse_snafu(b"1-0---0"));
        assert_eq!(314159265, parse_snafu(b"1121-1110-1=0"));
    }

    #[test]
    fn print_snafu_tests() {
        assert_eq!(print_snafu(1), "1");
        assert_eq!(print_snafu(2), "2");
        assert_eq!(print_snafu(3), "1=");
        assert_eq!(print_snafu(4), "1-");
        assert_eq!(print_snafu(5), "10");
        assert_eq!(print_snafu(6), "11");
        assert_eq!(print_snafu(7), "12");
        assert_eq!(print_snafu(8), "2=");
        assert_eq!(print_snafu(9), "2-");
        assert_eq!(print_snafu(10), "20");
        assert_eq!(print_snafu(15), "1=0");
        assert_eq!(print_snafu(20), "1-0");
        assert_eq!(print_snafu(2022), "1=11-2");
        assert_eq!(print_snafu(12345), "1-0---0");
        assert_eq!(print_snafu(314159265), "1121-1110-1=0");
    }
}
const REAL: &str = include_str!("real.txt");

const TEST: &str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

/*
  Decimal          SNAFU
        1              1
        2              2
        3             1=
        4             1-
        5             10
        6             11
        7             12
        8             2=
        9             2-
       10             20
       15            1=0
       20            1-0
     2022         1=11-2
    12345        1-0---0
314159265  1121-1110-1=0

*/
