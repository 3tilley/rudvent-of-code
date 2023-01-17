use crate::solution::StructSolution;
use clap::Parser;
use clap::Subcommand;

pub const BANNER: &str = r#"
╭━━╮╱╱╱╱╱╱╭━━┳╮╱╱╭┳━╮╭━╮╱╱╭╮╱╱╱╱╱╱╱╱╭╮╱╱╱╭━╮╭━╮╱╱╭╮
┃━━╋┳┳━┳━┳╋╮╭╋╋╮╭┫┃━┫┃╋┣┳┳╯┣━┳━┳━┳━┳┫╰╮╭━┫━┫┃╭╋━┳╯┣━╮
┣━━┃┃┃╋┃┻┫╭┫┃┃┃╰┫╰╋━┃┃╮┫┃┃╋┣╮┃╭┫┻┫┃┃┃╭┫┃╋┃╭╯┃╰┫╋┃╋┃┻┫
╰━━┻━┫╭┻━┻╯╰╯╰┻━┻━┻━╯╰┻┻━┻━╯╰━╯╰━┻┻━┻━╯╰━┻╯╱╰━┻━┻━┻━╯
╱╱╱╱╱╰╯
"#;

#[derive(Parser)]
#[command(author, about = "I am a program", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcmd: Option<Commands>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
}

#[derive(Subcommand)]
pub enum Commands {
    Fetch {
        day: u8,
        #[arg(short, long)]
        overwrite: bool,
        #[arg(short, long)]
        dry_run: bool,
    },
    Desc {
        day: u8,
        #[arg(short, long)]
        dry_run: bool,
        #[arg(short, long)]
        all_html: bool,
        #[arg(short, long)]
        part_2: bool,
    },
    Run {
        day: u8,
        #[arg(short, long)]
        example: bool,
        #[arg(short, long)]
        part_2: bool,
    },
}

// pub fn make_solution<T, U, V>(day: u8) -> StructSolution<T, U, V> {
//     match day {
//         1 => crate::day1::make_sol(),
//         2 => crate::day2::make_sol(),
//         _ => panic!("Day {} not implemented", day),
//     }
// }
