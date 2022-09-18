#![feature(slice_group_by)]

mod add;
mod list;
mod login;
mod util;

use actix_files as fs;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use askama::Template;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

async fn index() -> Result<HttpResponse, Error> {
    let html = IndexTemplate {
        name: "kumavale".to_string(),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect(&database_url()).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/list", web::get().to(list::list))
            .route("/list", web::post().to(list::list_modify))
            .route("/add", web::get().to(add::add))
            .route("/add", web::post().to(add::add_lp))
            .route("/login", web::get().to(login::login))
            .service(fs::Files::new("/static", ".").show_files_listing())
    })
        .bind("app:80")?
        .run()
        .await
}

fn database_url() -> String {
    let postgres_host     = std::env::var("POSTGRES_HOST").unwrap();
    let postgres_port     = std::env::var("POSTGRES_PORT").unwrap();
    let postgres_user     = std::env::var("POSTGRES_USER").unwrap();
    let postgres_password = std::env::var("POSTGRES_PASSWORD").unwrap();
    let postgres_database = std::env::var("POSTGRES_DATABASE").unwrap();

    format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_user,
        postgres_password,
        postgres_host,
        postgres_port,
        postgres_database,
    )
}
