use std::ascii::escape_default;

//use crate::day1a;
mod day1;
mod day2;
mod day3;
mod utils;

fn main() {
    let answer = day3::sol::b();
    println!("Answer: {:?}", answer);
    if utils::ask_bool_input() {
        terminal_clipboard::set_string(answer.to_string());
        println!("Answer on clipboard")
    } else {
        println!("Clipboard unchanged")
    }
}
