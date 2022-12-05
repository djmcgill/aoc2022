use itertools::Itertools;
use scanf::sscanf;

fn main() {
    let p1 = false;
    let mut stacks: [Vec<char>; 9] = Default::default();
    let mut iter = REAL.lines();

    // parse the problem by splitting each line into chunks where each chunk is a crate or empty
    let top_iter = (&mut iter).take_while(|line| !line.is_empty());
    top_iter.for_each(|line| {
        line.chars()
            .chunks(4) // it's important to have non-exact chunks
            .into_iter()
            .map(|chunk| chunk.skip(1).next().unwrap())
            .enumerate()
            .filter(|(_, char)| char.is_alphabetic()) // ignore spaces and numbers
            .for_each(|(i, char)| {
                stacks[i].push(char);
            });
    });

    // vecs can only push/pop from the front
    for stack in &mut stacks {
        stack.reverse();
    }

    // instructions
    iter.for_each(|line| {
        let mut quantity = 0;
        let mut source = 0;
        let mut dest = 0;
        sscanf!(line, "move {} from {} to {}", quantity, source, dest).unwrap();
        for _ in 0..quantity {
            let c = stacks[source - 1].pop().unwrap();
            stacks[dest - 1].push(c);
        }
        if !p1 {
            // oops we put the crates in the wrong order
            let len = stacks[dest - 1].len();
            stacks[dest - 1][len - quantity..].reverse();
        }
    });
    let p1 = stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>();
    println!("{}", p1);
}

const TEST: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

const REAL: &str = r#"        [M]     [B]             [N]
[T]     [H]     [V] [Q]         [H]
[Q]     [N]     [H] [W] [T]     [Q]
[V]     [P] [F] [Q] [P] [C]     [R]
[C]     [D] [T] [N] [N] [L] [S] [J]
[D] [V] [W] [R] [M] [G] [R] [N] [D]
[S] [F] [Q] [Q] [F] [F] [F] [Z] [S]
[N] [M] [F] [D] [R] [C] [W] [T] [M]
 1   2   3   4   5   6   7   8   9 

move 1 from 8 to 7
move 1 from 2 to 7
move 6 from 9 to 8
move 1 from 9 to 1
move 1 from 9 to 1
move 3 from 3 to 6
move 3 from 3 to 9
move 1 from 9 to 2
move 5 from 7 to 9
move 9 from 1 to 6
move 3 from 4 to 9
move 2 from 9 to 2
move 1 from 4 to 2
move 1 from 3 to 9
move 8 from 9 to 4
move 14 from 6 to 7
move 1 from 3 to 2
move 5 from 4 to 2
move 5 from 5 to 7
move 4 from 2 to 1
move 2 from 4 to 9
move 1 from 4 to 3
move 3 from 5 to 7
move 1 from 8 to 6
move 2 from 8 to 7
move 2 from 1 to 2
move 1 from 9 to 7
move 2 from 1 to 3
move 5 from 6 to 5
move 4 from 5 to 7
move 3 from 8 to 4
move 20 from 7 to 1
move 11 from 7 to 5
move 1 from 6 to 9
move 3 from 9 to 2
move 12 from 1 to 9
move 2 from 8 to 3
move 4 from 2 to 8
move 8 from 2 to 1
move 4 from 8 to 9
move 1 from 2 to 5
move 12 from 9 to 7
move 4 from 4 to 9
move 4 from 9 to 5
move 13 from 5 to 4
move 4 from 4 to 7
move 1 from 7 to 9
move 2 from 9 to 5
move 9 from 1 to 2
move 1 from 8 to 3
move 5 from 4 to 2
move 1 from 3 to 6
move 7 from 2 to 8
move 6 from 1 to 6
move 6 from 8 to 7
move 6 from 2 to 1
move 3 from 9 to 3
move 7 from 3 to 7
move 4 from 4 to 9
move 1 from 8 to 9
move 1 from 3 to 9
move 1 from 2 to 4
move 1 from 9 to 6
move 5 from 1 to 9
move 1 from 4 to 9
move 2 from 9 to 1
move 8 from 6 to 7
move 4 from 9 to 7
move 2 from 5 to 2
move 2 from 1 to 9
move 14 from 7 to 4
move 22 from 7 to 2
move 2 from 7 to 4
move 3 from 7 to 5
move 9 from 4 to 7
move 6 from 2 to 4
move 8 from 4 to 3
move 14 from 2 to 9
move 2 from 3 to 9
move 3 from 2 to 9
move 4 from 4 to 2
move 1 from 4 to 5
move 1 from 1 to 4
move 5 from 7 to 8
move 1 from 1 to 3
move 4 from 5 to 2
move 6 from 3 to 9
move 1 from 3 to 4
move 4 from 8 to 9
move 2 from 4 to 6
move 4 from 5 to 3
move 1 from 7 to 6
move 1 from 8 to 5
move 3 from 3 to 1
move 33 from 9 to 5
move 5 from 2 to 1
move 1 from 3 to 5
move 1 from 7 to 6
move 18 from 5 to 1
move 1 from 2 to 8
move 6 from 5 to 4
move 1 from 8 to 7
move 2 from 4 to 1
move 4 from 1 to 2
move 19 from 1 to 2
move 4 from 6 to 8
move 4 from 1 to 8
move 14 from 2 to 9
move 5 from 2 to 4
move 1 from 8 to 2
move 8 from 2 to 5
move 5 from 8 to 4
move 4 from 9 to 7
move 1 from 8 to 1
move 16 from 5 to 4
move 15 from 4 to 5
move 1 from 9 to 5
move 5 from 7 to 6
move 2 from 7 to 6
move 1 from 1 to 9
move 7 from 6 to 7
move 1 from 8 to 5
move 1 from 1 to 9
move 12 from 5 to 7
move 7 from 5 to 9
move 12 from 7 to 2
move 1 from 7 to 4
move 7 from 4 to 7
move 2 from 9 to 4
move 5 from 4 to 9
move 8 from 2 to 3
move 4 from 2 to 4
move 9 from 4 to 8
move 6 from 3 to 5
move 8 from 7 to 3
move 1 from 4 to 3
move 7 from 8 to 9
move 4 from 5 to 4
move 6 from 3 to 1
move 4 from 3 to 4
move 1 from 3 to 6
move 6 from 4 to 9
move 1 from 6 to 5
move 17 from 9 to 4
move 3 from 7 to 3
move 1 from 7 to 9
move 2 from 5 to 3
move 2 from 1 to 3
move 2 from 8 to 9
move 1 from 5 to 1
move 14 from 4 to 5
move 2 from 3 to 2
move 1 from 7 to 6
move 10 from 9 to 4
move 12 from 9 to 4
move 9 from 4 to 5
move 1 from 2 to 9
move 13 from 5 to 9
move 2 from 5 to 1
move 1 from 2 to 9
move 3 from 4 to 2
move 12 from 4 to 7
move 8 from 5 to 7
move 1 from 1 to 9
move 1 from 6 to 4
move 1 from 5 to 4
move 1 from 4 to 8
move 5 from 3 to 4
move 10 from 9 to 6
move 3 from 6 to 2
move 7 from 6 to 5
move 6 from 5 to 4
move 1 from 8 to 5
move 1 from 1 to 4
move 2 from 7 to 2
move 5 from 4 to 9
move 2 from 5 to 8
move 1 from 1 to 3
move 2 from 1 to 7
move 6 from 7 to 9
move 9 from 9 to 8
move 1 from 1 to 3
move 4 from 2 to 7
move 11 from 7 to 3
move 11 from 8 to 6
move 7 from 3 to 1
move 4 from 7 to 2
move 3 from 2 to 9
move 8 from 1 to 5
move 2 from 7 to 5
move 2 from 2 to 9
move 2 from 3 to 9
move 11 from 4 to 7
move 7 from 9 to 5
move 6 from 6 to 5
move 2 from 2 to 9
move 1 from 2 to 3
move 6 from 9 to 4
move 3 from 9 to 1
move 4 from 3 to 5
move 6 from 7 to 1
move 2 from 6 to 3
move 2 from 9 to 2
move 3 from 3 to 2
move 3 from 6 to 8
move 2 from 7 to 5
move 20 from 5 to 6
move 8 from 5 to 1
move 1 from 5 to 9
move 2 from 8 to 4
move 1 from 8 to 7
move 16 from 1 to 8
move 8 from 8 to 9
move 4 from 2 to 4
move 1 from 1 to 5
move 1 from 5 to 4
move 3 from 8 to 4
move 14 from 4 to 6
move 5 from 8 to 7
move 6 from 7 to 8
move 29 from 6 to 2
move 3 from 9 to 8
move 21 from 2 to 3
move 1 from 8 to 3
move 6 from 9 to 4
move 8 from 3 to 5
move 7 from 8 to 4
move 7 from 3 to 9
move 3 from 7 to 2
move 12 from 4 to 8
move 2 from 3 to 1
move 2 from 9 to 1
move 1 from 6 to 7
move 1 from 7 to 6
move 1 from 6 to 3
move 3 from 1 to 8
move 2 from 4 to 1
move 4 from 6 to 1
move 5 from 2 to 7
move 1 from 1 to 2
move 5 from 1 to 2
move 2 from 8 to 1
move 1 from 4 to 5
move 9 from 8 to 4
move 3 from 7 to 9
move 7 from 5 to 7
move 2 from 5 to 9
move 4 from 9 to 2
move 3 from 3 to 2
move 5 from 2 to 7
move 2 from 8 to 2
move 2 from 7 to 3
move 1 from 8 to 6
move 2 from 1 to 2
move 1 from 6 to 7
move 1 from 8 to 1
move 12 from 7 to 1
move 5 from 2 to 7
move 7 from 4 to 2
move 2 from 4 to 1
move 5 from 3 to 8
move 7 from 1 to 9
move 4 from 7 to 1
move 7 from 1 to 5
move 12 from 9 to 2
move 27 from 2 to 4
move 3 from 8 to 9
move 6 from 2 to 5
move 6 from 1 to 8
move 1 from 7 to 6
move 9 from 5 to 2
move 3 from 9 to 2
move 13 from 4 to 5
move 10 from 2 to 7
move 1 from 9 to 8
move 11 from 5 to 7
move 1 from 8 to 7
move 1 from 2 to 6
move 13 from 4 to 3
move 23 from 7 to 4
move 1 from 6 to 9
move 1 from 2 to 4
move 7 from 3 to 5
move 1 from 9 to 8
move 19 from 4 to 1
move 2 from 4 to 1
move 1 from 7 to 6
move 1 from 4 to 5
move 1 from 5 to 7
move 11 from 5 to 1
move 2 from 5 to 4
move 2 from 6 to 9
move 3 from 8 to 2
move 2 from 8 to 1
move 3 from 2 to 1
move 1 from 9 to 5
move 6 from 1 to 3
move 1 from 9 to 7
move 2 from 7 to 5
move 2 from 8 to 6
move 1 from 3 to 2
move 2 from 8 to 5
move 1 from 2 to 1
move 3 from 4 to 1
move 3 from 5 to 1
move 2 from 5 to 1
move 2 from 6 to 9
move 1 from 9 to 6
move 1 from 4 to 5
move 1 from 9 to 8
move 1 from 8 to 6
move 8 from 1 to 6
move 7 from 1 to 8
move 9 from 1 to 6
move 1 from 5 to 3
move 3 from 8 to 4
move 11 from 3 to 4
move 1 from 3 to 6
move 10 from 6 to 8
move 13 from 1 to 6
move 3 from 4 to 5
move 7 from 8 to 6
move 3 from 8 to 5
move 6 from 5 to 3
move 22 from 6 to 9
move 4 from 3 to 6
move 4 from 9 to 5
move 1 from 1 to 5
move 2 from 3 to 4
move 2 from 1 to 5
move 1 from 9 to 2
move 5 from 8 to 3
move 2 from 9 to 2
move 11 from 6 to 9
move 3 from 2 to 7
move 1 from 6 to 7
move 12 from 9 to 8
move 4 from 7 to 1
move 12 from 4 to 8
move 2 from 4 to 7
move 1 from 1 to 8
move 1 from 5 to 1
move 19 from 8 to 4
move 4 from 5 to 1
move 1 from 7 to 4
move 1 from 7 to 1
move 3 from 3 to 4
move 2 from 8 to 4
move 1 from 5 to 7
move 1 from 7 to 9
move 8 from 1 to 8
move 1 from 1 to 4
move 1 from 3 to 9
move 1 from 3 to 5
move 1 from 5 to 2
move 7 from 8 to 7
move 16 from 4 to 7
move 1 from 7 to 4
move 3 from 8 to 2
move 14 from 7 to 4
move 1 from 5 to 8
move 5 from 7 to 5
move 16 from 4 to 5
move 3 from 5 to 4
move 3 from 2 to 1
move 1 from 7 to 9
move 11 from 4 to 2
move 3 from 8 to 6
move 2 from 1 to 8
move 1 from 4 to 9
move 18 from 5 to 1
move 1 from 8 to 7
move 3 from 7 to 9
move 18 from 9 to 3
move 3 from 6 to 9
move 7 from 1 to 6
move 1 from 8 to 4
move 1 from 4 to 9
move 3 from 6 to 4
move 5 from 9 to 2
move 2 from 4 to 7
move 7 from 2 to 8
move 1 from 7 to 3
move 2 from 6 to 8
move 1 from 9 to 5
move 1 from 6 to 8
move 1 from 4 to 8
move 1 from 5 to 3
move 1 from 7 to 5
move 8 from 8 to 7
move 10 from 2 to 6
move 1 from 9 to 3
move 6 from 6 to 2
move 5 from 6 to 2
move 7 from 2 to 7
move 12 from 1 to 6
move 2 from 2 to 1
move 1 from 2 to 5
move 4 from 7 to 6
move 12 from 3 to 1
move 2 from 7 to 2
move 9 from 3 to 8
move 1 from 2 to 6
move 1 from 5 to 4
move 9 from 6 to 5
move 1 from 7 to 6
move 1 from 4 to 9
move 9 from 6 to 7
move 7 from 8 to 3
move 6 from 3 to 1
move 4 from 8 to 3
move 5 from 3 to 1
move 1 from 9 to 8
move 2 from 8 to 9
move 5 from 5 to 7
move 14 from 7 to 8
move 1 from 9 to 4
move 2 from 2 to 1
move 3 from 5 to 3
move 2 from 3 to 1
move 1 from 4 to 6
move 6 from 8 to 6
move 6 from 8 to 3
move 3 from 6 to 1
move 2 from 8 to 9
move 19 from 1 to 6
move 3 from 9 to 3
move 6 from 3 to 4
move 6 from 6 to 2
move 4 from 3 to 9
move 1 from 7 to 9
move 2 from 5 to 7
move 5 from 9 to 6
move 6 from 7 to 2
move 11 from 2 to 5
move 2 from 7 to 4
move 4 from 4 to 3
move 2 from 4 to 8
move 12 from 1 to 2
move 1 from 8 to 2
move 8 from 5 to 7
move 2 from 4 to 9
move 2 from 7 to 1
move 4 from 2 to 3
move 1 from 8 to 6
move 1 from 1 to 5
move 2 from 9 to 1
move 2 from 7 to 3
move 2 from 5 to 2
move 1 from 5 to 7
move 2 from 7 to 8
move 1 from 5 to 7
move 5 from 3 to 4
move 3 from 1 to 7
move 1 from 2 to 4
move 15 from 6 to 1
move 4 from 4 to 1
move 4 from 2 to 3
move 8 from 3 to 2
move 5 from 2 to 4
move 1 from 8 to 6
move 1 from 8 to 9
move 1 from 3 to 1
move 3 from 7 to 3
move 5 from 7 to 6
move 4 from 2 to 9
move 6 from 2 to 6
move 4 from 9 to 6
move 12 from 1 to 5
move 6 from 4 to 1
move 1 from 3 to 6
move 4 from 5 to 8
move 7 from 5 to 3
move 3 from 8 to 2
move 1 from 2 to 3
move 1 from 9 to 5
move 1 from 4 to 5
move 1 from 8 to 5
move 8 from 6 to 9
move 10 from 1 to 4
move 3 from 6 to 1
move 9 from 3 to 6
move 1 from 3 to 8
move 1 from 2 to 4
move 6 from 9 to 1
move 1 from 1 to 4
move 10 from 1 to 6
move 1 from 8 to 6
move 13 from 6 to 7
move 1 from 2 to 1
move 1 from 9 to 6
move 9 from 7 to 5
move 1 from 9 to 4
move 3 from 7 to 1
move 3 from 5 to 6
move 10 from 4 to 7
move 5 from 6 to 5
move 3 from 4 to 5
move 13 from 6 to 9
move 7 from 5 to 3
move 6 from 3 to 2
move 5 from 6 to 4
move 4 from 2 to 8"#;
