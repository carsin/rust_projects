use std::io;
use std::io::Write;

use std::process::exit;

struct Coordinate {
    x: usize,
    y: usize,
}

fn main() {
    let mut board = [[0u8; 3]; 3];
    let mut turn = false;
    let mut turn_as_char;
    let mut turn_as_int;

    let play_bot_input = console_input("Play against bot? y/n: ");
    let play_bot_input = play_bot_input.trim();
    let bot_playing: bool = match play_bot_input {
        "y" => true,
        "n" => false,
        _ => {
            println!("Invalid input, defaulting to bot.");
            false
        }
    };

    loop {
        print_board(&board);
        match turn {
            false => turn_as_char = "X",
            true => turn_as_char = "O",
        };

        turn_as_int = turn as u8 + 1;

        println!("\n{}'s turn.", turn_as_char);
        let last_play_coords = get_player_input();

        // Check if position is empty
        if board[last_play_coords.y][last_play_coords.x] == 0 {
            board[last_play_coords.y][last_play_coords.x] = turn_as_int;
            let win = check_for_win(&board, turn_as_int);
            if win > 0 {
                print_board(&board);
                println!("{} wins!", turn_as_char);
                if play_again() { board = [[0u8; 3]; 3]; }
            }

            if check_for_draw(board) {
                if play_again() { board = [[0u8; 3]; 3]; }
            }

        } else {
            println!("\nPosition occupied!");
            continue;
        }

        turn = !turn;

        if bot_playing {
            println!("Bots turn!");
        }
    }
}

fn minimax(board: &[[u8; 3]; 3], depth: isize) -> isize {
    let score = check_for_win(&board, 2) as isize;
    if score == 2 { return score };

    0
}

fn check_for_draw(board: [[u8; 3]; 3]) -> bool {
    for i in 0..3 {
        if board[i].contains(&0) { break; }
        if i == 2 {
            return true;
            println!("DRAW!!!!!!!");
        }
    }

    false
}


fn play_again() -> bool {
    loop {
        let input = console_input("Play again? y/n: ");

        let input = input.trim();
        match input {
            "y" => return true,
            "n" => exit(0),
            _ => continue
        }
    }
}

fn get_player_input() -> Coordinate {
    let mut x_input = String::new();
    let mut x: usize = 0;

    while x_input == String::new() {
        x_input = console_input("X: ");

        if x_input.trim().parse::<usize>().is_ok() {
            x = x_input.trim().parse::<usize>().unwrap();
            if x > 3 {
                println!("Must input a number lower than 3");
                x_input = String::new();
            }
        } else {
            println!("Not a number: {}", x_input.trim());
            x_input = String::new();
        }
    }

    let mut y_input: String = String::new();
    let mut y: usize = 0;
    while y_input == String::new() {
        y_input = console_input("Y: ");

        if y_input.trim().parse::<usize>().is_ok() {
            y = y_input.trim().parse::<usize>().unwrap();
            if y > 3 {
                println!("Must input a number lower than 3");
                y_input = String::new();
            }
        } else {
            println!("Not a number: {}", y_input.trim());
            y_input = String::new();
        }
    }

    Coordinate { y: y - 1, x: x - 1 }
}

fn check_for_win(board: &[[u8; 3]; 3], turn_as_int: u8) -> u8 {
    // Horizontal & Vertical
    for y in 0..3 {
        for x in 0..3 {
            if board[y][x] != turn_as_int { break; }
            if x == 2 {
                return turn_as_int;
            }

            if y == 2 {
                return turn_as_int;
            }
        }
    }
    // Diagonal left to right
    for i in 0..3 {
        if board[i][i] != turn_as_int { break; }
        if i == 2 {
            return turn_as_int;
        }
    }
    // Diagonal right to left
    for i in 0..3 {
        if board[i][2 - i] != turn_as_int { break; }
        if i == 2 {
            return turn_as_int;
        }
    }

    0 // Return 0 if no winners
}

fn print_board(board: &[[u8; 3]; 3]) {
    print!("\n    1   2   3  \n");
    print!("  ┌───┬───┬───┐\n");
    for y in 0..3 {
        print!("{} │ ", y + 1);
        for x in 0..3 {
            match board[y][x] {
                0 => print!("."),
                1 => print!("X"),
                2 => print!("O"),
                _ => print!("Error!"),
            }
            print!(" │ ");
        }
        print!("\n");
    }
    print!("  └───┴───┴───┘\n");

    io::stdout().flush().unwrap();
}

fn console_input(msg: &str) -> String {
    let mut input = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input
}
