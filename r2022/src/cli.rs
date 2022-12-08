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
    },
    Run {
        day: u8,
        #[arg(short, long)]
        example: bool,
    },
}
