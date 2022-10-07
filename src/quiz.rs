use actix_web::{web, Error, HttpResponse, Responder};
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
    lists: Vec<Vec<LetterPair>>,
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

pub async fn shuffle_lp(
    user: Option<Identity>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    if user.is_none() {
        // ログインしていない場合は空のJSONを返す
        let dummy = LetterPairJSON { lists: Vec::new(), };
        return Ok(web::Json(dummy));
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
        ORDER BY
            RANDOM()
        ")
        .bind(&username)
        .fetch_all(&**pool)
        .await
        .unwrap();

    let shuffle_lp = LetterPairJSON {
        lists: rows.group_by(|a, b| a.initial == b.initial)
                   .map(|list| list.to_vec())
                   .collect(),
    };

    Ok(web::Json(shuffle_lp))
}
