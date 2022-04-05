use std::fs::File;
use solver::solver;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    puzzle: Option<String>,

    #[clap(short, long)]
    file: Option<String>
}

fn main() {
    let args = Args::parse();
    match args.puzzle {
        Some(p) => {
            solver::treat_grid(&p);
        }
        _ => {}
    }
    match args.file {
        Some(f) => {
            solver::solve_file(File::open(f).expect("File must exist"))
        }
        _ => {}
    }
}
