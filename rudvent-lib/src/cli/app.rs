use crate::advent_interactions::DayData;
use crate::cli::new::NewInstructions;
use crate::cli::solve::SolveInstructions;
use crate::cli::{Cli, Commands};
use crate::printer::Printer;
use clap_verbosity_flag::Level;
use color_eyre::eyre::Result;
use std::path::PathBuf;

use crate::types::SolutionBuilders;

#[derive(Debug)]
pub struct App {
    pub project_root: Option<PathBuf>,
    pub data_directory: PathBuf,
    pub days_directory: PathBuf,
    pub log_level: Level,
    pub cli: Cli,
    pub printer: Printer,
    pub year: u16,
    pub day_format: String,
    pub solutions: SolutionBuilders,
    pub auth_token: String,
}

impl App {
    pub fn run(&self) -> Result<()> {
        match &self.cli.sub_cmd {
            Commands::New { day, overwrite, example } => {
                let instructions = NewInstructions {
                    day: *day,
                    overwrite: *overwrite,
                    example: example.clone(),
                    app: self,
                };
                instructions.execute()?;
                Ok(())
            }
            Commands::Fetch {
                day,
                overwrite,
                dry_run,
            } => {
                self.fetch_data(*day, *dry_run)?;
                Ok(())
            }
            Commands::Desc {
                day,
                dry_run,
                all_html,
                part,
            } => {
                println!("Fetching description for day {}", day);
                let day_data = DayData::new(
                    self.year,
                    *day,
                    *dry_run,
                    self.data_directory.clone(),
                    self.auth_token.clone(),
                );
                if *all_html {
                    Ok(println!("{}", day_data.html(part.is_part_1(), true, false)?))
                } else {
                    let html = day_data.html(part.is_part_2(), false, false)?;
                    let pretty = html2text::from_read(html.as_bytes(), 80);
                    Ok(println!("{}", pretty))
                }
            }
            Commands::Solve {
                day,
                example,
                part,
                other_args,
                no_post,
            } => {
                let mut instructions = SolveInstructions {
                    day: *day,
                    example: *example,
                    part_1: part.is_part_1(),
                    other_args: other_args.clone(),
                    solutions: self.solutions,
                    app: self,
                    no_post: *no_post,
                };
                instructions.execute()?;
                Ok(())
            }
        }
    }

    fn fetch_data(&self, day: u8, dry_run: bool) -> Result<()> {
        self.printer
            .print_or_info(&format!("Fetching data {}", day));
        let day_data = DayData::new(
            self.year,
            day,
            dry_run,
            self.data_directory.clone(),
            self.auth_token.clone(),
        );
        day_data.fetch_data()?;
        if dry_run {
            self.printer.success(&*format!(
                "Access successful, but because of --dry-run no data saved for day {}",
                day
            ));
        } else {
            self.printer
                .success(&*format!("Fetched and saved data for day {}", day));
        }
        Ok(())
    }
}
