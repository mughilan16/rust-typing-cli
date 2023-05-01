use std::time::Duration;
use std::time::Instant;
mod print_menu;
mod random_strings;

enum MenuOptions {
    RandomSentence,
    MistakeWords,
    Words,
    Exit,
    Invalid,
}

fn get_choice() -> i32 {
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    let trimmed = choice.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => i,
        Err(..) => 0,
    }
}

fn menu() -> MenuOptions {
    print_menu::print_menu();
    let choice = get_choice();
    println!("{}", choice);
    match choice {
        1 => MenuOptions::RandomSentence,
        2 => MenuOptions::MistakeWords,
        3 => MenuOptions::Words,
        4 => MenuOptions::Exit,
        _ => MenuOptions::Invalid,
    }
}

fn random_sentence() {
    let strings = random_strings::get_random_strings();
    let mut answer = String::new();
    let mut result = print_menu::Result::Type;
    let now = Instant::now();
    let mut time = Duration::new(0, 0);
    for (index, string) in strings.iter().enumerate() {
        print_menu::print_string(string.to_string(), index + 1, strings.len(), result, time);
        std::io::stdin().read_line(&mut answer).unwrap();
        time = Instant::now() - now;
        result = check_answer(string.to_string(), answer.to_string());
    }
}

fn check_answer(string: String, answer_string: String) -> print_menu::Result {
    let words: Vec<&str> = string.split_whitespace().collect();
    let answers: Vec<&str> = answer_string.split_whitespace().collect();
    let mut is_all_correct = print_menu::Result::Correct;
    for (index, answer) in answers.iter().enumerate() {
        if index >= words.len() {
            break;
        }
        if *words[index] != **answer {
            is_all_correct = print_menu::Result::Wrong;
        }
    }
    is_all_correct
}

fn practice_mistake_words() {}

fn typing_words() {}

fn main() {
    loop {
        let choice = menu();
        match choice {
            MenuOptions::RandomSentence => {
                random_sentence();
            }
            MenuOptions::MistakeWords => {
                practice_mistake_words();
            }
            MenuOptions::Words => {
                typing_words();
            }
            MenuOptions::Exit => {
                break;
            }
            MenuOptions::Invalid => {
                continue;
            }
        }
    }
}
