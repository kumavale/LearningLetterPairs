use actix_web::{Error, HttpResponse};
use actix_identity::Identity;
use actix_session::Session;
use askama::Template;
use crate::util;

#[derive(Template)]
#[template(path = "quiz.html")]
struct QuizTemplate {
    sign:  String,
}

pub async fn quiz(
    user: Option<Identity>,
    session: Session,
) -> Result<HttpResponse, Error> {
    // 現在のURLを保存
    session.insert("current_url", "/quiz").unwrap();

    if user.is_none() {
        return Ok(util::redirect("/login"));
    }

    let html = QuizTemplate {
        sign: "logout".to_string(),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}
