use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Running guess the number game...");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        print!("Input guess: ");
        io::stdout().flush().unwrap(); // Emit print immediately to console

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! You guessed {}", guess),
            Ordering::Greater => println!("Too big! You guessed {}", guess),
            Ordering::Equal => {
                println!("You got it! The number was {}", guess);
                break;
            }
        }
    }
}
