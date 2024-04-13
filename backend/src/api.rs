use axum::extract::{Json, Multipart, State};
use google_cloud_storage::client::Client;
use google_cloud_storage::client::ClientConfig;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use image::io::Reader as ImageReader;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use std::borrow::Cow;
use std::io::Cursor;
use std::sync::Arc;
use tower_cookies::Cookies;

use crate::auth;

/// レターペア管理用構造体
#[derive(sqlx::FromRow, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Pair {
    initial: String,
    next: String,
    object: String,
    image: String,
}

/// カード削除用プロパティ
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LetterPair {
    pair: String,
}

/// レターペア一覧の取得
pub async fn get_all_pair(State(pool): State<Arc<MySqlPool>>, cookies: Cookies) -> Json<Vec<Pair>> {
    let claims = auth::validate_token(cookies.get("jwt").unwrap().value()).unwrap();
    let pairs = sqlx::query_as::<_, Pair>(r#"SELECT * FROM pairs WHERE id = ?;"#)
        .bind(claims.id)
        .fetch_all(&*pool)
        .await
        .unwrap();
    Json(pairs)
}

/// レターペアの追加
pub async fn add_pair(
    State(pool): State<Arc<MySqlPool>>,
    cookies: Cookies,
    mut multipart: Multipart,
) -> Json<Pair> {
    let claims = auth::validate_token(cookies.get("jwt").unwrap().value()).unwrap();
    let mut data = Pair::default();
    while let Some(field) = multipart.next_field().await.unwrap() {
        match &*field.name().unwrap().to_string() {
            "InputPair" => {
                let pair = field.text().await.unwrap();
                let mut pair = pair.chars();
                data.initial = pair.next().unwrap().to_string();
                data.next = pair.next().unwrap().to_string();
            }
            "InputObject" => {
                data.object = field.text().await.unwrap();
            }
            "InputImage" => {
                let bytes = field.bytes().await.unwrap();
                if !bytes.is_empty() {
                    // 画像へコンバート
                    let img = ImageReader::new(Cursor::new(&bytes))
                        .with_guessed_format()
                        .unwrap()
                        .decode()
                        .unwrap();
                    // 画像をトリミング
                    let img = img.resize(256, 256, image::imageops::FilterType::Triangle);
                    // 画像をバイト列へ書き出す
                    let mut raw = Cursor::new(vec![]);
                    img.write_to(&mut raw, image::ImageFormat::Png).unwrap();
                    // GoogleCloud へアップロード
                    // TODO: ファイル名はハッシュにする
                    let filename = format!("{}{}.png", data.initial, data.next); // TODO: 厳密にはここで`InputPair`の情報を得られる保証はない
                    let config = ClientConfig {
                        storage_endpoint: "http://localhost:4443".to_string(),
                        ..Default::default()
                    }
                    .anonymous();
                    let client = Client::new(config);
                    let upload_type =
                        UploadType::Simple(Media::new(Cow::Owned(format!("test/{filename}"))));
                    // FIXME: "missing field `etag`"
                    let _uploaded = client
                        .upload_object(
                            &UploadObjectRequest {
                                bucket: "learning-letter-pairs".to_string(),
                                ..Default::default()
                            },
                            raw.into_inner(),
                            &upload_type,
                        )
                        .await;
                    data.image = format!("http://localhost:4443/download/storage/v1/b/learning-letter-pairs/o/test/{filename}?alt=media");
                }
            }
            _ => unreachable!(),
        }
    }
    sqlx::query(r#"INSERT INTO pairs (id, initial, next, object, image) VALUES (?, ?, ?, ?, ?);"#)
        .bind(claims.id)
        .bind(&data.initial)
        .bind(&data.next)
        .bind(&data.object)
        .bind(&data.image)
        .execute(&*pool)
        .await
        .unwrap();
    Json(data)
}

/// レターペアの削除
pub async fn delete_pair(
    State(pool): State<Arc<MySqlPool>>,
    cookies: Cookies,
    Json(data): Json<LetterPair>,
) -> Json<Pair> {
    let claims = auth::validate_token(cookies.get("jwt").unwrap().value()).unwrap();
    tracing::info!("{:?}", &data);
    let mut pair = data.pair.chars();
    let initial = pair.next().unwrap().to_string();
    let next = pair.next().unwrap().to_string();
    let pair = sqlx::query_as::<_, Pair>(
        r#"SELECT * FROM pairs WHERE id = ? AND initial = ? AND next = ?;"#,
    )
    .bind(claims.id)
    .bind(&initial)
    .bind(&next)
    .fetch_one(&*pool)
    .await
    .unwrap();
    sqlx::query(r#"DELETE FROM pairs WHERE id = ? AND initial = ? AND next = ?;"#)
        .bind(claims.id)
        .bind(&initial)
        .bind(&next)
        .execute(&*pool)
        .await
        .unwrap();
    Json(pair)
}

/// レターペアの修正
pub async fn update_pair(
    State(pool): State<Arc<MySqlPool>>,
    cookies: Cookies,
    mut multipart: Multipart,
) -> Json<Pair> {
    let claims = auth::validate_token(cookies.get("jwt").unwrap().value()).unwrap();
    let mut data = Pair::default();
    while let Some(field) = multipart.next_field().await.unwrap() {
        match &*field.name().unwrap().to_string() {
            "InputPair" => {
                let pair = field.text().await.unwrap();
                let mut pair = pair.chars();
                data.initial = pair.next().unwrap().to_string();
                data.next = pair.next().unwrap().to_string();
            }
            "InputObject" => {
                data.object = field.text().await.unwrap();
            }
            "InputImage" => {
                let bytes = field.bytes().await.unwrap();
                if !bytes.is_empty() {
                    // 画像へコンバート
                    let img = ImageReader::new(Cursor::new(&bytes))
                        .with_guessed_format()
                        .unwrap()
                        .decode()
                        .unwrap();
                    // 画像をトリミング
                    let img = img.resize(256, 256, image::imageops::FilterType::Triangle);
                    // 画像をバイト列へ書き出す
                    let mut raw = Cursor::new(vec![]);
                    img.write_to(&mut raw, image::ImageFormat::Png).unwrap();
                    // GoogleCloud へアップロード
                    let filename = format!("{}{}.png", data.initial, data.next); // TODO: 厳密にはここで`InputPair`の情報を得られる保証はない
                    let config = ClientConfig {
                        storage_endpoint: "http://localhost:4443".to_string(),
                        ..Default::default()
                    }
                    .anonymous();
                    let client = Client::new(config);
                    let upload_type =
                        UploadType::Simple(Media::new(Cow::Owned(format!("test/{filename}"))));
                    let _uploaded = client
                        .upload_object(
                            &UploadObjectRequest {
                                bucket: "learning-letter-pairs".to_string(),
                                ..Default::default()
                            },
                            raw.into_inner(),
                            &upload_type,
                        )
                        .await;
                    data.image = format!("http://localhost:4443/download/storage/v1/b/learning-letter-pairs/o/test/{filename}?alt=media");
                }
            }
            _ => unreachable!(),
        }
    }
    if data.image.is_empty() {
        sqlx::query(r#"UPDATE pairs SET initial = ?, next = ?, object = ? WHERE id = ? AND initial = ? AND next = ?;"#)
            .bind(&data.initial)
            .bind(&data.next)
            .bind(&data.object)
            .bind(claims.id)
            .bind(&data.initial)
            .bind(&data.next)
            .execute(&*pool)
            .await
            .unwrap();
    } else {
        sqlx::query(r#"UPDATE pairs SET initial = ?, next = ?, object = ?, image = ? WHERE id = ? AND initial = ? AND next = ?;"#)
            .bind(&data.initial)
            .bind(&data.next)
            .bind(&data.object)
            .bind(&data.image)
            .bind(claims.id)
            .bind(&data.initial)
            .bind(&data.next)
            .execute(&*pool)
            .await
            .unwrap();
    }
    Json(data)
}
