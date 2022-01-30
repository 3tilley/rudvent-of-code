use clap::{App, Arg, Parser};
use std::ascii::escape_default;
use std::thread::sleep;
use std::time::Duration;
use chrono::prelude::*;
use rudvent::utils::Solution;

enum Section {
    A,
    B,
}

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

    let section = Section.A;

    match section {
        Section::A => {
            let input = rudvent::day8::sol::Sol::prepare_a(false);
            let start : DateTime<Utc> = Utc::now();
            let mid_result = rudvent::day8::sol::Sol::inner_a(input);
            let duration = Utc::now() - start;
            let answer = rudvent::day8::sol::Sol::output_a(mid_result);
        }
        Section::B => {
            let input = rudvent::day8::sol::Sol::prepare_b(false);
            let start : DateTime<Utc> = Utc::now();
            let mid_result = rudvent::day8::sol::Sol::inner_b(input);
            let duration = Utc::now() - start;
            let answer = rudvent::day8::sol::Sol::output_b(mid_result);
        }
    }

    println!("{}us", duration.num_microseconds().unwrap());

    println!("Answer: {:?}", answer);
    //if utils::ask_bool_input() {
        //terminal_clipboard::set_string(answer.to_string());
        //println!("Answer on clipboard")
    //} else {
        //println!("Clipboard unchanged")
    //}
}
