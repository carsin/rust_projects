use std::io;
use std::io::Write;

fn main() {
    let board = [[0; 3]; 3];
    let mut turn = false;

    let playing = true;
    while playing {
        print_board(&board);
        get_player_input(&turn);

        turn = !turn;
    }
}

fn print_board(board: &[[i64; 3]; 3]) {
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

fn get_player_input(turn: &bool) {
    let mut x_input = String::new();
    while x_input == String::new() {
        print!("Turn: {}, Choose x position: ", &turn);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut x_input).expect("Failed");

        if x_input.trim().parse::<i64>().is_ok() {
            x_input = x_input.trim().parse().unwrap();
            println!("x: {}", x_input);
            continue;
        } else {
            println!("Not a number: {}", x_input);
            x_input = String::new();
        }
    }
}
