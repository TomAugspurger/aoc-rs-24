use aoc_rs_24::{
    d01, d02, d03, d04, d05, d06, d07, d08, d09, d10, d11, d12, d13, d14, d15, d16, d17, d18, d19,
    d20, d22, d23, d24, d25,
};
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

    D9 {
        #[arg(short, long)]
        input: PathBuf,
    },
    D10 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, action)]
        as_rating: bool,
    },

    D11 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, default_value_t = 25)]
        n_blinks: u64,
    },
    D12 {
        #[arg(short, long)]
        input: PathBuf,
    },
    D13 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, default_value_t = 0)]
        offset: u64,
    },
    D14 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, default_value_t = 100)]
        n_iter: usize,

        #[arg(short, long, default_value_t = 101)]
        width: usize,

        #[arg(long, default_value_t = 103)]
        height: usize,
    },

    D15 {
        #[arg(short, long)]
        input: PathBuf,
    },

    D16 {
        #[arg(short, long)]
        input: PathBuf,
    },

    D17 {
        #[arg(short, long)]
        input: PathBuf,
    },
    D18 {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(long, default_value_t = 71)]
        n_rows: usize,

        #[arg(long, default_value_t = 71)]
        n_cols: usize,

        #[arg(long, default_value_t = 1024)]
        n_steps: usize,
    },

    D19 {
        #[arg(short, long)]
        input: PathBuf,
    },
    D20 {
        #[arg(short, long)]
        input: PathBuf,
    },
    D22 {
        #[arg(short, long)]
        input: PathBuf,
    },
    D23 {
        #[arg(short, long)]
        input: PathBuf,
    },
    D24 {
        #[arg(short, long)]
        input: PathBuf,
    },
    D25 {
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
            println!("{result}");
        }

        Some(Commands::D9 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d09::run(&input);
            // 6448989155953
            println!("{result}");
        }

        Some(Commands::D10 { input, as_rating }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d10::main(&input, *as_rating);
            // 531
            println!("{result}");
        }
        Some(Commands::D11 { input, n_blinks }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d11::main(&input, *n_blinks);
            // 531
            println!("{result}");
        }
        Some(Commands::D12 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d12::main(&input);
            println!("{result}");
        }
        Some(Commands::D13 { input, offset }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d13::main(&input, *offset);
            // 531
            println!("{result}");
        }

        Some(Commands::D14 {
            input,
            n_iter,
            width,
            height,
        }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d14::main(&input, *n_iter, *width, *height);
            // 531
            println!("{result}");
        }

        Some(Commands::D15 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d15::main(&input);
            println!("{result}");
        }

        Some(Commands::D16 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d16::main(&input);
            println!("{result}");
        }
        Some(Commands::D17 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d17::main(&input);
            println!("{result}");
        }

        Some(Commands::D18 {
            input,
            n_rows,
            n_cols,
            n_steps,
        }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d18::main(&input, *n_rows, *n_cols, *n_steps);
            println!("{result}");
        }

        Some(Commands::D19 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d19::main(&input);
            println!("{result}");
        }

        Some(Commands::D20 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d20::main(&input);
            println!("{result}");
        }

        Some(Commands::D22 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d22::main(&input);
            println!("{result}");
        }
        Some(Commands::D23 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d23::main(&input);
            println!("{result}");
        }
        Some(Commands::D24 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d24::main(&input);
            println!("{result}");
        }
        Some(Commands::D25 { input }) => {
            let input = fs::read_to_string(input).expect("Failed to read file.");
            let result = d25::main(&input);
            println!("{result}");
        }

        None => {}
    }
}
