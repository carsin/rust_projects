use std::io;
use std::io::Write;

struct Coordinate {
    x: usize,
    y: usize,
}

fn main() {
    let mut board = [[0u8; 3]; 3];
    let mut turn = false;
    let mut turn_as_char;
    let mut turn_as_int;

    loop {
        print_board(&board);
        turn_as_int = turn as u8 + 1;
        match turn {
            false => turn_as_char = "X",
            true => turn_as_char = "Y",
        };

        println!("\n{}'s turn.", turn_as_char);
        let last_play_coords = get_player_input();

        if board[last_play_coords.y][last_play_coords.x] == 0 {
            board[last_play_coords.y][last_play_coords.x] = turn_as_int;
            if check_for_win(&board, &last_play_coords, turn_as_int) {
                println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
                print_board(&board);
                println!("{} wins!", turn_as_char);
                io::stdin().read_line(&mut String::new()).unwrap();
                break;
            }
            turn = !turn;
        } else {
            println!("That space is already occupied!");
        }
    }
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

fn get_player_input() -> Coordinate {
    let mut x_input = String::new();
    let mut x: usize = 0;

    while x_input == String::new() {
        print!("X: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut x_input).expect("Failed");

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

    let mut y_input = String::new();
    let mut y: usize = 0;
    while y_input == String::new() {
        print!("Y: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut y_input).expect("Failed");

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
