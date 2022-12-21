#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
use clap::Parser;
use color_eyre::eyre::Result;
use scraper::html::Select;
use scraper::{ElementRef, Html, Selector};
use std::fmt::{Debug, Display};
use std::thread;

use crate::cli::BANNER;
use crate::day3::RuckSack;
use crate::solution::StructSolution;
use cli::{Cli, Commands};
use types::Output;

use crate::utils::{ask_bool_input, ask_index_input, process_answer, DayData};

mod cli;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod solution;
mod stack_analysis;
mod types;
mod utils;

const STACK_SIZE: usize = 128 * 1024 * 1024;

fn inner_main() -> Result<()> {
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
        Some(Commands::Desc {
            day,
            dry_run,
            all_html,
            part_2,
        }) => {
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
        Some(Commands::Run {
            day,
            example,
            part_2,
        }) => {
            println!("Running day {}", day);
            let sol = day12::make_sol();
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
                let posted = sol.day_data.check_for_posting(false)?;
                if !posted {
                    println!("You have not posted your answer yet!");
                    if ask_bool_input("Would you like to post your answer now?", false) {
                        let result = sol.day_data.post_ans(&ans.to_string(), !part_2);
                        match result {
                            Ok(x) => {
                                println!("{}", x);
                                let new_html = sol.day_data.html(false, false)?;
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
                let posted = sol.day_data.check_for_posting(false)?;
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

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(inner_main)
        .unwrap();
    child.join().unwrap()
}

fn check_example_and_continue<T, U: Output, V, W: Output>(
    sol: &StructSolution<T, U, V, W>,
    part_1: bool,
) -> bool {
    let suffix = if part_1 { "1" } else { "2" };
    println!("Checking example {}", suffix);
    if part_1 {
        let ans = sol.check_example_1();
        println!("Example matches: {}", ans.unwrap());
    } else {
        let ans = sol.check_example_2();
        println!("Example matches: {}", ans.unwrap());
    };
    ask_bool_input("Run the full input set?", true)
}
