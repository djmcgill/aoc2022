use std::str::FromStr;

fn main() {
    let mut target_count = 20;
    let target_step = 40;
    let mut x: isize = 1;
    let mut c: usize = 0;
    let mut sum = 0;

    let mut grid = ['.'; 240];

    for line in REAL.lines() {
        // dbg!(x, c, target_count);
        let (dx, dc) = match &line[..4] {
            "addx" => (isize::from_str(&line[5..]).unwrap(), 2),
            _ /* "noop" */=> (0, 1),
        };
        if c + dc >= target_count {
            sum += target_count * x as usize;
            target_count += 40;
        }
        for _ in 0..dc {
            // dbg!(x, c);
            let foo = (x - 1..=x + 1).contains(&(c as isize % 40));
            // println!("{}\n\n",  foo);
            if foo {
                grid[c] = '#';
            }
            c += 1;
        }
        x += dx;
    }
    println!("{}", sum);
    for i in 0..grid.len() / 40 {
        let row = &grid[i * 40..(i + 1) * 40];
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

const REAL: &str = include_str!("real.txt");
const TEST1: &str = r#"noop
addx 3
addx -5"#;
const TEST2: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
