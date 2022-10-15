use actix_web::{HttpResponse, Responder};
use actix_identity::Identity;
use askama::Template;

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate {
    sign: String,
}

pub async fn not_found(
    user: Option<Identity>,
) -> impl Responder {
    let html = if user.is_some() {
        NotFoundTemplate { sign: "logout".to_string(), }
    } else {
        NotFoundTemplate { sign: "login".to_string(), }
    };
    let view = html.render().expect("failed to render html");
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(view)
}
