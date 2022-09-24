#![feature(slice_group_by)]

mod add;
mod list;
mod login;
mod util;

use actix_files as fs;
use actix_web::{cookie::Key, web, App, Error, HttpResponse, HttpServer};
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use askama::Template;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    username: String,
    sign:     String,
}

async fn index(user: Option<Identity>) -> Result<HttpResponse, Error> {
    let html = if let Some(user) = user {
        IndexTemplate {
            username: user.id().unwrap(),
            sign:     "logout".to_string(),
        }
    } else {
        IndexTemplate {
            username: "Anonymous".to_string(),
            sign:     "login".to_string(),
        }
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect(&database_url()).await.unwrap();
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone()
                )
                .cookie_secure(false)
                .build()
            )
            .route("/", web::get().to(index))
            .route("/list", web::get().to(list::list))
            .route("/list", web::post().to(list::list_modify))
            .route("/add", web::get().to(add::add))
            .route("/add", web::post().to(add::add_lp))
            .route("/login", web::get().to(login::login))
            .route("/login", web::post().to(login::process_login))
            .route("/logout", web::get().to(login::process_logout))
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
