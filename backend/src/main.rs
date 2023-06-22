use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::{
    extract::{Json, Multipart, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    routing::{get, post, delete},
    Router,
};
use dotenv::dotenv;
use http::{header::CONTENT_TYPE, HeaderValue, Method, Request};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use tower_http::cors::CorsLayer;

/// レターペア管理用構造体
#[derive(sqlx::FromRow, Clone, Debug, Default, Serialize, Deserialize)]
struct Pair {
    initial: String,
    next: String,
    object: String,
    image: String,
}

/// カード削除用プロパティ
#[derive(Clone, Debug, Serialize, Deserialize)]
struct LetterPair {
    pair: String,
}

/// レターペア一覧の取得
async fn get_all_pair(State(pool): State<Arc<MySqlPool>>) -> Json<Vec<Pair>> {
    let mut conn = pool.acquire().await.unwrap();
    let pairs = sqlx::query_as::<_, Pair>(r#"SELECT * FROM pairs;"#)
        .fetch_all(&mut conn)
        .await
        .unwrap();
    Json(pairs)
}

/// レターペアの追加
async fn add_pair(State(pool): State<Arc<MySqlPool>>, mut multipart: Multipart) -> Json<Pair> {
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
                // TODO: 画像をトリミング
                // TODO: URLを生成
                // TODO: S3へアップロード
                data.image = "http://127.0.0.1:9000/llp/kumavale/あい.png".to_string();
            }
            _ => unreachable!()
        }
    }
    sqlx::query(r#"INSERT INTO pairs (initial, next, object, image) VALUES (?, ?, ?, ?);"#)
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
async fn delete_pair(State(pool): State<Arc<MySqlPool>>, Json(data): Json<LetterPair>) -> Json<Pair> {
    tracing::info!("{:?}", &data);
    let mut pair = data.pair.chars();
    let initial = pair.next().unwrap().to_string();
    let next = pair.next().unwrap().to_string();
    let mut conn = pool.acquire().await.unwrap();
    let pair = sqlx::query_as::<_, Pair>(r#"SELECT * FROM pairs WHERE initial = ? AND next = ?;"#)
        .bind(&initial)
        .bind(&next)
        .fetch_one(&mut conn)
        .await
        .unwrap();
    sqlx::query(r#"DELETE FROM pairs WHERE initial = ? AND next = ?;"#)
        .bind(&initial)
        .bind(&next)
        .execute(&mut conn)
        .await
        .unwrap();
    Json(pair)
}

/// アクセスログ出力イベントハンドラ
async fn access_log_on_request<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // HTTPメソッド及びURIを出力する
    tracing::info!("[{}] {}", req.method(), req.uri());
    Ok(next.run(req).await)
}

#[tokio::main]
async fn main() {
    // .envファイルの読み込み
    dotenv().ok();
    // ログ出力情報の初期化
    tracing_subscriber::fmt().init();

    // データベースへ接続
    let mysql_user     = env::var("MYSQL_USER").unwrap();
    let mysql_password = env::var("MYSQL_PASSWORD").unwrap();
    let mysql_database = env::var("MYSQL_DATABASE").unwrap();
    let database_url = format!("mysql://{mysql_user}:{mysql_password}@localhost:3306/{mysql_database}");
    let pool = MySqlPool::connect(&database_url).await.unwrap();
    // テーブル作成、サンプルレコードの登録
    sqlx::migrate!().run(&pool).await.unwrap();

    // ルーティング設定
    let app = Router::new()
        .route("/pairs", get(get_all_pair))
        .route("/pairs", post(add_pair))
        .route("/pairs", delete(delete_pair))
        .layer(
            CorsLayer::new()
                // フロントエンドからの通信を許可
                .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::DELETE])
                .allow_headers([CONTENT_TYPE]),
        )
        .layer(axum::middleware::from_fn(access_log_on_request))
        .with_state(Arc::new(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
