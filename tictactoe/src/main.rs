use std::io;
use std::io::Write;

fn main() {
    let mut board = [[0u8; 3]; 3];

    print_board(&board);
}

fn print_board(board: &[[u8; 3]; 3]) {
    for x in 0..3 {
        for y in 0..3 {
            match board[x][y] {
                0 => print!("_"),
                1 => print!("X"),
                2 => print!("O"),
                _ => print!("Error!"),
            }
            io::stdout().flush().unwrap();
        }
        println!("");
    }
}
