use clap::Parser;
use scraper::html::Select;
use scraper::{ElementRef, Html, Selector};

use crate::cli::BANNER;
use cli::{Cli, Commands};

use crate::utils::{ask_bool_input, ask_index_input, DayData};

mod cli;
mod day1;
mod utils;

// const url template
const day_template: &str = "https://adventofcode.com/2022/day/{day}";

fn day_url(day: u8) -> String {
    day_template.replace("{day}", &day.to_string())
}

fn handle_downloads(day: u8) {}

fn fetch_day_example(day: u8) {
    let url = day_url(day);
    let html = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let doc = Html::parse_document(&html);
    let pre_selector = Selector::parse("pre code").unwrap();
    let pres = doc.select(&pre_selector).collect::<Vec<_>>();
    println!("\n{} pre tags\n", pres.len());
    match pres.len() {
        0 => println!("No obvious example blocks found"),
        1 => {
            let pre = pres.get(0).unwrap();
            if quiz_to_save(pre) {
                save_example(day, &pre.inner_html());
            }
        }
        x => {
            println!("Found {} potential example blocks, please select one:", x);
            let index = ask_index_input("Enter a digit to choose", &pres, 3, 0);
            save_example(day, &pres.get(index).unwrap().inner_html());
        }
    }
}

fn quiz_to_save(pre: &ElementRef) -> bool {
    let found_example = pre.inner_html();
    println!("\nFound example:\n{}", found_example);
    ask_bool_input("Save this example?", true)
}

fn save_example(day: u8, content: &str) {
    // Just printing for now
    println!("Saving example for day {}:\n{}", day, content);
}

fn main() {
    let cli = Cli::parse();
    // println!("{}, {:?}", cli.subcmd.unwrap().day(), utils::get_input_file_path(1));
    match cli.subcmd {
        Some(Commands::Fetch { day, overwrite }) => {
            println!("Fetching day {}", day);
            let day_data = DayData::new(day);
            if overwrite || !day_data.example_1_path().exists() {
                println!(
                    "Fetching example 1 from {}",
                    day_data.example_1_path().display()
                );
                fetch_day_example(day);
            } else {
                println!("Example already exists, skipping");
            }

            // println!("{}", text);
            //let input_file_path = utils::get_input_file_path(day);
        }
        // Print help as well as a banner
        //None => println!("{}\n{}", BANNER, cli.about),
        None => println!("{}\n{}", BANNER, cli.debug),
    }
}
