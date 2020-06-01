use std::io;
use std::io::Write;

fn main() {
    let mut board = [[0; 3]; 3];
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
        println!("{}'s turn.", turn_as_char);

        let play_position = get_player_input();

        if board[play_position[0]][play_position[1]] == 0 {
            board[play_position[0]][play_position[1]] = turn_as_int;
            if check_for_win(&board, &play_position, turn_as_int) {
                print_board(&board);
                println!("{} wins!", turn_as_char);
                break;
            }

            // turn = !turn;
        } else {
            println!("That space is already occupied!");
        }
    }
}

fn check_for_win(board: &[[u8; 3]; 3], last_move: &[usize; 2], turn_as_int: u8) -> bool {
    // Horizontal
    for x in 0..3 {
        if board[last_move[0]][x] != turn_as_int { break; }
        if x == 2 {
            return true;
        }
    }
    // Vertical
    for y in 0..3 {
        if board[y][last_move[1]] != turn_as_int { break; }
        if y == 2 {
            return true;
        }
    }

    // Diagonal left to right

    false
}

fn print_board(board: &[[u8; 3]; 3]) {
    print!("    1   2   3  \n");
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

fn get_player_input() -> [usize; 2] {
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
    return [y - 1, x - 1];
}
