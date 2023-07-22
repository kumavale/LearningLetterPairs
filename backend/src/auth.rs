use std::sync::Arc;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use sqlx::mysql::MySqlPool;
use tower_cookies::{Cookie, Cookies};
use http::Request;
use crate::model::Claims;

/// ログインチェック
pub async fn auth<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let Some(token) = cookies.get("jwt").map(|t|t.value().to_string()) else {
        tracing::info!("not found jwt");
        return Err(StatusCode::UNAUTHORIZED);
    };
    match validate_token(&token) {
        Ok(_payload) => {
            Ok(next.run(req).await)
        }
        Err(_) => {
            tracing::info!("failed to validate token");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub async fn login_user(State(_pool): State<Arc<MySqlPool>>, cookies: Cookies, credentials: Json<Credentials>) -> impl IntoResponse {
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
