use axum::{
    body::Body,
    extract::{Json, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::Request;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use std::sync::Arc;
use tower_cookies::{Cookie, Cookies};

use crate::{crypt, model::Claims};

/// JWT の有効期限
pub const EXP_DAYS: i64 = 1;

/// ログインチェック
pub async fn auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let Some(token) = cookies.get("jwt").map(|t| t.value().to_string()) else {
        tracing::info!("not found jwt");
        return Err(StatusCode::UNAUTHORIZED);
    };
    match validate_token(&token) {
        Ok(_payload) => Ok(next.run(req).await),
        Err(e) => {
            tracing::info!("failed to validate token: {e}");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(sqlx::FromRow, Clone, Debug, Default, Serialize, Deserialize)]
pub struct User {
    id: u64,
    username: String,
    password_hash: String,
}

#[derive(Debug, Serialize)]
pub enum LoginStatus {
    Success,
    Failed,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    status: LoginStatus,
    username: String,
}

pub async fn login_user(
    State(pool): State<Arc<MySqlPool>>,
    cookies: Cookies,
    credentials: Json<Credentials>,
) -> impl IntoResponse {
    let is_valid_user =
        validate_password(pool.clone(), &credentials.username, &credentials.password).await;

    if let Some(user) = is_valid_user {
        // JWTの発行とCookieへのセット
        let claims = Claims::new(user.id, user.username.to_string());
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()), // JWTのシークレットキー
        )
        .unwrap();

        tracing::info!("login jwt: {}", token);
        let jwt = Cookie::build(("jwt", token))
            .path("/")
            .same_site(tower_cookies::cookie::SameSite::None)
            //.secure(true)
            //.http_only(true)
            .build();
        cookies.add(jwt);
        tracing::info!("Logged in successfully ({})", user.username);
        Json(LoginResponse {
            status: LoginStatus::Success,
            username: user.username,
        })
    } else {
        tracing::warn!("Logged in failed");
        Json(LoginResponse {
            status: LoginStatus::Failed,
            username: "".to_string(),
        })
    }
}

pub async fn register(
    State(pool): State<Arc<MySqlPool>>,
    cookies: Cookies,
    credentials: Json<Credentials>,
) -> impl IntoResponse {
    // ユーザー有無を確認
    if sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = ?"#)
        .bind(&credentials.username)
        .fetch_one(&*pool)
        .await
        .is_ok()
    {
        // 該当ユーザーが既に存在します
        tracing::warn!("this username is alreadly exist: {}", &credentials.username);
        return Json(LoginResponse {
            status: LoginStatus::Failed,
            username: "".to_string(),
        });
    };

    // パスワードハッシュを生成
    let password_hash = crypt::compute_password_hash(&credentials.password).unwrap();

    // ユーザーをDBに登録する
    if sqlx::query(r#"INSERT INTO users (username, password_hash) VALUES (?, ?)"#)
        .bind(&credentials.username)
        .bind(&password_hash)
        .execute(&*pool)
        .await
        .is_err()
    {
        // ユーザー登録に失敗
        tracing::warn!("failed to register user: {}", &credentials.username);
        return Json(LoginResponse {
            status: LoginStatus::Failed,
            username: "".to_string(),
        });
    }

    // ユーザーを取得
    let user = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = ?"#)
        .bind(&credentials.username)
        .fetch_one(&*pool)
        .await
        .unwrap();

    // JWTの発行とCookieへのセット
    let claims = Claims::new(user.id, user.username.to_string());
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()), // JWTのシークレットキー
    )
    .unwrap();

    tracing::warn!("register jwt: {}", token);
    let jwt = Cookie::build(("jwt", token))
        .path("/")
        .same_site(tower_cookies::cookie::SameSite::None)
        //.secure(true)
        //.http_only(true)
        .build();
    cookies.add(jwt);
    tracing::info!("Logged in successfully ({})", user.username);
    Json(LoginResponse {
        status: LoginStatus::Success,
        username: user.username,
    })
}

// パスワードの検証処理
async fn validate_password(pool: Arc<MySqlPool>, username: &str, password: &str) -> Option<User> {
    // ユーザーを取得
    let Ok(user) = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = ?"#)
        .bind(username)
        .fetch_one(&*pool)
        .await
    else {
        // 該当ユーザーが存在しません
        tracing::warn!("user does not exist: {}", username);
        return None;
    };

    // パスワードの検証
    #[allow(clippy::redundant_pattern_matching)]
    if let Err(_) = crypt::verify_password(password, &user.password_hash) {
        //     ^ ここは平文のパスワードが返ってくるので、ログに出力してはいけない
        // 無効なパスワード
        tracing::warn!("validate password error: username: {}", user.username);
        return None;
    }

    Some(user)
}

// JWTの検証処理
pub fn validate_token(token: &str) -> Result<Claims, String> {
    let decoding_key = jsonwebtoken::DecodingKey::from_secret("secret".as_ref());
    let validation = jsonwebtoken::Validation::default();
    match jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(format!("validate JWT error: {:?}", e)),
    }
}
