use aoc_rs_24::{d01, d02, d03, d04, d05, d06, d07, d08};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    D1 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, action)]
        as_score: bool,
    },

    D2 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, action)]
        damped: bool,
    },

    D3 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, action)]
        switches: bool,
    },

    D4 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, action)]
        xs: bool,
    },

    D5 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, action)]
        fix_only: bool,
    },

    D6 {
        #[arg(short, long)]
        input: PathBuf,
    },

    D7 {
        #[arg(short, long)]
        input: PathBuf,
    },

    D8 {
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::D1 { input, as_score }) => {
            let data = d01::parse_input(&fs::read_to_string(input).expect("Failed to read file."));
            if *as_score {
                let result = d01::similarity_score(&data);
                println!("{result}"); // 1830467
            } else {
                let result = d01::find_distance(&data);
                println!("{result}"); // 26674158
            }
        }
        Some(Commands::D2 { input, damped }) => {
            let data = d02::parse_input(&fs::read_to_string(input).expect("Failed to read file."));
            let result = d02::count_safe(&data, *damped);
            println!("{result}"); // 442, 493
        }
        Some(Commands::D3 { input, switches }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");

            let result = if *switches {
                d03::evaluate_switches(&input) // 84893551 is too high
            } else {
                d03::evaluate(&input) // 160672468
            };
            println!("{result}"); //
        }
        Some(Commands::D4 { input, xs }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d04::count_xmas(&input, *xs);
            // 2454 and 1858
            println!("{result}");
        }
        Some(Commands::D5 { input, fix_only }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d05::check(&input, fix_only);
            println!("{result}");
        }
        Some(Commands::D6 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d06::count_positions(&input);
            println!("{result}");
        }
        Some(Commands::D7 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d07::total_calibration_result(&input);
            // part 1: 20281182715321 in ~1s
            // part 2: 159490400628354 (in 22s)
            println!("{result}");
        }

        Some(Commands::D8 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d08::count_antinodes(&input);
            // 394 is too high.
            println!("{result}");
        }

        None => {}
    }
}
