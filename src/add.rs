use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use askama::Template;
use futures_util::stream::StreamExt as _;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::util;

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

pub async fn add_lp(pool: web::Data<PgPool>, mut playload: Multipart) -> Result<HttpResponse, Error> {
    let mut initial  = String::new();
    let mut next     = String::new();
    let mut filename = String::new();
    let mut letters  = Vec::new();

    // lp
    if let Some(item) = playload.next().await {
        let mut field = item.unwrap();
        while let Some(chunk) = field.next().await {
            let data: &actix_web::web::Bytes = &chunk.unwrap();
            let name = std::str::from_utf8(data).unwrap();
            (initial, next) = util::split_pair(name).unwrap();
        }
    }

    // letters
    if let Some(item) = playload.next().await {
        let mut field = item.unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            letters.extend_from_slice(
                std::str::from_utf8(&data)
                .unwrap()
                .parse::<String>()
                .unwrap()
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>()
                .as_slice()
            );
        }
    }

    // image
    if let Some(item) = playload.next().await {
        let mut field = item.unwrap();
        filename = format!("{}{}.png", initial.to_lowercase(), next.to_lowercase());
        let filepath = format!("img/{}", &filename);

        // ファイル作成
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap()
            .unwrap();

        // バイナリをチャンクに分けてwhileループ
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // ファイルへの書き込み
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap()
                .unwrap();
            }

        f.sync_all().unwrap();
    }

    // DBへ保存
    sqlx::query(r#"
        INSERT INTO
            list (initial, next, objects, image)
        VALUES
            ($1, $2, $3, $4)
        ON CONFLICT
            (initial, next)
        DO UPDATE SET
            initial=$1, next=$2, objects=$3, image=$4
        "#)
        .bind(&initial)
        .bind(&next)
        .bind(&letters)
        .bind(&filename)
        .execute(&**pool)
        .await
        .unwrap();

    let html = AddTemplate {
        message: format!("Sccess ({}{})", &initial, &next),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

