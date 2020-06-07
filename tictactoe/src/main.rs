use std::io;
use std::io::Write;
use std::process::exit;
use std::cmp;
use std::f64;
pub const INFINITY: isize = f64::INFINITY as isize;

#[derive(PartialEq, Eq, Copy, Clone)] // Allows square to be evaluated with ==
enum Square {
    X,
    O,
    None,
}

impl Square {
    fn get_char(&self) -> char {
        match self {
            Square::X => return 'X',
            Square::O => return 'O',
            Square::None => return '_'
        }
    }
}

struct Coordinate {
    y: usize,
    x: usize,
}

fn main() {
    let mut board = [[Square::None; 3]; 3];
    let mut turn = Square::X;

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

        println!("\n{}'s turn.", turn.get_char());
        let last_play_coords = get_player_input();

        // Check if position is empty
        if board[last_play_coords.y][last_play_coords.x] == Square::None {
            board[last_play_coords.y][last_play_coords.x] = turn;
        } else {
            println!("\nPosition occupied!");
            continue;
        }

        // Check if game is over
        let win = check_for_win(&board);
        if win != Square::None {
            print_board(&board);
            println!("{} wins!", win.get_char());
            if play_again() { board = [[Square::None; 3]; 3]; }
        }

        // Check for draw
        if check_for_draw(&board) {
            print_board(&board);
            println!("DRAW!");
            if play_again() { board = [[Square::None; 3]; 3]; }
        }

        turn = match turn {
            Square::X => Square::O,
            Square::O => Square::X,
            _ => panic!("Error!")
        };

        if bot_playing {
            let bot_move = find_best_move(board);
            board[bot_move.y][bot_move.x] = Square::O;
            println!("Minimax chose {}, {}!", bot_move.x + 1, bot_move.y + 1);

            turn = match turn {
                Square::X => Square::O,
                Square::O => Square::X,
                _ => panic!("Error!")
            };

            let win = check_for_win(&board);
            if win != Square::None {
                print_board(&board);
                println!("{} wins!", win.get_char());
                if play_again() { board = [[Square::None; 3]; 3]; }
            }
        }
    }
}

fn minimax(board: [[Square; 3]; 3], depth: isize, maximizer: bool) -> isize{
    let winner = check_for_win(&board);
    match winner {
        Square::X => return -10 + depth,
        Square::O => return 10 - depth,
        _ => (),
    };

    if check_for_draw(&board) {
        return 0;
    }

    if maximizer {
        let mut score = -INFINITY;
        // Loop through all available positons
        for y in 0..3 {
            for x in 0..3 {
                if board[y][x] == Square::None {
                    let mut board_copy = board;
                    board_copy[y][x] = Square::O;
                    let new_score = minimax(board_copy, depth + 1, !maximizer);
                    score = cmp::max(score, new_score);
                }
            }
        }
        return score;
    } else {
        let mut score = INFINITY;
        for y in 0..3 {
            for x in 0..3 {
                if board[y][x] == Square::None {
                    let mut board_copy = board;
                    board_copy[y][x] = Square::X;
                    let new_score = minimax(board_copy, depth + 1, !maximizer);
                    score = cmp::min(score, new_score)
                }
            }
        }
        return score;
    }
}

fn find_best_move(board: [[Square; 3]; 3]) -> Coordinate {
    let mut best_score = -INFINITY;
    let mut move_coords = Coordinate {y: 0, x: 0 };

    for y in 0..3 {
        for x in 0..3 {
            if board[y][x] == Square::None {
                let mut board_copy = board;
                board_copy[y][x] = Square::O;
                let new_score = minimax(board_copy, 0, true);

                if new_score > best_score {
                    best_score = new_score;
                    move_coords.y = y;
                    move_coords.x = y;
                }
            }
        }
    }
    move_coords
}

fn check_for_win(board: &[[Square; 3]; 3]) -> Square {
    // Horizontal
    for y in 0..3 {
        if board[y][0] == board[y][1] && board[y][1] == board[y][2] && board[y][0] != Square::None {
            return board[y][0];
        }
    }

    // Vertical
    for x in 0..3 {
        if board[0][x] == board[1][x] && board[1][x] == board[2][x] && board[0][x] != Square::None {
            return board[0][x];
        }
    }

    // Diagonal left to right
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != Square::None {
        return board[0][0];
    }

    // Diagonal right to left
    if board[2][0] == board[1][1] && board[1][1] == board[0][2] && board[2][0] != Square::None {
        return board[2][0];
    }

    Square::None // Return Square::None if no winners
}

fn print_board(board: &[[Square; 3]; 3]) {
    print!("\n    1   2   3  \n");
    print!("  ┌───┬───┬───┐\n");
    for y in 0..3 {
        print!("{} | ", y + 1);
        for x in 0..3 {
            print!("{} │ ", board[y][x].get_char());
        }
        print!("\n");
    }
    print!("  └───┴───┴───┘\n");

    io::stdout().flush().unwrap();
}

fn check_for_draw(&board: &[[Square; 3]; 3]) -> bool {
    for y in 0..3 {
        for x in 0..3 {
            if &board[y][x] == &Square::None {
                return false;
            }
        }
    }
    true
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

fn console_input(msg: &str) -> String {
    let mut input = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input
}
