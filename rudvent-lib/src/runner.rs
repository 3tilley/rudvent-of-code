use clap::Parser;
use crate::cli::{Cli, Commands};

pub fn run() {
    let args = Cli::parse();
    match args.sub_cmd {
        Commands::New { .. } => {}
        Commands::Fetch { .. } => {}
        Commands::Desc { .. } => {}
        Commands::Run { .. } => {}
    }
}
