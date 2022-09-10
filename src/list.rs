use actix_web::{web, Error, HttpResponse};
use askama::Template;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::add;
use crate::util;

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
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
    submit: String,
}

pub async fn list(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
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
            list.image <> '' OR objects <> '{}'
        ORDER BY
            name
        ")
        .fetch_all(&**pool)
        .await
        .unwrap();

    let html = ListTemplate {
        lists: rows.group_by(|a, b| a.initial == b.initial)
                   .map(|list| list.to_vec())
                   .collect(),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

pub async fn list_modify(pool: web::Data<PgPool>, params: web::Form<ListModifyParams>) -> Result<HttpResponse, Error> {
    #[derive(sqlx::FromRow)]
    struct Image {
        pub filename: String,
    }
    let name   = &params.name;
    let submit = &params.submit;
    let (initial, next) = util::split_pair(name).unwrap();

    match &**submit {
        "Modify" => {
            return add::add(pool, name.to_string()).await;
        }
        "Delete" => {
            // 画像ファイル削除
            let image = sqlx::query_as::<_, Image>("
                SELECT
                    list.image AS filename
                FROM
                    list
                WHERE
                    initial=$1 AND next=$2
                ")
                .bind(&initial)
                .bind(&next)
                .fetch_one(&**pool)
                .await
                .unwrap();
            let filepath = format!("img/{}", image.filename);
            std::fs::remove_file(filepath).unwrap();

            // DBレコード削除
            sqlx::query(r#"
                DELETE FROM
                    list
                WHERE
                    initial=$1 AND next=$2
                "#)
                .bind(&initial)
                .bind(&next)
                .execute(&**pool)
                .await
                .unwrap();
        }
        _ => unreachable!()
    }

    list(pool).await
}
