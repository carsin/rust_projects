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

    let play_bot_input = console_input("Play against bot? Y/N: ");
    let play_bot_input = play_bot_input.trim();
    let mut bot_playing = true;
    if play_bot_input == "Y" {
        bot_playing = true;
    } else if play_bot_input == "N" {
        bot_playing = false;
    } else {
        println!("Invalid input, defaulting to bot.")
    }

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
            if check_for_win(&board, &last_play_coords, turn_as_int) {
                print_board(&board);
                println!("{} wins!", turn_as_char);
                if play_again() {
                    board = [[0u8; 3]; 3];
                }
            }

            let mut x = 0;
            for i in 0..3 {
                if board[i].contains(&0) {
                    continue;
                } else {
                    x += 1;
                }

                if x == 3 {
                    println!("DRAW!!!!!!!");
                    if play_again() {
                        board = [[0u8; 3]; 3];
                    }
                }
            }
            println!("Rows filled: {}", x);
        } else {
            println!("\nPosition occupied!");
            continue;
        }

        if bot_playing {
            println!("Bots turn!");
        } else {
            turn = !turn;
        }
    }
}

fn play_again() -> bool {
    loop {
        let input = console_input("Play again? Y/N: ");

        let input = input.trim();
        if input == "Y" {
            return true;
        } else if input == "N" {
            exit(0);
        } else {
            continue;
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

fn check_for_win(board: &[[u8; 3]; 3], last_move: &Coordinate, turn_as_int: u8) -> bool {
    // Horizontal
    for x in 0..3 {
        if board[last_move.y][x] != turn_as_int { break; }
        if x == 2 {
            return true;
        }
    }
    // Vertical
    for y in 0..3 {
        if board[y][last_move.x] != turn_as_int { break; }
        if y == 2 {
            return true;
        }
    }
    // Diagonal left to right
    if last_move.y == last_move.x {
        for i in 0..3 {
            if board[i][i] != turn_as_int { break; }
            if i == 2 {
                return true;
            }
        }
    }
    // Diagonal right to left
    if last_move.y + last_move.x == 2 {
        for i in 0..3 {
            if board[i][2 - i] != turn_as_int { break; }
            if i == 2 {
                return true;
            }
        }
    }

    false
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
