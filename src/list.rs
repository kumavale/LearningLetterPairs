use actix_web::{web, Error, HttpResponse};
use askama::Template;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    lists: Vec<Vec<LetterPair>>,
}

#[derive(sqlx::FromRow, Clone, Debug)]
struct LetterPair {
    pub initial:  String,
    pub _next:     String,
    pub name:     String,
    pub objects:  Vec<String>,
    pub image:    String,
    pub hiragana: String,
}

pub async fn list(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let rows = sqlx::query_as::<_, LetterPair>("
        SELECT
            list.initial,
            list.next,
            list.name,
            list.objects,
            list.image,
            hiragana.name AS hiragana
        FROM
            list
        LEFT JOIN
            hiragana
        ON
            list.initial = hiragana.id
        WHERE
            list.image <> '' OR objects <> '{}'
        ORDER BY
            list.name
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

