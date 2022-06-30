use std::io::Write;
use actix_web::{web, Error, HttpResponse};
use actix_multipart::Multipart;
use askama::Template;
use serde::{Deserialize, Serialize};
use futures_util::stream::StreamExt as _;

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

pub async fn add_lp(mut playload: Multipart) -> Result<HttpResponse, Error> {
    let mut lp = "".to_string();
    let mut letters = Vec::new();

    while let Some(item) = playload.next().await {
        let mut field = item.unwrap();
        let content_type = field.content_disposition();

        match content_type.get_name().unwrap() {
            "lp" => {
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    lp = std::str::from_utf8(&data).unwrap().parse().unwrap();
                }
                dbg!(&lp);
            }
            "letters" => {
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
                dbg!(&letters);
            }
            "image" => {
                 let filename = format!("{lp}.png");
                 let filepath = format!("img/{}", filename);

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
            _ => unreachable!(),
        }
    }

    let html = AddTemplate {
        message: format!("Sccess ({})", lp),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

