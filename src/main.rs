use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool};
use rand::Rng;
use std::path::Path;
use actix_web::http::StatusCode;

#[allow(dead_code)]
struct CopyPasta {
    id: i64,
    body: String
}

struct AppState {
 db: SqlitePool
}

async fn connect(filename: impl AsRef<Path>) -> Result<SqlitePool, Error> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(false);

    SqlitePool::connect_with(options).await
}

#[get("/{tail:.*}")]
async fn default(data: web::Data<AppState>) -> HttpResponse {
    let mut rng = rand::thread_rng();
    //let num: i64 = rng.gen_range(0..389137);
    let num: i64 = rng.gen_range(0..9000);
    let copypasta: CopyPasta = sqlx::query_as!(
        CopyPasta,
        r#"SELECT * FROM copypastas WHERE id = ?"#,
        num
    )
    .fetch_one(&data.db)
    .await
    .expect("Failed to query sqlite database.");
    return HttpResponse::build(StatusCode::from_u16(418).unwrap())
        .content_type("text/html")
        .body(format!("<!DOCTYPE html><html><head><meta charset=\"UTF-8\"></head><body>{}</body></html>", copypasta.body));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("DATABASE_URL", "./data/copypastas.sqlite");
    let conn = connect("./data/copypastas.sqlite").await.expect("Failed to open sqlite database.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: conn.clone() }))
            .wrap(middleware::Compress::default())
            .service(default)
    })
    .bind(("0.0.0.0", 6757))?
    .workers(5)
    .run()
    .await
}
