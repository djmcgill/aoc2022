use fxhash::FxHashSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut visited = FxHashSet::default();

    let mut xh = 0isize;
    let mut yh = 0isize;
    let mut xt = 0isize;
    let mut yt = 0isize;

    for line in REAL.lines() {
        let bytes = line.as_bytes();
        let mut n = parse_u8_from_bytes(&bytes[2..]) as isize;

        // TODO: okay so after the first one, we know that they're aligned
        // and as can just push the rest of the way
        match bytes[0] {
            b'U' => {
                let mut simple = false;
                while n != 0 {
                    if !simple {
                        yh += 1;
                        // up and down is just left and right with your head sideways
                        simple = step_right_p1(yh, xh, &mut yt, &mut xt);
                        visited.insert((xt, yt));
                        n -= 1;
                    } else {
                        // we can skip the rest of the loop
                        yh += n;
                        for _ in 0..n {
                            yt += 1;
                            visited.insert((xt, yt));
                        }
                        break;
                    }
                }
            }
            b'D' => {
                let mut simple = false;
                while n != 0 {
                    if !simple {
                        yh -= 1;
                        simple = step_left_p1(yh, xh, &mut yt, &mut xt);
                        visited.insert((xt, yt));
                        n -= 1;
                    } else {
                        yh -= n;
                        for _ in 0..n {
                            yt -= 1;
                            visited.insert((xt, yt));
                        }
                        break;
                    }
                }
            }
            b'L' => {
                let mut simple = false;
                while n != 0 {
                    if !simple {
                        xh -= 1;
                        simple = step_left_p1(xh, yh, &mut xt, &mut yt);
                        visited.insert((xt, yt));
                        n -= 1;
                    } else {
                        xh -= n;
                        for _ in 0..n {
                            xt -= 1;
                            visited.insert((xt, yt));
                        }
                        break;
                    }
                }
            }
            _ /* b'R' */ => {
                let mut simple = false;
                while n != 0 {
                    if !simple {
                        xh += 1;
                        simple = step_right_p1(xh, yh, &mut xt, &mut yt);
                        visited.insert((xt, yt));
                        n -= 1;
                    } else {
                        xh += n;
                        for _ in 0..n {
                            xt += 1;
                            visited.insert((xt, yt));
                        }
                        break;
                    }
                }
            }
        }
    }

    let p1 = visited.len();
    let p1_time = Instant::now();

    // P2
    let mut visited = FxHashSet::default();
    let mut rope: [(isize, isize); 10] = Default::default();
    for line in REAL.lines() {
        let bytes = line.as_bytes();
        let n = parse_u8_from_bytes(&bytes[2..]);

        // okay to do the same optimisation here, each move changes what the _next_ valid
        // move is
        // NEW HOTNESS
        match bytes[0] {
            b'U' => {
                for _ in 0..n {
                    rope[0].1 += 1;
                    step_up_p2(rope[0].0, rope[0].1, &mut rope[1..]);
                    visited.insert(rope[9]);
                }
            }
            b'D' => {
                for _ in 0..n {
                    rope[0].1 -= 1;
                    step_down_p2(rope[0].0, rope[0].1, &mut rope[1..]);
                    visited.insert(rope[9]);
                }
            }
            b'L' => {
                for _ in 0..n {
                    rope[0].0 -= 1;
                    step_left_p2(rope[0].0, rope[0].1, &mut rope[1..]);
                    visited.insert(rope[9]);
                }
            }
            _ => {
                for _ in 0..n {
                    rope[0].0 += 1;
                    step_right_p2(rope[0].0, rope[0].1, &mut rope[1..]);
                    visited.insert(rope[9]);
                }
            }
        }

        // OLD BUSTED
        // let (x_mod, y_mod) = match bytes[0] {
        //         b'U'=> {(0, 1)}
        //         b'D'=> {(0, -1)}
        //         b'L'=> {(-1, 0)}
        //         _ /* 'R' */ => {(1, 0)}
        //     };

        // for _ in 0..n {
        //     rope[0].0 += x_mod;
        //     rope[0].1 += y_mod;

        //     // FIXME: this is broke lol
        //     match bytes[0] {
        //         b'U' => step_up_p2(rope[0].0, rope[0].1, &mut rope[1..]),
        //         b'D' => step_down_p2(rope[0].0, rope[0].1, &mut rope[1..]),
        //         b'L' => step_left_p2(rope[0].0, rope[0].1, &mut rope[1..]),
        //         _ => step_right_p2(rope[0].0, rope[0].1, &mut rope[1..]),
        //     }
        //     // for i in 0..rope.len() - 1 {
        //     //     step(rope[i].0, rope[i].1, &mut rope[i + 1].0, &mut rope[i + 1].1);
        //     // }

        //     visited.insert(rope[9]);
        // }
    }
    let p2 = visited.len();
    let p2_time = Instant::now();

    println!("{} {}", p1, p2);
    println!("p1: {:?}", p1_time - start);
    println!("p2: {:?}", p2_time - p1_time);
}

// needs diag moves for p2
fn step_left_p1(xh: isize, yh: isize, xt: &mut isize, yt: &mut isize) -> bool {
    if xh == *xt - 2 {
        *xt -= 1;
        *yt = yh;
        true
    } else {
        // noop
        false
    }
}
fn step_right_p1(xh: isize, yh: isize, xt: &mut isize, yt: &mut isize) -> bool {
    if xh == *xt + 2 {
        *xt += 1;
        *yt = yh;
        true
    } else {
        // noop
        false
    }
}
// we're going full continuation
fn step_left_p2(xh: isize, yh: isize, tail: &mut [(isize, isize)]) {
    if tail.is_empty() {
        return;
    }
    let xt = &mut tail[0].0;
    let yt = &mut tail[0].1;

    if xh == *xt - 2 {
        *xt -= 1;
        if yh > *yt {
            *yt += 1; // diag up left
            step_up_left_p2(*xt, *yt, &mut tail[1..])
        } else if yh < *yt {
            *yt -= 1; // diag down left
            step_down_left_p2(*xt, *yt, &mut tail[1..])
        } else {
            step_left_p2(*xt, *yt, &mut tail[1..])
        }
    } // else noop
}

fn step_right_p2(xh: isize, yh: isize, tail: &mut [(isize, isize)]) {
    if tail.is_empty() {
        return;
    }
    let xt = &mut tail[0].0;
    let yt = &mut tail[0].1;

    if xh == *xt + 2 {
        *xt += 1;
        if yh > *yt {
            *yt += 1; // diag up right
            step_up_right_p2(*xt, *yt, &mut tail[1..])
        } else if yh < *yt {
            *yt -= 1; // diag down right
            step_down_right_p2(*xt, *yt, &mut tail[1..])
        } else {
            step_right_p2(*xt, *yt, &mut tail[1..])
        }
    } // else noop
}

fn step_up_p2(xh: isize, yh: isize, tail: &mut [(isize, isize)]) {
    if tail.is_empty() {
        return;
    }
    let xt = &mut tail[0].0;
    let yt = &mut tail[0].1;

    if yh == *yt + 2 {
        *yt += 1;
        if xh > *xt {
            *xt += 1; // diag right up
            step_up_right_p2(*xt, *yt, &mut tail[1..])
        } else if xh < *xt {
            *xt -= 1; // diag left up
            step_up_left_p2(*xt, *yt, &mut tail[1..])
        } else {
            step_up_p2(*xt, *yt, &mut tail[1..])
        }
    } // else noop
}
fn step_down_p2(xh: isize, yh: isize, tail: &mut [(isize, isize)]) {
    if tail.is_empty() {
        return;
    }
    let xt = &mut tail[0].0;
    let yt = &mut tail[0].1;

    if yh == *yt - 2 {
        *yt -= 1;
        if xh > *xt {
            *xt += 1; // diag right down
            step_down_right_p2(*xt, *yt, &mut tail[1..])
        } else if xh < *xt {
            *xt -= 1; // diag left down
            step_down_left_p2(*xt, *yt, &mut tail[1..])
        } else {
            step_down_p2(*xt, *yt, &mut tail[1..])
        }
    } // else noop
}
fn step_up_left_p2(xh: isize, yh: isize, tail: &mut [(isize, isize)]) {
    if tail.is_empty() {
        return;
    }
    let xt = &mut tail[0].0;
    let yt = &mut tail[0].1;

    // it's either a step up, or a step left, or it's a diag
    if xh == *xt - 2 {
        if yh == *yt + 2 {
            *xt -= 1;
            *yt += 1;
            step_up_left_p2(*xt, *yt, &mut tail[1..])
        } else {
            step_left_p2(xh, yh, tail)
        }
    } else {
        if yh == *yt + 2 {
            step_up_p2(xh, yh, tail)
        }
    }
}
fn step_up_right_p2(xh: isize, yh: isize, tail: &mut [(isize, isize)]) {
    if tail.is_empty() {
        return;
    }
    let xt = &mut tail[0].0;
    let yt = &mut tail[0].1;

    if xh == *xt + 2 {
        if yh == *yt + 2 {
            *xt += 1;
            *yt += 1;
            step_up_right_p2(*xt, *yt, &mut tail[1..])
        } else {
            step_right_p2(xh, yh, tail)
        }
    } else {
        if yh == *yt + 2 {
            step_up_p2(xh, yh, tail)
        }
    }
}
fn step_down_left_p2(xh: isize, yh: isize, tail: &mut [(isize, isize)]) {
    if tail.is_empty() {
        return;
    }
    let xt = &mut tail[0].0;
    let yt = &mut tail[0].1;

    if xh == *xt - 2 {
        if yh == *yt - 2 {
            *xt -= 1;
            *yt -= 1;
            step_down_left_p2(*xt, *yt, &mut tail[1..])
        } else {
            step_left_p2(xh, yh, tail)
        }
    } else {
        if yh == *yt - 2 {
            step_down_p2(xh, yh, tail)
        }
    }
}
fn step_down_right_p2(xh: isize, yh: isize, tail: &mut [(isize, isize)]) {
    if tail.is_empty() {
        return;
    }
    let xt = &mut tail[0].0;
    let yt = &mut tail[0].1;

    if xh == *xt + 2 {
        if yh == *yt - 2 {
            *xt += 1;
            *yt -= 1;
            step_down_right_p2(*xt, *yt, &mut tail[1..])
        } else {
            step_right_p2(xh, yh, tail)
        }
    } else {
        if yh == *yt - 2 {
            step_down_p2(xh, yh, tail)
        }
    }
}

// OLD BUSTED
// fn step(xh: isize, yh: isize, xt: &mut isize, yt: &mut isize) {
//     if (xh - *xt).abs() <= 1 && (yh - *yt).abs() <= 1 {
//         // noop
//     } else if xh == *xt {
//         // vertical
//         if yh == *yt + 2 {
//             *yt += 1;
//         } else if yh == *yt - 2 {
//             *yt -= 1;
//         } else {
//             unreachable!();
//         }
//     } else if yh == *yt {
//         // horizontal
//         if xh == *xt + 2 {
//             *xt += 1;
//         } else if xh == *xt - 2 {
//             *xt -= 1;
//         } else {
//             unreachable!();
//         }
//         // fixme: corner moves
//     } else if xh == *xt + 2 && yh == *yt + 2 {
//         *xt += 1;
//         *yt += 1;
//     } else if xh == *xt - 2 && yh == *yt + 2 {
//         *xt -= 1;
//         *yt += 1;
//     } else if xh == *xt + 2 && yh == *yt - 2 {
//         *xt += 1;
//         *yt -= 1;
//     } else if xh == *xt - 2 && yh == *yt - 2 {
//         *xt -= 1;
//         *yt -= 1;
//     } else if xh == *xt + 2 {
//         // println!("knight move right");
//         *xt += 1;
//         *yt = yh;
//     } else if xh == *xt - 2 {
//         // println!("knight move left");
//         *xt -= 1;
//         *yt = yh;
//     } else if yh == *yt + 2 {
//         // println!("knight move up");
//         *yt += 1;
//         *xt = xh;
//     } else if yh == *yt - 2 {
//         // println!("knight move down");
//         *yt -= 1;
//         *xt = xh;
//     } else {
//         unreachable!()
//     }
//     // println!("t2: ({}, {})", xt, yt);
//     debug_assert!((xh - *xt).abs() <= 1 && (yh - *yt).abs() <= 1);
// }

fn parse_u8_from_bytes(bytes: &[u8]) -> u8 {
    let mut n = bytes[0] - b'0';
    for c in &bytes[1..] {
        n *= 10;
        n += c - b'0';
    }
    n
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {}
}

const TEST: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;
const REAL: &str = include_str!("real.txt");
