use clap::Parser;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufRead;
use std::{env, io};
use sudoku::Sudoku;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
struct Puzzle {
    id: String,
    puzzle: String,
    solution: String,
    num_clues: i16,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Puzzle(id: {}, puzzle: {}, solution: {}, num_clues: {})",
            &self.id, &self.puzzle, &self.solution, &self.num_clues
        )
    }
}
async fn insert_puzzle(puzzle: Puzzle, pool: &Pool<Postgres>) {
    println!("Inserting {}", puzzle);
    sqlx::query!(
        "INSERT INTO puzzles (id, puzzle, solution, num_clues) VALUES ($1, $2, $3, $4)",
        puzzle.id,
        puzzle.puzzle,
        puzzle.solution,
        puzzle.num_clues
    )
    .execute(pool)
    .await
    .expect("Managed to insert");
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    count: Option<u32>,

    #[clap(short, long)]
    file: Option<String>,
}

fn puzzle_from_sudoku(puzzle: String, solution: String) -> Puzzle {
    let num_clues = puzzle.clone().chars().filter(|f| *f != '.').count() as i16;
    Puzzle {
        id: ulid::Ulid::new().to_string(),
        puzzle,
        solution,
        num_clues,
    }
}

fn generate_puzzle() -> Puzzle {
    let puzzle = Sudoku::generate_unique();
    puzzle_from_sudoku(
        puzzle.to_string(),
        puzzle
            .solve_unique()
            .expect("Just generated it")
            .to_string(),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args = Args::parse();
    match args.file {
        Some(f) => {
            let file = File::open(f).expect("File must exist");
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(
                    env::var("DATABASE_URL")
                        .expect("DATABASE_URL must be set")
                        .as_str(),
                )
                .await
                .expect("Failed to connect to POSTGRES");

            for line in io::BufReader::new(file).lines() {
                match line {
                    Ok(l) => match Sudoku::from_str_line(&l) {
                        Ok(s) => {
                            let p = puzzle_from_sudoku(
                                s.to_string(),
                                s.solve_unique()
                                    .expect("Generated from an already solved puzzle")
                                    .to_string(),
                            );
                            insert_puzzle(p, &pool).await;
                        }
                        _ => {}
                    },
                    _ => {
                        println!("Could not find a puzzle");
                    }
                };
            }
        }
        None => {}
    }
    match args.count {
        Some(count) => {
            if count > 0 {
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(
                        env::var("DATABASE_URL")
                            .expect("DATABASE_URL must be set")
                            .as_str(),
                    )
                    .await
                    .expect("Failed to connect to POSTGRES");
                for _ in 0..count {
                    let puzzle = generate_puzzle();
                    insert_puzzle(puzzle, &pool).await;
                }
            }
        }
        _ => println!("Got told to generate 0 puzzles"),
    }
    Ok(())
}
