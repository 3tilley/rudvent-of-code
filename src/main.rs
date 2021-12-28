use clap::{App, Arg, Parser};
use std::ascii::escape_default;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod utils;

fn main() {
    env_logger::init();
    let matches = App::new("Rudvent of code")
        .arg(
            Arg::new("input_file")
                .long("input")
                .short('i')
                .takes_value(true)
                .default_value("e1"),
        )
        .get_matches();

    let input_file = match matches.value_of("input_file") {
        None => "input.txt",
        Some(s) => {
            if s.len() <= 2 {
                if s.chars().nth(0).unwrap() == 'e' {
                    "example.txt"
                } else {
                    s
                }
            } else {
                s
            }
        }
    };

    let answer = day5::sol::b(input_file);
    println!("Answer: {:?}", answer);
    if utils::ask_bool_input() {
        terminal_clipboard::set_string(answer.to_string());
        println!("Answer on clipboard")
    } else {
        println!("Clipboard unchanged")
    }
}
