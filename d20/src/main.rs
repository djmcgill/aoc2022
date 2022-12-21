use std::{cmp::Ordering, str::FromStr, time::Instant};

// const INPUT: &str = TEST;
const INPUT: &str = REAL;
const KEY: isize = 811589153;

#[derive(Debug)]
struct Node {
    x: isize,
    next: usize,
    prev: usize,
}

fn main() {
    let mut commands = vec![];
    let mut zero_ix = 0;

    let start_time = Instant::now();

    for line in INPUT.lines() {
        let next = commands.len() + 1;
        let prev = if next == 1 { 0 } else { next - 2 };
        let x = isize::from_str(line).unwrap() * KEY;
        if x == 0 {
            zero_ix = next - 1;
        }
        commands.push(Node { x, next, prev });
    }
    let n = commands.len();
    commands[0].prev = n - 1;
    commands[n - 1].next = 0;
    // print_ll(&commands, zero_ix);

    for _ in 0..10 {
        for i in 0..n {
            let x = commands[i].x;
            // print_ll(&commands, zero_ix);

            if x != 0 {
                let mut ix = i;

                // remove ourself
                let prev_ix = commands[ix].prev;
                let next_ix = commands[ix].next;
                commands[prev_ix].next = next_ix;
                commands[next_ix].prev = prev_ix;
                ix = next_ix;

                for _ in 0..x.unsigned_abs() % (n - 1) {
                    if x > 0 {
                        ix = commands[ix].next;
                    } else {
                        ix = commands[ix].prev;
                    }
                }

                // insert behind ix
                let prev_ix = commands[ix].prev;
                commands[i].prev = prev_ix;
                commands[prev_ix].next = i;

                commands[i].next = ix;
                commands[ix].prev = i;
            }
        }
        // print_ll(&commands, zero_ix);
    }
    // print_ll(&commands, zero_ix);

    let mut p2 = 0;
    let mut i = 0;
    let mut ix = zero_ix;
    let x1 = 1000 % n;
    let x2 = 2000 % n;
    let x3 = 3000 % n;
    // println!("{} {} {}", x1, x2, x3);

    while i < n {
        if i == x1 || i == x2 || i == x3 {
            // println!("{} {}: {}", i, ix, commands[ix].x);
            p2 += commands[ix].x;
        }
        ix = commands[ix].next;
        i += 1;
    }
    let end_time = Instant::now();
    println!("p2: {}", p2);
    println!("{:?}", end_time - start_time);
}

fn print_ll(commands: &Vec<Node>, zero_ix: usize) {
    println!("zero: {}: {:?}", zero_ix, commands[zero_ix]);

    let mut n = 0;
    let mut ix = zero_ix;
    print!("{:2} ", commands[ix].x);
    ix = commands[ix].next;
    while ix != zero_ix && n <= commands.len() + 1 {
        print!("{:2} ", commands[ix].x);
        ix = commands[ix].next;
        n += 1;
    }
    println!("");
}

const REAL: &str = include_str!("real.txt");
const TEST: &str = r"1
2
-3
3
-2
0
4
";
