use axum::{
    extract::Json,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::cors::CorsLayer;
use http::{header::CONTENT_TYPE, HeaderValue, Method, Request};

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login_user))
        .layer(axum::middleware::from_fn(access_log_on_request))
        .layer(CookieManagerLayer::new())
        .layer(
            CorsLayer::new()
                // フロントエンドからの通信を許可
                .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST])
                .allow_headers([CONTENT_TYPE])
                .allow_credentials(true)
        )
}

/// アクセスログ出力イベントハンドラ
async fn access_log_on_request<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // HTTPメソッド及びURIを出力する
    tracing::info!("[{}] {}", req.method(), req.uri());
    Ok(next.run(req).await)
}

#[derive(Debug, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct JwtPayload {
    // TODO: 必要なユーザー情報を追加する
}

async fn login_user(cookies: Cookies, credentials: Json<Credentials>) -> impl IntoResponse {
    // TODO: パスワードの検証処理を実装する
    let is_valid_user = validate_password(&credentials.username, &credentials.password);

    if is_valid_user {
        // JWTの発行とCookieへのセット
        let jwt_payload = JwtPayload {
            // TODO: 必要なユーザー情報をセットする
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &jwt_payload,
            &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()), // JWTのシークレットキー
        )
        .unwrap();

        let mut jwt = Cookie::new("jwt", token);
        jwt.set_path("/");
        jwt.set_same_site(Some(tower_cookies::cookie::SameSite::None));
        //jwt.set_secure(Some(true));
        //jwt.set_http_only(Some(true));
        cookies.add(jwt);
        //Json(token)
        "Logged in successfully".to_string().into_response()
    } else {
        "Invalid credentials".to_string().into_response()
        //Json("".to_string())
    }
}

// パスワードの検証処理
fn validate_password(_id: &str, _password: &str) -> bool {
    // TODO: ユーザーのIDとパスワードを検証する処理を実装する
    // 検証が成功した場合はtrueを返し、そうでなければfalseを返す
    true
}
