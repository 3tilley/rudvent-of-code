pub mod app_builder;
pub mod app;

pub use app_builder::AppBuilder;
pub use app::App;

use clap::Parser;
use clap::Subcommand;
use color_eyre::eyre::{eyre, Result};

#[derive(Parser, Debug)]
#[command(
    author,
    about = "Advent of code runner for Rust",
    version,
    arg_required_else_help = true
)]
pub struct Cli {
    #[clap(subcommand)]
    pub sub_cmd: Commands,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create the template for a new day
    New {
        day: u8,
        #[arg(short, long)]
        overwrite: bool,
    },
    /// Fetch data for a particular day
    Fetch {
        day: u8,
        #[arg(short, long)]
        overwrite: bool,
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Show the problem description for a particular day
    Desc {
        day: u8,
        #[arg(short, long)]
        dry_run: bool,
        #[arg(short, long)]
        all_html: bool,
        #[arg(short, long)]
        part_2: bool,
    },
    /// Run the problem code for one of the days
    #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
    Run {
        day: u8,
        #[arg(short, long)]
        example: bool,
        #[arg(short, long)]
        part_2: bool,
        other_args: Vec<String>,
    },
}

pub fn split_options(opts: Vec<String>) -> Result<Vec<(String, String)>> {
    let mut it = opts.iter();
    let mut arg_pairs = Vec::new();
    loop {
        match it.next() {
            Some(full_arg) => {
                if full_arg.starts_with("--X") {
                    let x = &full_arg[3..];
                    if x.contains("=") {
                        let (a, v) = x.split_once("=").unwrap();
                        arg_pairs.push((a.to_string(), v.to_string()));
                    } else {
                        arg_pairs.push((x.to_string(), it.next().unwrap().to_string()));
                    }
                } else {
                    return Err(eyre!("All day-specific options should start with '--X'"));
                }
            }
            None => break,
        }
    }
    Ok(arg_pairs)
}

// pub fn make_solution<T, U, V>(day: u8) -> StructSolution<T, U, V> {
//     match day {
//         1 => crate::day1::make_sol(),
//         2 => crate::day2::make_sol(),
//         _ => panic!("Day {} not implemented", day),
//     }
// }
