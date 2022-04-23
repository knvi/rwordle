use colored::*;
use rand::seq::SliceRandom;
use std::{
    collections::HashMap,
};

const ALL_WORDS: &str = include_str!("words.txt");

// returns a vector of all the words in the file
fn words_list() -> Vec<&'static str> {
    ALL_WORDS
        .split('\n')
        .collect()
}

// returns a random item from a string vector
fn random_word(words: &Vec<&'static str>) -> String {
    let mut rng = rand::thread_rng();
    words.choose(&mut rng).unwrap().to_string()
}

// get user's input and then check it in a viable words hashmap 
fn get_user_input(viable_words: &HashMap<&'static str, i32>) -> String {
    loop {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("Error reading input");
        let input = input.trim().to_lowercase();
        if viable_words.contains_key(&input as &str) {
            return input;
        } else {
            println!("That's not a valid word. Please try again!");
        }
    }
}
        
fn display_format(word: String, actual_word: String) {
    word.chars().enumerate().for_each(|(pos, c)| {
        let display = if actual_word.chars().nth(pos).unwrap() == c { 
            format!("{c}").to_uppercase().bright_green()
        } else if actual_word.chars().any(|wc| wc == c) {
            format!("{c}").to_uppercase().bright_yellow()
        } else {
            format!("{c}").to_uppercase().red()
        };
        print!("{display}");
    });

    println!();
}

fn main() {
    println!("Welcome to RWordle! You have 6 tries to guess a 5 letter word. Good luck!");

    let words = words_list(); 
    let hash: HashMap<&'static str, i32> = words.clone()
                                    .into_iter()
                                    .map(|x| (x,1))
                                    .collect();
    const MAX_GUESSES: u32 = 6;

    loop {
        let answer = random_word(&words);

        let mut win: bool = false;

        for _ in 0..MAX_GUESSES {
            println!("Guess a word!");

            let guess : String = get_user_input(&hash);

            display_format(guess.clone(), answer.clone());

            if guess == answer {
                win = true;
                break;
            }
        }

        if win {
            println!("You win!");
        } else {
            println!("You didn't win! The correct word was {}.", answer.to_uppercase());
        }

        println!("\nPlay again? (Y/n)");
        let mut again : String = String::new();
        std::io::stdin().read_line(&mut again)
                   .expect("Failed to read line");
        if again.trim().to_lowercase().chars().next().unwrap() == 'y' { continue; } 
        else { break; }
    }
}
