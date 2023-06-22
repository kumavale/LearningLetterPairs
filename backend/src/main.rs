use std::net::SocketAddr;
use axum::{
    extract::{Json, Multipart},
    http::StatusCode,
    middleware::Next,
    response::Response,
    routing::{get, post},
    Router,
};
use http::{header::CONTENT_TYPE, HeaderValue, Method, Request};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

/// レターペア管理用構造体
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Pair {
    initial: String,
    next: String,
    object: String,
    image: String,
}

/// レターペア一覧の取得
async fn get_all_pair() -> Json<Vec<Pair>> {
    let pairs = vec![
        Pair {
            initial: "あ".to_string(),
            next: "い".to_string(),
            object: "アイス".to_string(),
            image: "http://127.0.0.1:9000/llp/kumavale/あい.png".to_string(),
        }
    ];
    Json(pairs)
}

/// レターペアの追加
async fn add_pair(mut multipart: Multipart) -> Json<Pair> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        tracing::info!("Length of `{}` is {} bytes", name, data.len());
    }
    let pairs = Pair {
        initial: "あ".to_string(),
        next: "い".to_string(),
        object: "アイス".to_string(),
        image: "http://127.0.0.1:9000/llp/kumavale/あい.png".to_string(),
    };
    Json(pairs)
}

/// アクセスログ出力イベントハンドラ
async fn access_log_on_request<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // HTTPメソッド及びURIを出力する
    tracing::info!("[{}] {}", req.method(), req.uri());
    Ok(next.run(req).await)
}

#[tokio::main]
async fn main() {
    // ログ出力情報の初期化
    tracing_subscriber::fmt().init();
    // ルーティング設定
    let app = Router::new()
        .route("/pairs", get(get_all_pair))
        .route("/pairs", post(add_pair))
        .layer(
            CorsLayer::new()
                // フロントエンドからの通信を許可
                .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([CONTENT_TYPE]),
        )
        .layer(axum::middleware::from_fn(access_log_on_request));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
