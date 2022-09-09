#![feature(slice_group_by)]

mod add;
mod list;
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
    let pool = PgPool::connect("postgres://postgres:postgres@localhost/letterpairs").await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/list", web::get().to(list::list))
            .route("/list", web::post().to(list::list_modify))
            .route("/add", web::get().to(add::add))
            .route("/add", web::post().to(add::add_lp))
            .service(fs::Files::new("/static", ".").show_files_listing())
    })
        //.bind("localhost:8080")?
        .bind("192.168.10.101:8080")?
        .run()
        .await
}
