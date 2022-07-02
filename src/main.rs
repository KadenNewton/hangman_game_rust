use rand::{seq::IteratorRandom, thread_rng};
use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
    process::Command,
};

struct GameData {
    secret_word: String,
    discovered_letters: String,
    lives: i32,
    status: String,
}

enum UserInputStatus {
    AlreadyDiscovered,
    LetterGuessed,
    LetterMissed,
}

fn main() {
    let random_word = get_random_word();

    let mut gd: GameData = GameData {
        secret_word: random_word,
        discovered_letters: String::new(),
        lives: 5,
        status: String::new(),
    };
    let mut secret_word_masked = format_masked_string(&gd.secret_word, &gd.discovered_letters);

    // Main game loop
    loop {
        update_screen(&gd, &secret_word_masked);

        println!("Type your guess:");
        let user_guess = read_guess();

        if validate_user_guess(user_guess) {
            let guess_lower = user_guess.unwrap().to_lowercase().next().unwrap();

            match check_user_guess(&gd, guess_lower) {
                UserInputStatus::LetterGuessed => {
                    gd.discovered_letters.push(guess_lower);
                    let status = format!("You discovered {}", guess_lower);
                    gd.status = status;
                    secret_word_masked =
                        format_masked_string(&gd.secret_word, &gd.discovered_letters);

                    if !secret_word_masked.contains('_') {
                        gd.status = "You won!".to_string();
                        update_screen(&gd, &secret_word_masked);
                        break;
                    }
                }

                UserInputStatus::LetterMissed => {
                    gd.discovered_letters.push(guess_lower);
                    gd.lives -= 1;

                    if gd.lives == 0 {
                        gd.status = "You lost!".to_string();
                        secret_word_masked = format_masked_string(&gd.secret_word, &gd.secret_word);
                        update_screen(&gd, &secret_word_masked);
                        println!();
                        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
                        main();
                    } else {
                        let status = format!("Unfortunately, no {}", guess_lower);
                        gd.status = status;
                    }
                }

                UserInputStatus::AlreadyDiscovered => {
                    let status = format!("{} is already discovered!", guess_lower);
                    gd.status = status;
                }
            }
        } else {
            let status = "It is not a letter!".to_string();
            gd.status = status;
        }
    }
}

fn get_random_word() -> String {
    let f = File::open("words.txt")
        .unwrap_or_else(|e| panic!("file not found: {}: {}", "words.txt", e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines.choose(&mut thread_rng()).expect("File had no lines")
}

fn format_masked_string(input: &str, mask: &str) -> String {
    let mut result: String = String::new();

    for (_, c) in input.chars().enumerate() {
        result.push(if c == ' ' || mask.contains(c) { c } else { '_' });
        result.push(' ');
    }
    result
}

fn read_guess() -> Option<char> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().chars().next()
}

fn validate_user_guess(user_guess: Option<char>) -> bool {
    match user_guess {
        Some(guess) => {
            if !guess.is_alphabetic() {
                false
            } else {
                true
            }
        }

        None => false,
    }
}

fn check_user_guess(gd: &GameData, user_guess: char) -> UserInputStatus {
    if gd.discovered_letters.contains(user_guess) {
        return UserInputStatus::AlreadyDiscovered;
    }

    if !gd.secret_word.contains(user_guess) {
        return UserInputStatus::LetterMissed;
    }

    UserInputStatus::LetterGuessed
}

fn update_screen(gd: &GameData, secret_word: &String) {
    clear();
    println!("HANGMAN: CAN YOU GUESS THE WORD?");
    println!(
        "Lives: {}. Discovered letters: {}",
        gd.lives, gd.discovered_letters
    );
    print_hangman(gd);
    println!("{}", secret_word);
    println!("{}", gd.status);
}

fn print_hangman(gd: &GameData) {
    match gd.lives {
        0 => {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         XO ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|            ");
            println!("|            ");
        }

        1 => {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        2 => {
            println!(" _________   ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        3 => {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
        }

        4 => {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
        }

        _ => {
            println!("             ");
            println!("             ");
            println!("             ");
            println!("             ");
            println!("          O  ");
            println!("         /|\\ ");
            println!("         / \\ ");
        }
    }
}

fn clear() {
    print!("{}c", 27 as char);
}
