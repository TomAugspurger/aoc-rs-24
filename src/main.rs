use aoc_rs_24::{d01, d02, d03};
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
                d03::evaluate_switches(&input)  // 84893551 is too high
            }
            else {
                d03::evaluate(&input) // 160672468
            };
            println!("{result}"); //
        }
        None => {}
    }
}
