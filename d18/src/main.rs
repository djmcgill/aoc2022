use std::{collections::HashSet, str::FromStr, time::Instant};

// const INPUT: &str = TEST;
const N: u8 = 22;
const Y_MULT: usize = N as usize;
const Z_MULT: usize = N as usize * N as usize;
const LEN: usize = N as usize * N as usize * N as usize;
const INPUT: &str = REAL;

// 4496
// 254
fn ix((x, y, z): (u8, u8, u8)) -> usize {
    (z as usize) * Z_MULT + (y as usize) * Y_MULT + x as usize
}
// todo: use powers of 2 and bitmask tbh
fn un_ix(ix: usize) -> (u8, u8, u8) {
    let z = ix / Z_MULT;
    let xy = ix % Z_MULT;
    let y = xy / Y_MULT;
    let x = xy % Y_MULT;
    (x as u8, y as u8, z as u8)
}
fn main() {
    let start_time = Instant::now();
    // grid now counts neighbours each voxel has
    let mut grid: Vec<(bool, u8)> = Vec::with_capacity(LEN);
    for _ in 0..LEN {
        grid.push((false, 0));
    }

    for line in INPUT.lines() {
        let mut coords = line.split(',');
        let x = u8::from_str(coords.next().unwrap()).unwrap();
        let y = u8::from_str(coords.next().unwrap()).unwrap();
        let z = u8::from_str(coords.next().unwrap()).unwrap();

        let dxs: &[i8] = if x == 0 {
            &[0, 1]
        } else if x == N - 1 {
            &[0, -1]
        } else {
            &[0, -1, 1]
        };
        let dys: &[i8] = if y == 0 {
            &[0, 1]
        } else if y == N - 1 {
            &[0, -1]
        } else {
            &[0, -1, 1]
        };
        let dzs: &[i8] = if z == 0 {
            &[0, 1]
        } else if z == N - 1 {
            &[0, -1]
        } else {
            &[0, -1, 1]
        };

        for &dx in dxs {
            for &dy in dys {
                for &dz in dzs {
                    let mut c = 0;
                    if dx != 0 {
                        c += 1
                    }
                    if dy != 0 {
                        c += 1
                    }
                    if dz != 0 {
                        c += 1
                    }
                    let x = x.saturating_add_signed(dx);
                    let y = y.saturating_add_signed(dy);
                    let z = z.saturating_add_signed(dz);
                    if c == 0 {
                        // center, so mark as solid
                        grid[ix((x, y, z))].0 = true;
                    } else if c == 1 {
                        // face neighbour, so increment face neighbours
                        grid[ix((x, y, z))].1 += 1;
                    }
                }
            }
        }
    }
    let p2_a = Instant::now();
    let mut p2 = 0;

    let mut air = HashSet::new();
    for ix in 0..LEN {
        let &(present, neighbours) = &grid[ix];
        if present {
            p2 += 6 - neighbours as usize;
        } else {
            air.insert(ix);
        }
    }

    let mut to_visit = vec![0];
    while let Some(x) = to_visit.pop() {
        for neighbour in neighbours(x, &grid) {
            if air.remove(&neighbour) {
                to_visit.push(neighbour);
            }
        }
    }

    for ix in air {
        p2 -= grid[ix].1 as usize;
    }

    // let p1 = n * 6 - touches * 2;
    // let p1_time = Instant::now();
    // println!("p1: {}", p1);
    // println!("{:?}", p1_time - start_time);
    let p2_time = Instant::now();
    println!("p2: {}", p2);
    println!("{:?}", p2_time - start_time);
    println!("grid/parse {:?}", p2_a - start_time);
    println!("connected components {:?}", p2_time - p2_a);
}

fn neighbours(i: usize, grid: &Vec<(bool, u8)>) -> Vec<usize> {
    let mut neighbours = vec![];
    let (x, y, z) = un_ix(i);

    if x != 0 && !grid[i - 1].0 {
        neighbours.push(i - 1);
    }
    if x != N - 1 && !grid[i + 1].0 {
        neighbours.push(i + 1);
    }
    if y != 0 && !grid[i - Y_MULT].0 {
        neighbours.push(i - Y_MULT);
    }
    if y != N - 1 && !grid[i + Y_MULT].0 {
        neighbours.push(i + Y_MULT);
    }
    if z != 0 && !grid[i - Z_MULT].0 {
        neighbours.push(i - Z_MULT);
    }
    if z != N - 1 && !grid[i + Z_MULT].0 {
        neighbours.push(i + Z_MULT);
    }
    neighbours
}

const REAL: &str = include_str!("real.txt");
const TEST: &str = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ixes() {
        for i in 0..N * N * N {
            assert_eq!(ix(un_ix(i)), i);
        }
    }
}
