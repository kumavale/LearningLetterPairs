use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use actix_identity::Identity;
use actix_session::Session;
use askama::Template;
use futures_util::stream::StreamExt as _;
use serde::Deserialize;
use sqlx::PgPool;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::util;

#[derive(Template)]
#[template(path = "add.html")]
struct AddTemplate {
    sign:     String,
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

#[derive(Deserialize)]
pub struct AddParam {
    lp: Option<String>,
}

pub async fn add(
    user: Option<Identity>,
    pool: web::Data<PgPool>,
    session: Session,
    params: web::Query<AddParam>,
) -> Result<HttpResponse, Error> {
    // 現在のURLを保存
    session.insert("current_url", "/add").unwrap();

    if user.is_none() {
        return Ok(util::redirect("/login"));
    }

    let username = user.unwrap().id().unwrap();
    let (letters, filename) = if let Some(lp) = &params.lp {
        let (initial, next) = util::split_pair(&lp).unwrap();
        let add_lp_params = sqlx::query_as::<_, AddLpParams>("
            SELECT
                list.objects AS letters,
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
            .unwrap();
        (add_lp_params.letters.join("\n"), add_lp_params.filename)
    } else {
        ("".to_string(), "".to_string())
    };

    let html = AddTemplate {
        sign: "logout".to_string(),
        pair: params.lp.as_ref().unwrap_or(&"".to_string()).to_string(),
        letters,
        filename,
        message: "".to_string(),
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

pub async fn add_lp(
    user: Option<Identity>,
    pool: web::Data<PgPool>,
    session: Session,
    mut playload: Multipart,
) -> Result<HttpResponse, Error> {
    #[derive(sqlx::FromRow, Clone, Debug)]
    struct Image { filename: String, }

    // 現在のURLを保存
    session.insert("current_url", "/add").unwrap();

    if user.is_none() {
        return Ok(util::redirect("/login"));
    }

    let username = user.unwrap().id().unwrap();
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
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        filename = format!("{}.png", rand_string);

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

            // 画像ファイルが既にある場合は削除する
            let image = sqlx::query_as::<_, Image>("
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
                .await;
            if let Ok(image) = image {
                if image.filename != "" {
                    let filepath = format!("img/{}", image.filename);
                    std::fs::remove_file(filepath).unwrap();
                }
            }
        } else {
            // 画像ファイルが選択されなかった場合、既に画像ファイルが保存されているならそのファイル名。
            // 画像ファイルが保存されていない場合は空文字にする
            let image = sqlx::query_as::<_, Image>("
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
                .await;
            filename = if let Ok(image) = image {
                image.filename
            } else {
                "".to_string()
            }
        }
    }

    // DBへ保存
    sqlx::query(r#"
        INSERT INTO
            list (username, initial, next, objects, image)
        VALUES
            ($1, $2, $3, $4, $5)
        ON CONFLICT
            (username, initial, next)
        DO UPDATE SET
            username=$1, initial=$2, next=$3, objects=$4, image=$5
        "#)
        .bind(&username)
        .bind(&initial)
        .bind(&next)
        .bind(&letters)
        .bind(&filename)
        .execute(&**pool)
        .await
        .unwrap();

    let html = AddTemplate {
        sign:     "logout".to_string(),
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
