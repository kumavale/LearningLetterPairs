use std::sync::Arc;

use axum::{
    http::StatusCode,
    middleware::Next,
    response::Response,
    routing::{get, post, delete, put},
    Router,
};
use sqlx::mysql::MySqlPool;
use tower_http::cors::CorsLayer;
use http::{header::CONTENT_TYPE, HeaderValue, Method, Request};
use crate::{api, auth};

pub fn create_router(pool: MySqlPool) -> Router {
    let cors = CorsLayer::new()
        // フロントエンドからの通信を許可
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .route("/pairs", get(api::get_all_pair))
        .route("/pairs", post(api::add_pair))
        .route("/pairs", delete(api::delete_pair))
        .route("/pairs", put(api::update_pair))
        .route("/login", post(auth::login_user))
        .layer(cors)
        .layer(axum::middleware::from_fn(auth::auth))
        .layer(axum::middleware::from_fn(access_log_on_request))
        .layer(tower_cookies::CookieManagerLayer::new())
        .with_state(Arc::new(pool))
}

/// アクセスログ出力イベントハンドラ
async fn access_log_on_request<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // HTTPメソッド及びURIを出力する
    tracing::info!("[{}] {}", req.method(), req.uri());
    Ok(next.run(req).await)
}
