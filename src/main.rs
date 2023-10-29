use actix_web::http::StatusCode;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool};
use std::path::Path;

#[allow(dead_code)]
struct CopyPasta {
    id: i64,
    body: String,
}

struct AppState {
    db: SqlitePool,
}

async fn connect(filename: impl AsRef<Path>) -> Result<SqlitePool, Error> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(false);

    SqlitePool::connect_with(options).await
}

#[get("/{tail:.*}")]
async fn default(data: web::Data<AppState>) -> HttpResponse {
    match sqlx::query_as!(
        CopyPasta,
        r#"SELECT * FROM copypastas 
        ORDER BY RANDOM()
        LIMIT 1"#
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(copypasta) => {
            HttpResponse::build(StatusCode::from_u16(418).unwrap()) // Safety: valid status code
                .content_type("text/html")
                .body(format!(
                "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"></head><body>{}</body></html>",
                copypasta.body
            ))
        }
        Err(e) => {
            eprintln!("Failed to fetch copypasta: {e:?}");

            const ERROR_BODY: &'static str = "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"></head><body>There was an error. Please try again later.</body></html>";
            HttpResponse::build(StatusCode::from_u16(500).unwrap()) // Safety: valid status code
                .content_type("text/html")
                .body(ERROR_BODY)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match dotenv() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to load .env file: {e:?}. This may not be fatal. Continuing");
        }
    }

    let database_path = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => {
            println!(
                "DATABASE_URL env variable not set, using default \"./data/copypastas.sqlite\". (error: {e:?})"
            );
            std::env::set_var("DATABASE_URL", "./data/copypastas.sqlite");
            "./data/copypastas.sqlite".to_string()
        }
    };

    if !Path::new(&database_path).exists() {
        eprintln!(
            "Database file \"{database_path}\" does not exist. Please create it and try again."
        );
        std::process::exit(1);
    }

    let raw_port = match std::env::var("TEAPOT_FORTUNE_PORT") {
        Ok(port) => port,
        Err(e) => {
            println!(
                "TEAPOT_FORTUNE_PORT env variable not set, using default of 6757. (error: {e:?})"
            );
            std::env::set_var("PORT", "6757");
            "6757".to_string()
        }
    };

    let port = match raw_port.parse::<u16>() {
        Ok(port) => port,
        Err(e) => {
            eprintln!("TEAPOT_FORTUNE_PORT \"{raw_port}\" is not a valid port number. Using default of 6757 (error: {e:?})");
            std::env::set_var("PORT", "6757");
            6757
        }
    };

    let conn = connect(database_path)
        .await
        .expect("Failed to open sqlite database.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: conn.clone() }))
            .wrap(middleware::Compress::default())
            .service(default)
    })
    .bind(("0.0.0.0", port))?
    .workers(5)
    .run()
    .await
}
