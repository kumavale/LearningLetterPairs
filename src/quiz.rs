use actix_web::{web, HttpResponse, Responder};
use actix_identity::Identity;
use actix_session::Session;
use askama::Template;
use serde::Serialize;
use sqlx::PgPool;
use crate::util;

#[derive(Template)]
#[template(path = "quiz.html")]
struct QuizTemplate {
    sign:  String,
}

#[derive(sqlx::FromRow, Clone, Debug)]
#[derive(Serialize)]
struct LetterPair {
    pub initial: String,
    pub next:    String,
    pub objects: Vec<String>,
    pub image:   String,
    pub name:    String,
}

#[derive(Serialize)]
struct LetterPairJSON {
    lists: Vec<LetterPair>,
}

pub async fn quiz(
    user: Option<Identity>,
    session: Session,
) -> impl Responder {
    // 現在のURLを保存
    session.insert("current_url", "/quiz").unwrap();

    if user.is_none() {
        return util::redirect("/login");
    }

    let html = QuizTemplate {
        sign: "logout".to_string(),
    };
    let view = html.render().expect("failed to render html");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(view)
}

// すべてのLPをJSON形式で返す
pub async fn lp_json(
    user: Option<Identity>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    if user.is_none() {
        // ログインしていない場合は空のJSONを返す
        let dummy = LetterPairJSON { lists: Vec::new(), };
        return web::Json(dummy);
    }

    let username = user.unwrap().id().unwrap();
    let rows = sqlx::query_as::<_, LetterPair>("
        SELECT
            list.initial,
            list.next,
            list.objects,
            list.image,
            (list.initial || list.next) AS name
        FROM
            list
        WHERE
            username=$1
        ")
        .bind(&username)
        .fetch_all(&**pool)
        .await
        .unwrap();

    web::Json(LetterPairJSON { lists: rows })
}
