use std::{
    cmp::{max, min},
    str::FromStr,
    time::Instant,
};

const W: usize = 1000;
const H: usize = 1000;
const INPUT: &str = REAL;
// const INPUT: &str = TEST;
type Grid = [[bool; W]; H];

fn main() {
    let start = Instant::now();

    let mut grid = [[false; W]; H];
    let max_y = read_input(&mut grid);
    let mut p1 = 0;
    'outer: loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        loop {
            if sand_y > max_y {
                break 'outer;
            } else if drop_grain(&mut grid, &mut sand_x, &mut sand_y) {
                continue;
            } else {
                break;
            }
        }
        p1 += 1;
    }
    let p1_time = Instant::now();

    let mut grid = [[false; W]; H];
    let max_y = read_input(&mut grid);
    let mut p2 = 0;
    loop {
        if grid[0][500] {
            // we're done
            break;
        }
        let mut sand_x = 500;
        let mut sand_y = 0;
        loop {
            if sand_y == max_y + 1 {
                // we hit the floor
                grid[sand_y][sand_x] = true;
                break;
            } else if drop_grain(&mut grid, &mut sand_x, &mut sand_y) {
                continue;
            } else {
                break;
            }
        }
        p2 += 1;
    }
    let p2_time = Instant::now();

    println!("{} {}", p1, p2);
    println!("{:?} {:?}", p1_time - start, p2_time - p1_time);
}

fn drop_grain(grid: &mut Grid, sand_x: &mut usize, sand_y: &mut usize) -> bool {
    let target_down_y = *sand_y + 1;
    for dx in [0, -1, 1] {
        let target_x = (*sand_x as isize + dx) as usize;
        if !grid[target_down_y][target_x] {
            *sand_x = target_x;
            *sand_y = target_down_y;
            return true;
        }
    }

    // we're stuck here
    grid[*sand_y][*sand_x] = true;
    false
}

fn read_input(grid: &mut Grid) -> usize {
    let mut max_y = 0;

    // TODO: sliding windows?
    for line in INPUT.lines() {
        let mut pairs = line.split("->");
        let mut x0y0 = pairs.next().unwrap().split(',');
        let mut x0 = usize::from_str(&x0y0.next().unwrap().trim()).unwrap();
        let mut y0 = usize::from_str(&x0y0.next().unwrap().trim()).unwrap();
        max_y = max(max_y, y0);

        for pair in pairs {
            let mut xy = pair.split(',');
            let x1 = usize::from_str(&xy.next().unwrap().trim()).unwrap();
            let y1 = usize::from_str(&xy.next().unwrap().trim()).unwrap();
            max_y = max(max_y, y1);

            // println!("{},{} -> {},{}", x0, y0, x1, y1);
            for x in min(x0, x1)..=max(x0, x1) {
                grid[y0][x] = true;
            }
            for y in min(y0, y1)..=max(y0, y1) {
                grid[y][x0] = true;
            }
            x0 = x1;
            y0 = y1;
        }
    }
    max_y
}

const REAL: &str = include_str!("real.txt");

const TEST: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;
