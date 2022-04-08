use std::fs::File;
use std::io;
use std::io::BufRead;
use std::time::{Duration, Instant};
use sudoku::parse_errors::LineParseError;
use sudoku::Sudoku;

fn parse_grid(grid_string: &str) -> Result<Sudoku, LineParseError> {
    Sudoku::from_str_line(grid_string)
}

fn solve_sudoku(sudoku: Sudoku) -> Option<Sudoku> {
    sudoku.solve_unique()
}

pub fn treat_grid(grid_string: &str, verbose: bool) -> Duration {
    let sudoku = Sudoku::from_str_line(grid_string).expect("Should only pass string if valid");
    let now = Instant::now();
    let solved = solve_sudoku(sudoku);
    let duration = now.elapsed();
    if verbose {
        match solved {
            Some(s) => {
                println!("Grid complete ! in {} us", (duration.as_micros()));
                println!("{}", s.to_str_line());
            }
            None => {
                println!("Couldn't solve the puzzle in {} us", duration.as_micros());
            }
        }
    }
    duration
}

pub struct Puzzle {
    pub puzzle: String,
    pub solution: String,
}

pub fn generate_puzzle() -> Puzzle {
    let sudoku = Sudoku::generate_unique();
    Puzzle {
        puzzle: sudoku.to_string(),
        solution: sudoku.solve_unique().expect("").to_string(),
    }
}

pub fn from_string(puzzle: &str) -> Result<Sudoku, LineParseError> {
    Sudoku::from_str_line(puzzle)
}

pub fn solve_file(f: File, verbose: bool) {
    let lines = io::BufReader::new(f).lines();
    let mut durations: Vec<Duration> = vec![];
    for line in lines {
        if let Ok(l) = line {
            durations.push(treat_grid(&l, verbose));
        }
    }
    let micros = durations
        .iter()
        .map(|d| d.as_nanos())
        .collect::<Vec<u128>>();
    let totaltime: u128 = micros.iter().sum();
    let number_of_puzzles: u128 =
        u128::try_from(micros.len()).expect("usize to u128 should be fine");
    let max: &u128 = micros.iter().max().expect("There should be durations");
    let min: &u128 = micros.iter().min().expect("There should be durations");
    println!("Spent {} s in total for solving {} puzzles. avg per puzzle = {} ns or roughly {} puzzles/sec", totaltime / 1_000_000_000, micros.len(), totaltime / number_of_puzzles, 1_000_000_000 / (totaltime / number_of_puzzles));
    println!("Slowest puzzle took {} ns", max);
    println!("Fastest puzzle took {} ns", min);
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io;
    use std::io::BufRead;
    use sudoku::Sudoku;

    use crate::solver::CellValue;
    use crate::solver::{empty_grid, solve_grid, Grid};

    use super::parse_grid;
    fn empty_grid() -> Grid {
        [CellValue::Possibilities([true; 9]); 81]
    }

    #[test]
    fn an_empty_grid_can_be_parsed() {
        let grid =
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(81, grid.len());
        let parsed_grid = parse_grid(grid);
        assert!(parsed_grid.is_ok());
    }

    #[test]
    fn a_solved_grid_will_be_parsed_to_all_values() {
        let solved_grid =
            "581672439792843651364591782438957216256184973179326845845219367913768524627435198";
        assert_eq!(81, solved_grid.len());
        let parsed_grid = parse_grid(solved_grid).expect("Valid sudoku should give a sudoku board");
        assert!(parsed_grid.is_solved())
    }

    #[test]
    fn can_solve_banal_case_with_one_unknown() {
        let challenge =
            "081672439792843651364591782438957216256184973179326845845219367913768524627435198";
        let parsed = parse_grid(challenge).expect("Valid sudoku should give a sudoku board");
        let solved = parsed.solve_unique();
        assert!(match solved {
            Some(g) => {
                let s = g[0];
                let n = g[1];
                println!("{}", s);
                println!("{}", n);
                true
            }
            _ => false,
        });
    }
    #[test]
    fn can_solve_top_95_from_norvig() {
        let lines =
            io::BufReader::new(File::open("top95.txt").expect("File needs to be present")).lines();
        for line in lines {
            if let Ok(l) = line {
                let parsed = parse_grid(&*l);
                let solved = solve_grid(parsed);
                assert!(solved.is_some())
            }
        }
    }
}
