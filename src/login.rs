use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use askama::Template;
use futures_util::stream::StreamExt as _;
use sqlx::PgPool;
use crate::util;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

pub async fn login() -> Result<HttpResponse, Error> {
    let html = LoginTemplate {};
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}
