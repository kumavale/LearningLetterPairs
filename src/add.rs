use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use askama::Template;
use futures_util::stream::StreamExt as _;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    let mut initial = "";
    let mut next = "";
    let mut name = "".to_string();
    let mut filename = "".to_string();
    let mut letters = Vec::new();

    // lp
    if let Some(item) = playload.next().await {
        let mut field = item.unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            name = std::str::from_utf8(&data).unwrap().parse().unwrap();
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
        (initial, next) = split_pair(&name).unwrap();
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
            list (initial, next, name, objects, image)
        VALUES
            ($1, $2, $3, $4, $5)
        ON CONFLICT
            (initial, next)
        DO UPDATE SET
            initial=$1, next=$2, name=$3, objects=$4, image=$5
        "#)
        .bind(&initial)
        .bind(&next)
        .bind(&name)
        .bind(&letters)
        .bind(&filename)
        .execute(&**pool)
        .await
        .unwrap();

    let html = AddTemplate {
        message: format!("Sccess ({})", &name),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

#[allow(clippy::iter_nth_zero)]
fn split_pair(pair: &str) -> Result<(&str, &str), &str> {
    use std::collections::HashMap;
    use once_cell::sync::Lazy;

    static HIRAGARA_TABLE: Lazy<HashMap<char, &str>> = Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert('あ', "A");  m.insert('い', "I");  m.insert('う', "U");  m.insert('え', "E");
        m.insert('か', "KA"); m.insert('き', "KI"); m.insert('く', "KU"); m.insert('け', "KE");
        m.insert('さ', "SA"); m.insert('し', "SI"); m.insert('す', "SU"); m.insert('せ', "SE");
        m.insert('た', "TA"); m.insert('ち', "TI"); m.insert('つ', "TU"); m.insert('て', "TE");
        m.insert('な', "NA"); m.insert('に', "NI"); m.insert('ぬ', "NU"); m.insert('ね', "NE");
        m.insert('は', "HA"); m.insert('ひ', "HI"); m.insert('ふ', "HU"); m.insert('へ', "HE");
        m
    });

    let initial = HIRAGARA_TABLE.get(&pair.chars().nth(0).unwrap());
    let next    = HIRAGARA_TABLE.get(&pair.chars().nth(1).unwrap());

    if initial.is_none() || next.is_none() {
        return Err("invalid letter pair");
    }

    Ok((initial.unwrap(), next.unwrap()))
}

