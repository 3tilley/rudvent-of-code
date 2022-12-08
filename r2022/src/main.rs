use clap::Parser;
use color_eyre::eyre::Result;
use scraper::html::Select;
use scraper::{ElementRef, Html, Selector};

use crate::cli::BANNER;
use cli::{Cli, Commands};

use crate::utils::{ask_bool_input, ask_index_input, DayData};

mod cli;
mod day1;
mod utils;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    // println!("{}, {:?}", cli.subcmd.unwrap().day(), utils::get_input_file_path(1));
    match cli.subcmd {
        Some(Commands::Fetch { day, overwrite, dry_run }) => {
            println!("Fetching day {}", day);
            let day_data = DayData::new(day, dry_run);
            day_data.fetch_data();
            Ok(())

            // println!("{}", text);
            //let input_file_path = utils::get_input_file_path(day);
        },
        Some(Commands::Desc{ day, dry_run }) => {
            println!("Fetching description for day {}", day);
            let day_data = DayData::new(day, dry_run);
            Ok(println!("{}", day_data.html_for_day()))
        },
        Some(Commands::Run { day, example }) => {
            println!("Running day {}", day);
            let ans = day1::solution(example);
            Ok(println!("{}", ans))
        },
        // Print help as well as a banner
        //None => println!("{}\n{}", BANNER, cli.about),
        None => Ok(println!("{}\n{}", BANNER, cli.debug)),
    }
}
