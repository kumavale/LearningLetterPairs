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
    list: Vec<LetterPair>,
}

#[derive(sqlx::FromRow, Clone, Debug)]
struct LetterPair {
    pub initial: String,
    pub next:    String,
    pub name:    String,
    pub objects: Vec<String>,
    pub image:   String,
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
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/letterpairs").await.unwrap();
    let rows = sqlx::query_as::<_, LetterPair>("SELECT * from list")
        .fetch_all(&pool).await.unwrap();

    let html = ListTemplate {
        list: rows.iter().by_ref().take_while(|row|row.initial == "A").cloned().collect(),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
