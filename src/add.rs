use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use askama::Template;
use futures_util::stream::StreamExt as _;
use sqlx::PgPool;
use crate::util;

#[derive(Template)]
#[template(path = "add.html")]
struct AddTemplate {
    pair:     String,
    letters:  String,
    filename: String,
    message:  String,
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct AddLpParams {
    letters:  Vec<String>,
    filename: String,
}

pub async fn add(pool: web::Data<PgPool>, name: String) -> Result<HttpResponse, Error> {
    let (letters, filename) = if !name.is_empty() {
        let (initial, next) = util::split_pair(&name).unwrap();
        let add_lp_params = sqlx::query_as::<_, AddLpParams>("
            SELECT
                list.objects AS letters,
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
        (add_lp_params.letters.join("\n"), add_lp_params.filename)
    } else {
        ("".to_string(), "".to_string())
    };

    let html = AddTemplate {
        pair: name,
        letters,
        filename,
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
        let mut field  = item.unwrap();
        filename = format!("{}{}.png", initial.to_lowercase(), next.to_lowercase());

        // バイナリをチャンクに分けてwhileループ
        let mut filesize = 0;
        let mut bytes = vec![];
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            filesize += data.len();
            bytes.extend_from_slice(&data[..]);
        }

        if filesize > 0 {
            // ファイル作成
            let filepath = format!("img/{}", &filename);
            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap()
                .unwrap();
            // ファイルへの書き込み
            f = web::block(move || f.write_all(&bytes).map(|_| f))
                .await
                .unwrap()
                .unwrap();
            f.sync_all().unwrap();
        }
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
        pair:     "".to_string(),
        letters:  "".to_string(),
        filename: "".to_string(),
        message:  format!("Sccess ({}{})", &initial, &next),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}
