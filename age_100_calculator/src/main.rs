use std::io;
use std::io::Write;

extern crate chrono;
use chrono::prelude::*;

fn main() {
    let name = input(("What is your name? ").to_string());
    let age = input(format!("How old are you, {}? ", &name))
        .parse().expect("Failed converting age to number");

    let centennial_year = calculate_year_of_centennial(age);
    println!("{}, you will turn 100 during the year {}!", &name, &centennial_year);
}

fn input(user_message: String) -> String {
    // Print user_message & flush terminal output
    print!("{}", user_message);
    io::stdout().flush().unwrap();

    // Get terminal input & set it to input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed reading line");

    // Trim off whitespace
    let input: String = input.trim().to_string();
    // Return users input as String
    input
}

fn calculate_year_of_centennial(age: i32) -> i32 {
    // Get current year
    let current_year = Utc::today().year();
    // Calculate years left until age 100
    let years = 100 - age;

    &current_year + &years
}
