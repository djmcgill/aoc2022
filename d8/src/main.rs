use std::{cmp::max, collections::HashSet, str::FromStr, time::Instant};

fn main() {
    let start = Instant::now();
    // okay from the left and top we can keep track of the tallest tree we've seen
    // so far, and immediately know if a new tree is visible or not.
    // from the right and top we maintain a candidate list and when a new tree
    // comes in, we prune the candidate list of anything not greater
    let mut sum = 0;
    let mut tallest_from_top: [i8; N] = [-1; N];
    let mut strictly_descending_from_bot_with_y: Vec<Vec<(usize, i8)>> = vec![vec![]; N];

    for (y, row) in INPUT.lines().enumerate() {
        let mut tallest_from_left: i8 = -1;
        let mut strictly_descending_from_right_with_x: Vec<(usize, i8)> = Default::default();
        for (x, tree) in row.bytes().enumerate() {
            // we don't need 1-9, ints are ints
            let tree = tree as i8;
            let definitely_visible = tree > tallest_from_top[x] || tree > tallest_from_left;

            // update info
            tallest_from_left = max(tree, tallest_from_left);
            tallest_from_top[x] = max(tree, tallest_from_top[x]);
            // TODO: in theory don't have to do this pruning every iteration
            remove_all_entries_not_taller(tree, &mut strictly_descending_from_right_with_x);
            remove_all_entries_not_taller(tree, &mut strictly_descending_from_bot_with_y[x]);

            // we don't want to double-count trees here. So
            if definitely_visible {
                // if it's definitely good, then we just count once
                // and don't care if it's visible from right or bottom
                sum += 1;
                // TODO: only prune here, and at the end of the loops
                // remove_all_entries_not_taller(tree, &mut strictly_descending_from_right_with_x);
                // remove_all_entries_not_taller(tree, &mut strictly_descending_from_bot_with_y[x]);
            } else {
                // it might be good, depends if we find a future tree taller or shorter than it
                // so don't count it yet, and put it in the vec
                strictly_descending_from_bot_with_y[x].push((y, tree));
                strictly_descending_from_right_with_x.push((x, tree));
            }

            // println!("");
        }
        // TODO
        // force_strictly_descending_backwards(&mut strictly_descending_from_right_with_x);

        for (x, _) in strictly_descending_from_right_with_x {
            sum += 1;
            // if we know it definitely is visible from the right
            // we don't care about from the bot
            let _ = strictly_descending_from_bot_with_y[x].pop();
        }
    }
    // TODO
    // for (x, vec) in strictly_descending_from_bot_with_y.iter().enumerate() {
    //     sum += force_strictly_descending_backwards_count(&vec[..]);
    // }
    for vec in strictly_descending_from_bot_with_y {
        sum += vec.len();
    }

    let p1 = Instant::now();
    println!("{}", sum);
    // p1: 250.7µs
    println!("p1: {:?}", p1 - start);
}

// assumes a strictly descending list
fn remove_all_entries_not_taller<T>(size: i8, v: &mut Vec<(T, i8)>) {
    if !v.is_empty() {
        let len = v.len();
        for i in 0..len {
            let i = len - 1 - i;
            if v[i].1 <= size {
                let _ = v.pop();
            } else {
                // because it's strictly descending we're done
                break;
            }
        }
    }
}

// TODO
// fn force_strictly_descending_backwards<T: Copy>(vs: &mut Vec<(T, i8)>) {
//     if !vs.is_empty() {
//         let mut new_v = Vec::new();
//         let mut current_max = -1;
//         for i in (0..vs.len()).rev() {
//             let v = vs[i];
//             if v.1 > current_max {
//                 current_max = v.1;
//                 new_v.push(v);
//             }
//         }
//         *vs = new_v;
//     }
// }
// fn force_strictly_descending_backwards_count<T: std::fmt::Display>(vs: &[(T, i8)]) -> u8 {
//     let mut count = 0;
//     if !vs.is_empty() {
//         let mut current_max = -1;
//         for i in (0..vs.len()).rev() {
//             let v = vs[i].1;
//             if v > current_max {
//                 current_max = v;
//                 count += 1;
//                 // println!("     {}): 3", vs[i].0);
//             }
//         }
//     }
//     count
// }

const N: usize = REALN;
const INPUT: &str = REAL;

const TESTN: usize = 5;
const REALN: usize = 99;
const REAL: &str = include_str!("real.txt");
const TEST: &str = r#"30373
25512
65332
33549
35390
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_all_entries_not_taller_test() {
        let mut v: Vec<((), i8)> = vec![];
        remove_all_entries_not_taller(3, &mut v);
        assert_eq!(v, vec![]);

        let mut v: Vec<((), i8)> = vec![((), 5)];
        remove_all_entries_not_taller(3, &mut v);
        assert_eq!(v, vec![((), 5)]);

        let mut v: Vec<((), i8)> = vec![((), 5)];
        remove_all_entries_not_taller(7, &mut v);
        assert_eq!(v, vec![]);

        let mut v: Vec<((), i8)> = vec![((), 5)];
        remove_all_entries_not_taller(5, &mut v);
        assert_eq!(v, vec![]);
    }

    // #[test]
    // fn force_strictly_descending_backwards_test() {
    //     let mut v: Vec<(char, i8)> =
    //         vec![('a', 7), ('b', 5), ('c', 3), ('d', 1), ('e', 4), ('f', 4)];
    //     force_strictly_descending_backwards(&mut v);
    //     assert_eq!(v, vec![('f', 4), ('b', 5), ('a', 7)]);

    //     let mut v: Vec<((), i8)> = vec![((), 3), ((), 3), ((), 5), ((), 4), ((), 9)];
    //     force_strictly_descending_backwards(&mut v);
    //     assert_eq!(v, vec![((), 9)]);
    // }

    // #[test]
    // fn force_strictly_descending_backwards_count_test() {
    //     let v: Vec<(char, i8)> = vec![('a', 7), ('b', 5), ('c', 3), ('d', 1), ('e', 4), ('f', 4)];
    //     let ans = force_strictly_descending_backwards_count(&v[..]);
    //     assert_eq!(ans, 3);
    // }
}
