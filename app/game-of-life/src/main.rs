#![no_std]
#![no_main]

use tdriver::entry;
use tdriver::graphics;

entry!(main);

fn update_board(board: &[u64; graphics::HEIGHT]) -> [u64; graphics::HEIGHT] {
    let mut board_next: [u64; graphics::HEIGHT] = [0; graphics::HEIGHT];
    for r in 1..10 {
        // let c = 1;
        for c in 1..10 {
            let cur_state = (board[r] >> c) & 0b1;

            let neighbors_alive =   ((board[r-1] >> (c-1)) & 1) +
                ((board[r-1] >> c) & 1) +
                ((board[r-1] >> (c+1)) & 1) +
                ((board[r] >> (c-1)) & 1) +
                ((board[r] >> (c+1)) & 1) +
                ((board[r+1] >> (c-1)) & 1) +
                ((board[r+1] >> c) & 1) +
                ((board[r+1] >> (c+1)) & 1);
            if cur_state == 1 {
                if neighbors_alive < 2 || neighbors_alive > 3 {
                    board_next[r] &= !(0b1 << c);
                } else {
                    board_next[r] |= 0b1 << c;
                }
            } else {
                if neighbors_alive == 3 {
                    board_next[r] |= 0b1 << c;
                } else {
                    board_next[r] &= !(0b1 << c);
                }
            }
        }
    }
    board_next
}

// Entry point of user code
fn main() -> ! {
    let mut board: [u64; graphics::HEIGHT] = [
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000100000100,
        0b0000000000000000000000000000000000000000000000000000000100011000,
        0b0000000000000000000000000000000000000000000000000000000100001100,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
    ];
    let mut screen = graphics::init();
    graphics::write_long(&mut screen, &board);
    graphics::update(&mut screen);
    loop {
        // for i in 0..48 {
        //     for j in 0..64 {
        //         board[i] = 1 <<j;
        //         graphics::write_long(&mut screen, &board);
        //         graphics::update(&mut screen);
        //     }
        // }
        board = update_board(&board);
        graphics::write_long(&mut screen, &board);
        graphics::update(&mut screen);
    }
}
