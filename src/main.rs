use clap::Parser;

mod aoc_cli;
use aoc_cli::date::{AocDate, Day};

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
    Calendar {
        #[clap(long, short = 'e', action = clap::ArgAction::SetTrue)]
        edit_readme: bool,
        #[clap(long, short = 'd')]
        day: Option<Day>,
    },
    Download {
        #[clap(long, short = 'o', action = clap::ArgAction::SetTrue)]
        overwrite: bool,
    },
    Template {
        #[clap(long, short = 'd', action = clap::ArgAction::SetTrue)]
        download_input: bool,
        #[clap(long, short = 'o', action = clap::ArgAction::SetTrue)]
        overwrite: bool,
    },
    Solve {
        // part: u8,
        // submit_part: Option<u8>,
        #[clap(long, short = 'r', action = clap::ArgAction::SetTrue)]
        release: bool,
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
        AdventOfCodeCommand::Calendar {edit_readme, day} => {
            aoc_cli::calendar(&args.get_aoc_date(), edit_readme, day).unwrap();
        },
        AdventOfCodeCommand::Template { download_input, overwrite} => {
            aoc_cli::template(&args.get_aoc_date(), download_input, overwrite);
        },
        AdventOfCodeCommand::Solve { release } => {
            aoc_cli::solve(&args.get_aoc_date(), release);
        }
    }
}
