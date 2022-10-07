#![feature(slice_group_by)]

mod add;
mod create;
mod crypt;
mod list;
mod login;
mod quiz;
mod util;

use actix_files as fs;
use actix_web::{cookie::Key, web, App, Error, HttpResponse, HttpServer};
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use askama::Template;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    username: String,
    sign:     String,
}

async fn index(
    user: Option<Identity>,
    session: Session,
) -> Result<HttpResponse, Error> {
    // 現在のURLを保存
    session.insert("current_url", "/").unwrap();

    let html = if let Some(user) = user {
        IndexTemplate {
            username: user.id().unwrap(),
            sign:     "logout".to_string(),
        }
    } else {
        IndexTemplate {
            username: "".to_string(),
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
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(),secret_key.clone())
                .cookie_secure(false)
                .build())
            .route("/", web::get().to(index))
            .route("/list", web::get().to(list::list))
            .route("/lp_delete", web::post().to(list::lp_delete))
            .route("/add", web::get().to(add::add))
            .route("/add", web::post().to(add::add_lp))
            .route("/quiz", web::get().to(quiz::quiz))
            .route("/login", web::get().to(login::login))
            .route("/login", web::post().to(login::process_login))
            .route("/logout", web::get().to(login::process_logout))
            .route("/create", web::get().to(create::create))
            .route("/create", web::post().to(create::create_account))
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
