use std::io;
use std::io::Write;

fn main() {
    let mut board = [[0; 3]; 3];
    let mut turn = false;
    let playing = true;

    while playing {
        print_board(&board);
        match turn {
            false => println!("It is X's turn"),
            true => println!("It is O's turn"),
        };

        let play_position = get_player_input();

        if board[play_position[0]][play_position[1]] == 0{
            board[play_position[0]][play_position[1]] = turn as u8 + 1;
            turn = !turn;
        } else {
            println!("That space is already occupied!");
        }
    }
}

fn print_board(board: &[[u8; 3]; 3]) {
    print!("    1   2   3  \n");
    print!("  ┌───┬───┬───┐\n");
    for x in 0..3 {
        print!("{} │ ", x + 1);
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
            } else {
                continue;
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
            } else {
                continue;
            }
        } else {
            println!("Not a number: {}", y_input.trim());
            y_input = String::new();
        }
    }
    return [x - 1, y - 1];
}
