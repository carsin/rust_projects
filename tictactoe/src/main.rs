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

fn print_board(board: &[[u8; 3]; 3]) {
    print!("┌───┬───┬───┐\n");
    for x in 0..3 {
        print!("│ ");
        for y in 0..3 {
            match board[x][y] {
                0 => print!("."),
                1 => print!("X"),
                2 => print!("O"),
                _ => print!("Error!"),
            }
            print!(" │ ");
        }
        print!("\n");
    }
    print!("└───┴───┴───┘\n");

    io::stdout().flush().unwrap();
}

fn get_player_input(turn: &bool) -> u8 {
    let mut x_input = String::new();
    while x_input == String::new() {
        print!("Turn: {}, Choose x position: ", &turn);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut x_input).expect("Failed");

        if x_input.trim().parse::<u8>().is_ok() {
            let x: u8 = x_input.trim().parse::<u8>().unwrap();
            if x > 3 { // Why do I have to convert 3 to a string?
                println!("Must input a number lower than 3");
                x_input = String::new();
            } else {
                return x;
            }
        } else {
            println!("Not a number: {}", x_input);
            x_input = String::new();
        }
    }

    0
}
