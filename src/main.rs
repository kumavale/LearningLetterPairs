use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer};
use askama::Template;
use sqlx::postgres::PgPoolOptions;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
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

async fn list() -> Result<HttpResponse, Error> {
    let html = ListTemplate {
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let pool = PgPoolOptions::new()
    //    .max_connections(5)
    //    .connect("postgres://postgres:postgres@localhost/xxxx").await.unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/list", web::get().to(list))
            .service(fs::Files::new("/static", ".").show_files_listing())
    })
        .bind("localhost:8080")?
        .run()
        .await
}
