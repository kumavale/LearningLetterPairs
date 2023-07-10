use axum::{
    extract::Json,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::Deserialize;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::cors::CorsLayer;
use http::{header::CONTENT_TYPE, HeaderValue, Method, Request};
use crate::model::Claims;

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login_user))
        .layer(axum::middleware::from_fn(access_log_on_request))
        .layer(axum::middleware::from_fn(check_login))
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

/// ログインチェック
async fn check_login<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let Some(token) = cookies.get("jwt").map(|t|t.value().to_string()) else {
        return Err(StatusCode::TEMPORARY_REDIRECT);
    };
    match validate_token(&token) {
        Ok(_payload) => {
            Ok(next.run(req).await)
        }
        Err(_) => {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

#[derive(Debug, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

async fn login_user(cookies: Cookies, credentials: Json<Credentials>) -> impl IntoResponse {
    // TODO: パスワードの検証処理を実装する
    let is_valid_user = validate_password(&credentials.username, &credentials.password);

    if is_valid_user {
        // JWTの発行とCookieへのセット
        let claims = Claims {
            name: credentials.username.to_string(),
            exp: 10000000000,
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
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

// JWTの検証処理
fn validate_token(token: &str) -> Result<Claims, ()> {
    let decoding_key = jsonwebtoken::DecodingKey::from_secret("secret".as_ref());
    let validation = jsonwebtoken::Validation::default();
    match jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => {
            tracing::error!("validate error: {:?}", e);
            Err(())
        }
    }
}
