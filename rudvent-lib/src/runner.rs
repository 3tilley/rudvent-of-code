use crate::cli::{Cli, Commands};
use clap::Parser;

pub fn run() {
    let args = Cli::parse();
    match args.sub_cmd {
        Commands::New { .. } => {}
        Commands::Fetch { .. } => {}
        Commands::Desc { .. } => {}
        Commands::Solve { .. } => {}
    }
}
