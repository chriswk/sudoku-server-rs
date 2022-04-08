use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use sudoku::Sudoku;

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct PuzzleRow {
    id: String,
    puzzle: String,
    solution: String,
    num_clues: i16,
}

#[get("/puzzles")]
async fn puzzles(db_pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let conn = db_pool.get_ref();
    if let Ok(rows) = sqlx::query_as!(
        PuzzleRow,
        "SELECT id, puzzle, solution, num_clues FROM puzzles"
    )
    .fetch_all(conn)
    .await
    {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&rows).unwrap())
    } else {
        HttpResponse::ServiceUnavailable().finish()
    }
}
#[get("/randompuzzle")]
async fn random_puzzle() -> HttpResponse {
    let solution = Sudoku::generate_filled();
    let puzzle = Sudoku::generate_unique_from(solution);
    let body = PuzzleRow {
        id: "generated".to_string(),
        puzzle: puzzle.to_string(),
        solution: solution.to_string(),
        num_clues: 0,
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&body).expect(""))
}
#[get("/puzzles/{id}")]
async fn get_puzzle(db_pool: web::Data<Pool<Postgres>>, id: web::Path<(String,)>) -> HttpResponse {
    let puzzle = sqlx::query_as!(
        PuzzleRow,
        r#"
        SELECT id, puzzle, solution, num_clues FROM puzzles WHERE id = $1
    "#,
        id.into_inner().0
    )
    .fetch_optional(db_pool.get_ref())
    .await;
    match puzzle {
        Ok(puz) => match puz {
            Some(p) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&p).unwrap()),
            _ => HttpResponse::NotFound().finish(),
        },
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT")
        .map(|f| i32::from_str_radix(&f, 10))
        .unwrap_or(Ok(4200))
        .expect("Found a port");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
                .as_str(),
        )
        .await
        .expect("Failed to connect to POSTGRES");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(puzzles)
            .service(get_puzzle)
            .service(random_puzzle)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
