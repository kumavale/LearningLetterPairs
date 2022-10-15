#![feature(slice_group_by)]

mod add;
mod create;
mod crypt;
mod error;
mod index;
mod list;
mod login;
mod quiz;
mod util;

use actix_files as fs;
use actix_web::{cookie::Key, web, App, HttpServer};
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect(&util::database_url()).await.unwrap();
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(),secret_key.clone())
                .cookie_secure(false)
                .build())
            .route("/", web::get().to(index::index))
            .route("/list", web::get().to(list::list))
            .route("/lp_delete", web::post().to(list::lp_delete))
            .route("/add", web::get().to(add::add))
            .route("/add", web::post().to(add::add_lp))
            .route("/quiz", web::get().to(quiz::quiz))
            .route("/shuffle_lp", web::post().to(quiz::shuffle_lp))
            .route("/login", web::get().to(login::login))
            .route("/login", web::post().to(login::process_login))
            .route("/logout", web::get().to(login::process_logout))
            .route("/create", web::get().to(create::create))
            .route("/create", web::post().to(create::create_account))
            .service(fs::Files::new("/static", ".").show_files_listing())
            .default_service(web::get().to(error::not_found))
    })
        .bind("app:80")?
        .run()
        .await
}
