use actix_web::{HttpResponse, Responder};
use actix_identity::Identity;
use actix_session::Session;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    username: String,
    sign:     String,
}

pub async fn index(
    user: Option<Identity>,
    session: Session,
) -> impl Responder {
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
    HttpResponse::Ok()
        .content_type("text/html")
        .body(view)
}
