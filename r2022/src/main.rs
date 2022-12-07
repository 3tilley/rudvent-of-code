mod day1;
mod utils;

use clap::Subcommand;

use clap::Parser;
use scraper::{Html, Selector, ElementRef};
use scraper::html::Select;
use crate::utils::{ask_bool_input, ask_index_input};

// const url template
const day_template: &str = "https://adventofcode.com/2022/day/{day}";

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcmd: Option<Commands>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

}

#[derive(Subcommand)]
enum Commands {
    Fetch {
        day: u8,
        #[arg(short, long)]
        overwrite: bool,
    },


}

fn day_url(day: u8) -> String {
    day_template.replace("{day}", &day.to_string())
}

fn handle_downloads(day: u8) {

}

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
    match cli.subcmd.unwrap() {
        Commands::Fetch { day, overwrite } => {
            println!("Fetching day {}", day);
            fetch_day_example(day);

            // println!("{}", text);




            //let input_file_path = utils::get_input_file_path(day);
        }
    }

}
