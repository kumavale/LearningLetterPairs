use actix_web::{web, HttpResponse, Responder};
use actix_identity::Identity;
use actix_session::Session;
use askama::Template;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::util;

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    sign:  String,
    lists: Vec<Vec<LetterPair>>,
}

#[derive(sqlx::FromRow, Clone, Debug)]
struct LetterPair {
    pub initial: String,
    pub next:    String,
    pub objects: Vec<String>,
    pub image:   String,
    pub name:    String,
}

#[derive(Serialize, Deserialize)]
pub struct ListModifyParams {
    name:   String,
}

pub async fn list(
    user: Option<Identity>,
    pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    // 現在のURLを保存
    session.insert("current_url", "/list").unwrap();

    if user.is_none() {
        return util::redirect("/login");
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
            name
        ")
        .bind(&username)
        .fetch_all(&**pool)
        .await
        .unwrap_or_default();

    let html = ListTemplate {
        lists: rows.group_by(|a, b| a.initial == b.initial)
                   .map(|list| list.to_vec())
                   .collect(),
        sign: "logout".to_string(),
    };
    let view = html.render().expect("failed to render html");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(view)
}

pub async fn lp_delete(
    user: Option<Identity>,
    pool: web::Data<PgPool>,
    params: web::Form<ListModifyParams>,
) -> impl Responder {
    if user.is_none() {
        return util::redirect("/login");
    }

    #[derive(sqlx::FromRow)]
    struct Image {
        pub filename: String,
    }
    let (initial, next) = util::split_pair(&params.name).unwrap();
    let username = user.as_ref().unwrap().id().unwrap();

    // 画像ファイル削除
    if let Ok(image) = sqlx::query_as::<_, Image>("
                SELECT
                    list.image AS filename
                FROM
                    list
                WHERE
                    username=$1 AND initial=$2 AND next=$3
                ")
        .bind(&username)
        .bind(&initial)
        .bind(&next)
        .fetch_one(&**pool)
        .await
    {
        if image.filename != "" {
            let filepath = format!("img/{}", image.filename);
            let _ = std::fs::remove_file(filepath);
        }
    }

    // DBレコード削除
    let _ = sqlx::query(r#"
                DELETE FROM
                    list
                WHERE
                    username=$1 AND initial=$2 AND next=$3
                "#)
        .bind(&username)
        .bind(&initial)
        .bind(&next)
        .execute(&**pool)
        .await;

    HttpResponse::Ok().finish()
}
