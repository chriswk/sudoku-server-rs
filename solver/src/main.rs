use std::fs::File;
use solver::solver;
use clap::Parser;
use sudoku::Sudoku;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    puzzle: Option<String>,

    #[clap(short, long)]
    file: Option<String>,

    #[clap(short, long)]
    count: Option<u32>,

    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    match args.puzzle {
        Some(p) => {
            solver::treat_grid(&p, args.verbose);
        }
        _ => {}
    }
    match args.file {
        Some(f) => {
            solver::solve_file(File::open(f).expect("File must exist"), args.verbose)
        }
        _ => {}
    }

    match args.count {
        Some(count) => {
            for _ in 0..count {
                let s = Sudoku::generate_unique();
                println!("{}", s);
            }
        }
        _ => {}
    }
}
