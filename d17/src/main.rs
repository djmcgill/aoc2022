use std::collections::VecDeque;

const INPUT: &str = TEST;

#[derive(Copy, Clone, Debug)]
struct Piece {
    left: usize,
    right: usize,
    height: usize,
    coords: &'static [(usize, usize)],
}

// |..@@@@.|
// |.......|
// |.......|
// |.......|
// +-------+
const H: Piece = Piece {
    left: 2,
    right: 1,
    height: 1,
    coords: &[(0, 0), (1, 0), (2, 0), (3, 0)],
};

// |...@...|
// |..@@@..|
// |...@...|
// |.......|
// |.......|
// |.......|
// +-------+
const P: Piece = Piece {
    left: 2,
    right: 2,
    height: 3,
    coords: &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
};

// |....@..|
// |....@..|
// |..@@@..|
// |.......|
// |.......|
// |.......|
// +-------+
const L: Piece = Piece {
    left: 2,
    right: 2,
    height: 3,
    coords: &[(2, 0), (2, 1), (2, 2), (1, 2), (0, 2)],
};

// |..@....|
// |..@....|
// |..@....|
// |..@....|
// |.......|
// |.......|
// |.......|
// +-------+
const V: Piece = Piece {
    left: 2,
    right: 4,
    height: 4,
    coords: &[(0, 0), (0, 1), (0, 2), (0, 3)],
};

// |..@@...|
// |..@@...|
// |.......|
// |.......|
// |.......|
// +-------+
const S: Piece = Piece {
    left: 2,
    right: 3,
    height: 2,
    coords: &[(0, 0), (1, 0), (0, 1), (1, 1)],
};
const PIECES: [Piece; 5] = [H, P, L, V, S];

fn main() {
    // okay tetris blocks fall down but we don't need everything, only until a row is blocked
    // todo: technically we could also stop when a | couldn't fit down it
    let mut board = VecDeque::<[bool; 7]>::new();
    let mut relative_floor = 0;
    let mut absolute_floor = 0;
    let mut relative_ceiling = 0;

    let mut piece_i = 0;
    let mut wind_j = 0;
    let wind = INPUT.as_bytes();
    loop {
        if piece_i > 5 {
            break;
        }
        let mut new_piece = PIECES[piece_i % PIECES.len()];
        piece_i += 1;

        // top left starts at first row
        let mut new_piece_ix = 0;

        // fixme: record relative floor
        // first add enough space above
        while relative_ceiling - relative_floor < new_piece.height + 3 {
            board.push_front([false; 7]);
            relative_ceiling += 1;
        }

        // // initial move without checking
        // for _ in 0..3 {
        //     match wind[wind_j % wind.len()] {
        //         b'<' if new_piece.left > 0 => {
        //             new_piece.left -= 1;
        //             new_piece.right += 1;
        //         }
        //         b'>' if new_piece.right > 0 => {
        //             new_piece.right -= 1;
        //             new_piece.left += 1;
        //         }
        //         _ => {}
        //     }
        //     wind_j += 1;
        // }
        // // move down without checking
        // new_piece_ix += 3;

        // now we might have collisions, so we go one by one
        loop {
            // wind move
            match wind[wind_j % wind.len()] {
                b'<' if new_piece.left > 0 => {
                    let mut can_move = true;
                    for (x, y) in new_piece.coords {
                        // prospective move left
                        let x = x + new_piece.left - 1;
                        let y = y + new_piece_ix;
                        can_move &= !board[y][x];
                    }
                    if can_move {
                        println!("<");
                        new_piece.left -= 1;
                        new_piece.right += 1;
                    }
                }
                b'>' if new_piece.right > 0 => {
                    let mut can_move = true;
                    for (x, y) in new_piece.coords {
                        // prospective move right
                        let x_ = x + new_piece.left + 1;
                        let y_ = y + new_piece_ix;
                        can_move &= !board[y_][x_];
                    }
                    if can_move {
                        println!(">");
                        new_piece.right -= 1;
                        new_piece.left += 1;
                    }
                }
                _ => {}
            }
            wind_j += 1;

            // fall move
            let mut can_move = true;
            for (x, y) in new_piece.coords {
                let x_ = x + new_piece.left;
                let y_ = y + new_piece_ix + 1;
                can_move &= y_ < board.len() && !board[y_][x_];
            }
            if can_move {
                println!("v");
                new_piece_ix += 1;
            } else {
                // instead of moving down, we're done
                for (x, y) in new_piece.coords {
                    let x_ = x + new_piece.left;
                    let y_ = y + new_piece_ix;
                    board[y_][x_] = true;
                }
                relative_floor += board.len() - new_piece_ix;
                print_board(&board);
                break; // next piece
            }
        }
    }
}

fn print_board(board: &VecDeque<[bool; 7]>) {
    for row in board.into_iter() {
        print!("|");
        for c in &row[..] {
            print!("{}", if *c { '#' } else { '.' });
        }
        println!("|");
    }
    print!("+");
    for _ in 0..7 {
        print!("-");
    }
    println!("+\n\n");
}

const TEST: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
