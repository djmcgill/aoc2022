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

const TEST: &str = include_str!("test.txt");
const REAL: &str = include_str!("real.txt");
