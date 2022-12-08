use itertools::Itertools;
use std::str::FromStr;

const TEST: &str = include_str!("test.txt");
const REAL: &str = include_str!("real.txt");

// itertools has a `k_largest` PR, maybe I want a `parsed` iterator too.
// also maybe a non annoying group_by.
fn main() {
    let x = REAL
        .lines()
        .map(<u32 as FromStr>::from_str)
        // only empty lines can be parse failures right
        .group_by(|x| x.is_ok())
        .into_iter()
        .filter(|(is_ok, _)| *is_ok)
        // now we can actually do the task
        .map(|(_, group)| group.map(|value| value.unwrap()).sum::<u32>())
        .k_largest(3)
        .sum::<u32>();
    println!("{:?}", x);
}
