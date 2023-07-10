use std::io::Cursor;
use std::sync::Arc;
use axum::extract::{Json, Multipart, State};
use image::io::Reader as ImageReader;
use s3::{
    Bucket,
    region::Region,
    creds::Credentials,
};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;

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
pub async fn get_all_pair(State(pool): State<Arc<MySqlPool>>) -> Json<Vec<Pair>> {
    let mut conn = pool.acquire().await.unwrap();
    let pairs = sqlx::query_as::<_, Pair>(r#"SELECT * FROM pairs WHERE id = ?;"#)
        .bind(0)
        .fetch_all(&mut conn)
        .await
        .unwrap();
    Json(pairs)
}

/// レターペアの追加
pub async fn add_pair(State(pool): State<Arc<MySqlPool>>, mut multipart: Multipart) -> Json<Pair> {
    let mut conn = pool.acquire().await.unwrap();
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
                    // S3へアップロード
                    let filename = format!("{}{}.png", data.initial, data.next);  // TODO: 厳密にはここで`InputPair`の情報を得られる保証はない
                    let bucket = Bucket::new(
                        "llp",
                        Region::Custom {
                            region: "us-west-rack-2".to_owned(),
                            endpoint: "http://localhost:9000".to_owned(),
                        },
                        Credentials::new(
                            Some("8ZzW3h29GHlem39vsRM6"),
                            Some("2qrNQY5x5ODkaJPey4DmkCDsfWuyucHhT9VJw3iC"),
                            None,
                            None,
                            None,
                        ).unwrap(),
                    ).unwrap();
                    bucket.put_object(&format!("llp/kumavale/{filename}"), &raw.into_inner()).await.unwrap();
                    data.image = format!("http://localhost:9000/llp/kumavale/{filename}");
                }
            }
            _ => unreachable!()
        }
    }
    sqlx::query(r#"INSERT INTO pairs (id, initial, next, object, image) VALUES (?, ?, ?, ?, ?);"#)
        .bind(0)
        .bind(&data.initial)
        .bind(&data.next)
        .bind(&data.object)
        .bind(&data.image)
        .execute(&mut conn)
        .await
        .unwrap();
    Json(data)
}

/// レターペアの削除
pub async fn delete_pair(State(pool): State<Arc<MySqlPool>>, Json(data): Json<LetterPair>) -> Json<Pair> {
    tracing::info!("{:?}", &data);
    let mut pair = data.pair.chars();
    let initial = pair.next().unwrap().to_string();
    let next = pair.next().unwrap().to_string();
    let mut conn = pool.acquire().await.unwrap();
    let pair = sqlx::query_as::<_, Pair>(r#"SELECT * FROM pairs WHERE id = ? AND initial = ? AND next = ?;"#)
        .bind(0)
        .bind(&initial)
        .bind(&next)
        .fetch_one(&mut conn)
        .await
        .unwrap();
    sqlx::query(r#"DELETE FROM pairs WHERE id = ? AND initial = ? AND next = ?;"#)
        .bind(0)
        .bind(&initial)
        .bind(&next)
        .execute(&mut conn)
        .await
        .unwrap();
    Json(pair)
}

/// レターペアの修正
pub async fn update_pair(State(pool): State<Arc<MySqlPool>>, mut multipart: Multipart) -> Json<Pair> {
    let mut conn = pool.acquire().await.unwrap();
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
                    // S3へアップロード
                    let filename = format!("{}{}.png", data.initial, data.next);  // TODO: 厳密にはここで`InputPair`の情報を得られる保証はない
                    let bucket = Bucket::new(
                        "llp",
                        Region::Custom {
                            region: "us-west-rack-2".to_owned(),
                            endpoint: "http://localhost:9000".to_owned(),
                        },
                        Credentials::new(
                            Some("8ZzW3h29GHlem39vsRM6"),
                            Some("2qrNQY5x5ODkaJPey4DmkCDsfWuyucHhT9VJw3iC"),
                            None,
                            None,
                            None,
                        ).unwrap(),
                    ).unwrap();
                    bucket.put_object(&format!("llp/kumavale/{filename}"), &raw.into_inner()).await.unwrap();
                    data.image = format!("http://localhost:9000/llp/kumavale/{filename}");
                }
            }
            _ => unreachable!()
        }
    }
    if data.image.is_empty() {
        sqlx::query(r#"UPDATE pairs SET initial = ?, next = ?, object = ? WHERE id = ? AND initial = ? AND next = ?;"#)
            .bind(&data.initial)
            .bind(&data.next)
            .bind(&data.object)
            .bind(0)
            .bind(&data.initial)
            .bind(&data.next)
            .execute(&mut conn)
            .await
            .unwrap();
    } else {
        sqlx::query(r#"UPDATE pairs SET initial = ?, next = ?, object = ?, image = ? WHERE id = ? AND initial = ? AND next = ?;"#)
            .bind(&data.initial)
            .bind(&data.next)
            .bind(&data.object)
            .bind(&data.image)
            .bind(0)
            .bind(&data.initial)
            .bind(&data.next)
            .execute(&mut conn)
            .await
            .unwrap();
    }
    Json(data)
}
