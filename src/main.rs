use clap::{App, Arg, Parser};
use std::ascii::escape_default;
use std::thread::sleep;
use std::time::Duration;
use chrono::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
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


    //let start : DateTime<Utc> = Utc::now();
    let answer = day6::sol::b("input.txt");
    //let duration = Utc::now() - start;

    //println!("{}us", duration.num_microseconds().unwrap());

    println!("Answer: {:?}", answer);
    //if utils::ask_bool_input() {
        //terminal_clipboard::set_string(answer.to_string());
        //println!("Answer on clipboard")
    //} else {
        //println!("Clipboard unchanged")
    //}
}
