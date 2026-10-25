use std::io;

use rand::prelude::*;

fn main() {
    // the main intro

    println!(
        "I am a mighty wizard!\n\
        stThink of a number between 1 and 100 (inclusive) and I will guess it!"
    );

    let mut low_num: u32 = 1;
    let mut high_num: u32 = 100;

    loop {
        let mut user_feedback = String::new();

        // we need to test if low_num is higher than high_num if the user is being tricky...
        if low_num >= high_num {
            println!("You are trying to trick me! Goodbye!");
            break;
        }

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

        match user_feedback.as_str() {
            "l" => low_num = wiz_guess + 1,
            "h" => high_num = wiz_guess - 1,
            "c" => {
                println!("Correct!");
                break;
            }
            _ => println!("I don't understand!"),
        }
    }
}
