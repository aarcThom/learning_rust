use std::io;

use rand::prelude::*;

fn main() {
    // the main intro

    println!(
        "I am a mighty wizard!\n\
        Think of a number between 1 and 100 (inclusive) and I will guess it!"
    );

    let mut low_num: u32 = 0;
    let mut high_num: u32 = 100;

    loop {
        let mut user_feedback = String::new();
        let wiz_guess: u32 = rand::rng().random_range(low_num..=high_num);

        println!(
            "Is your number {wiz_guess}? \n\
        Type 'l' if my guess is too low.\n\
        Type 'h' if my guess is too high.\n\
        Type 'c' if my guess is correct!"
        );

        io::stdin()
            .read_line(&mut user_feedback)
            .expect("You didn't provide a guess!");

        let user_feedback = user_feedback.trim().to_lowercase();

        if user_feedback == "l" {
            low_num = wiz_guess + 1;
        } else if user_feedback == "h" {
            high_num = wiz_guess - 1;
        } else if user_feedback == "c" {
            println!("Correct!");
            break;
        } else {
            println!("I don't understand!");
        }
    }
}
