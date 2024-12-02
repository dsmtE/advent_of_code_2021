use clap::Parser;

mod aoc_date;
mod aoc_cli;
mod path;
use aoc_date::{AocDate, Day};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct AdventOfCodeArgs {
    #[clap(subcommand)]
    command: AdventOfCodeCommand,

    #[clap(short, long)]
    day: Option<Day>,
}

impl AdventOfCodeArgs {
    fn get_aoc_date(&self) -> AocDate {
        AocDate::new(self.day, None)
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
enum AdventOfCodeCommand {
    Read,
    Calendar,
    Download {
        #[clap(long, short = 'o', action = clap::ArgAction::SetTrue)]
        overwrite: bool,
    },
    Template {
        download_input: Option<bool>,
        overwrite: Option<bool>,
    },
    Solve {
        // part: u8,
        // submit_part: Option<u8>,
        release: Option<bool>,
    }
}

fn main() {
    let args = AdventOfCodeArgs::parse();

    if  cfg!(debug_assertions) {
        println!("{:?}", args);
    }

    match args.command {
        AdventOfCodeCommand::Read => {
            aoc_cli::read(&args.get_aoc_date()).unwrap();
        },
        AdventOfCodeCommand::Download { overwrite } => {
            aoc_cli::download(&args.get_aoc_date(), overwrite).unwrap();
        },
        AdventOfCodeCommand::Calendar => {
            aoc_cli::calendar(&args.get_aoc_date()).unwrap();
        },
        AdventOfCodeCommand::Template { download_input, overwrite} => {
            aoc_cli::template(&args.get_aoc_date(), download_input.unwrap_or(false), overwrite.unwrap_or(false));
        },
        AdventOfCodeCommand::Solve { release } => {
            aoc_cli::solve(&args.get_aoc_date(), release.unwrap_or(false));
        }
    }
}
