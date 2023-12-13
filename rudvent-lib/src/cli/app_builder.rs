use crate::cli::app::App;
use crate::cli::Cli;
use crate::printer::Printer;
use crate::types::SolutionBuilders;
use clap::Parser;
use clap_verbosity_flag::{Level};
use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::info;

pub struct AppBuilder {
    pub project_root: Option<PathBuf>,
    pub data_directory: Option<PathBuf>,
    pub days_directory: Option<PathBuf>,
    pub log_level: Option<Level>,
    pub year: Option<u16>,
    pub day_format: Option<String>,
    pub auth_token: Option<String>,
    pub solutions: SolutionBuilders,
}

impl AppBuilder {
    pub fn new(solutions: SolutionBuilders) -> AppBuilder {
        AppBuilder {
            project_root: None,
            data_directory: None,
            days_directory: None,
            log_level: None,
            year: None,
            day_format: None,
            auth_token: None,
            solutions,
        }
    }

    pub fn with_data_directory(mut self, data_directory: PathBuf) -> AppBuilder {
        self.data_directory = Some(data_directory);
        self
    }

    pub fn build(self) -> App {
        let cli_args = Cli::parse();
        // It seems log::LevelFilter -> &str -> tracing::LevelFilter is the easiest way to convert
        let verbosity = cli_args.verbose.log_level_filter().as_str();
        tracing_subscriber::fmt()
            .with_max_level(tracing::level_filters::LevelFilter::from_str(verbosity).unwrap())
            .init();
        color_eyre::install().unwrap();
        info!("Building app");
        let manifest_dir = env::var("CARGO_MANIFEST_DIR");
        let auth_token = {
            self.auth_token.unwrap_or({
                dotenvy::var("AUTH_TOKEN").unwrap_or({
                let path = Path::new(
                    &manifest_dir
                        .clone()
                        .expect("If not running from cargo auth-token must be set"),
                )
                    .join(".env");
                dotenvy::from_path(&path).expect(&*format!("Failed to load .env file from {:?}", path));
                dotenvy::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set in .env")})
        })};
        let project_root = manifest_dir.clone().ok().map(|d| PathBuf::from(d));
        App {
            project_root,
            data_directory: self.data_directory.unwrap_or(
                PathBuf::from(
                    manifest_dir
                        .clone()
                        .expect("If not running from cargo data-directory must be set"),
                )
                .join("data"),
            ),
            days_directory: self.days_directory.unwrap_or(
                PathBuf::from(
                    &manifest_dir.expect("If not running from cargo days-directory must be set"),
                )
                .join("src/days"),
            ),
            log_level: self
                .log_level
                .unwrap_or(cli_args.verbose.log_level().unwrap_or(Level::Info)),
            cli: cli_args,
            printer: Printer {},
            // TODO: Handle this better, maybe assume that it's the current year?
            year: self.year.unwrap_or(2023),
            day_format: self.day_format.unwrap_or("day_{day}.rs".to_string()),
            solutions: self.solutions,
            auth_token,
        }
    }

    pub fn with_log_level(mut self, log_level: Level) -> AppBuilder {
        self.log_level = Some(log_level);
        self
    }

    pub fn with_days_directory(mut self, days_directory: PathBuf) -> AppBuilder {
        self.days_directory = Some(days_directory);
        self
    }

    pub fn with_year(mut self, year: u16) -> AppBuilder {
        self.year = Some(year);
        self
    }

    pub fn with_auth_token(mut self, auth_token: String) -> AppBuilder {
        self.auth_token = Some(auth_token);
        self
    }
}
