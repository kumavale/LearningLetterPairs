use actix_web::{web, Error, HttpResponse};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "add.html")]
struct AddTemplate {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddLpParams {
    lp:      String,
    letters: String,
}

pub async fn add() -> Result<HttpResponse, Error> {
    let html = AddTemplate {
        message: "".to_string(),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

pub async fn add_lp(params: web::Form<AddLpParams>) -> Result<HttpResponse, Error> {
    let html = AddTemplate {
        message: format!("Sccess ({})", params.lp),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

