use scanf::sscanf;

fn main() {
    let p1 = REAL
        .lines()
        .filter(|line| {
            let mut l1: u32 = 0;
            let mut u1: u32 = 0;
            let mut l2: u32 = 0;
            let mut u2: u32 = 0;
            sscanf!(line, "{}-{},{}-{}", l1, u1, l2, u2).is_ok()
                && (l1 >= l2 && u1 <= u2 || l1 <= l2 && u1 >= u2)
        })
        .count();

    let p2 = REAL
        .lines()
        .filter(|line| {
            let mut l1: u32 = 0;
            let mut u1: u32 = 0;
            let mut l2: u32 = 0;
            let mut u2: u32 = 0;
            sscanf!(line, "{}-{},{}-{}", l1, u1, l2, u2).is_ok()
                && ((l1..=u1).contains(&l2)
                    || (l1..=u1).contains(&u2)
                    || (l2..=u2).contains(&l1)
                    || (l2..=u2).contains(&u1))
        })
        .count();
    println!("{} {}", p1, p2);
}

const TEST: &str = include_str!("test.txt");
const REAL: &str = include_str!("real.txt");
