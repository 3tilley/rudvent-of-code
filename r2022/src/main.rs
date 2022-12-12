use clap::Parser;
use color_eyre::eyre::Result;
use scraper::html::Select;
use scraper::{ElementRef, Html, Selector};

use crate::cli::BANNER;
use cli::{Cli, Commands};

use crate::utils::{ask_bool_input, ask_index_input, DayData, process_answer};

mod cli;
mod day1;
mod utils;
mod day2;
mod solution;
mod day3;

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
                    println!("Checking example 1");
                    let ans = sol.check_example_1()?;
                    Ok(println!("Example matches: {}", ans))
                } else {
                    println!("Checking part 1");
                    let ans = sol.run_part_1();
                    println!("Answer: {}", ans);
                    let posted = sol.day_data.has_been_posted(false)?;
                    if !posted {
                        println!("You have not posted your answer yet!");
                        if ask_bool_input("Would you like to post your answer now?", false) {
                            let result = sol.day_data.post_ans(&ans.to_string(), !part_2)?;
                            process_answer(result).unwrap();
                        }
                    }
                    Ok(())
                }
            } else {
                if example {
                    println!("Checking example 2");
                    let ans = sol.check_example_2()?;
                    Ok(println!("Example matches: {}", ans))
                } else {
                    println!("Running part 2");
                    let ans = sol.run_part_2();
                    println!("Answer: {}", ans);
                    let posted = sol.day_data.has_been_posted(false)?;
                    if !posted {
                        println!("You have not posted your answer yet!");
                        if ask_bool_input("Would you like to post your answer now?", false) {
                            sol.day_data.post_ans(&ans.to_string(), !part_2)?;
                        }
                    }
                    Ok(())
                }
            }
        }
        // Print help as well as a banner
        //None => println!("{}\n{}", BANNER, cli.about),
        None => Ok(println!("{}\n{}", BANNER, cli.debug)),
    }
}
