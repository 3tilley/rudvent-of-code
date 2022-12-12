use std::fmt::{Debug, Display};
use clap::Parser;
use color_eyre::eyre::Result;
use scraper::html::Select;
use scraper::{ElementRef, Html, Selector};

use crate::cli::BANNER;
use cli::{Cli, Commands};
use types::Output;
use crate::day3::RuckSack;
use crate::solution::StructSolution;

use crate::utils::{ask_bool_input, ask_index_input, DayData, process_answer};

mod cli;
mod day1;
mod utils;
mod day2;
mod solution;
mod day3;
mod types;
mod day4;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let cli = Cli::parse();
    // println!("{}, {:?}", cli.subcmd.unwrap().day(), utils::get_input_file_path(1));
    match cli.subcmd {
        Some(Commands::Fetch {
            day,
            overwrite,
            dry_run,
        }) => {
            println!("Fetching day {}", day);
            let day_data = DayData::new(day, dry_run);
            day_data.fetch_data()

            // println!("{}", text);
            //let input_file_path = utils::get_input_file_path(day);
        }
        Some(Commands::Desc { day, dry_run, all_html, part_2 }) => {
            println!("Fetching description for day {}", day);
            let day_data = DayData::new(day, dry_run);
            if all_html {
                Ok(println!("{}", day_data.html(!part_2, true)?))
            } else {
                let html = day_data.html(!part_2, false)?;
                let pretty = html2text::from_read(html.as_bytes(), 80);
                Ok(println!("{}", pretty))
            }
        }
        Some(Commands::Run { day, example, part_2 }) => {
            println!("Running day {}", day);
            let sol = day3::make_sol();
            if !part_2 {
                if example {
                    let cont = check_example_and_continue(&sol, !part_2);
                    if !cont {
                        return Ok(());
                    }
                }
                println!("Checking part 1 with full input");
                let ans = sol.run_part_1();
                println!("Answer: {}", ans);
                let posted = sol.day_data.has_been_posted(false)?;
                if !posted {
                    println!("You have not posted your answer yet!");
                    if ask_bool_input("Would you like to post your answer now?", false) {
                        let result = sol.day_data.post_ans(&ans.to_string(), !part_2);
                        match result {
                            Ok(x) => {
                                println!("{}", x);
                                let new_html = sol.day_data.html(false,false)?;
                                let pretty = html2text::from_read(new_html.as_bytes(), 80);
                                println!("{}", pretty);

                            }
                            Err(e) => println!("Error posting answer: {}", e),
                        }
                    }
                }
                Ok(())
            } else {
                if example {
                    let cont = check_example_and_continue(&sol, !part_2);
                    if !cont {
                        return Ok(());
                    }
                }
                println!("Running part 2");
                let ans = sol.run_part_2();
                println!("Answer: {}", ans);
                let posted = sol.day_data.has_been_posted(false)?;
                if !posted {
                    println!("You have not posted your answer yet!");
                    if ask_bool_input("Would you like to post your answer now?", false) {
                        let result = sol.day_data.post_ans(&ans.to_string(), !part_2);
                        match result {
                            Ok(x) => {
                                println!("{}", x);
                                println!("Day {} complete, onto day {}!", day, day + 1);
                            }
                            Err(e) => println!("Error posting answer: {}", e),
                        }
                    }
                }
                Ok(())
            }
        }
        // Print help as well as a banner
        //None => println!("{}\n{}", BANNER, cli.about),
        None => Ok(println!("{}\n{}", BANNER, cli.debug)),
    }
}

fn check_example_and_continue<T, U: Output, V, W: Output>(sol: &StructSolution<T, U, V, W>, part_1: bool) -> bool {
    let suffix = if part_1 { "1" } else { "2" };
    println!("Checking example {}", suffix);
    if part_1 {
        let ans = sol.check_example_1();
        println!("Example matches: {}", ans.unwrap());
    } else {
        let ans = sol.check_example_2();
        println!("Example matches: {}", ans.unwrap());
    };
    !ask_bool_input("Run the full input set?", true)
}
