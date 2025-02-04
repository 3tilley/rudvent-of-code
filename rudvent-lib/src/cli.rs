pub mod app;
pub mod app_builder;
mod new;
mod solve;

pub use app::App;
pub use app_builder::AppBuilder;

use clap::Subcommand;
use clap::{Args, Parser};
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

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct Part {
    #[arg(short = '1', long, action, default_value_t = true)]
    part_1: bool,
    #[arg(short = '2', long, action)]
    part_2: bool,
}

impl Part {
    pub fn is_part_1(&self) -> bool {
        // part_2 is used here because part_1=true even when --part-2 is supplied
        // I wish there was a better way of doing this in Clap
        !self.part_2
    }

    pub fn is_part_2(&self) -> bool {
        self.part_2
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create the template for a new day
    New {
        day: u8,
        #[arg(short, long)]
        overwrite: bool,
        #[arg(short, long)]
        example: Option<String>
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
        #[command(flatten)]
        part: Part,
    },
    /// Run the problem code for one of the days
    #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
    Solve {
        day: u8,
        #[arg(short, long)]
        example: bool,
        #[command(flatten)]
        part: Part,
        other_args: Vec<String>,
        #[arg(short, long)]
        no_post: bool,
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
