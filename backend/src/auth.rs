use axum::{
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

/// ログインチェック
pub async fn auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let Some(token) = cookies.get("jwt").map(|t|t.value().to_string()) else {
        tracing::info!("not found jwt");
        return Err(StatusCode::UNAUTHORIZED);
    };
    match validate_token(&token) {
        Ok(_payload) => Ok(next.run(req).await),
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
    id: u64, // TODO: これは要らないかなぁ
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
        let claims = Claims {
            id: user.id,
            name: user.username.to_string(),
            exp: 10000000000, // TODO: 有効期限設定
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()), // JWTのシークレットキー
        )
        .unwrap();

        tracing::warn!("login jwt: {}", token);
        let jwt = Cookie::build("jwt", token)
            .path("/")
            .same_site(tower_cookies::cookie::SameSite::None)
            //.secure(true)
            //.http_only(true)
            .finish();
        cookies.add(jwt);
        tracing::info!("Logged in successfully ({})", user.username);
        Json(LoginResponse {
            status: LoginStatus::Success,
            id: user.id,
            username: user.username,
        })
    } else {
        tracing::warn!("Logged in failed");
        Json(LoginResponse {
            status: LoginStatus::Failed,
            id: 0,
            username: "".to_string(),
        })
    }
}

pub async fn register(
    State(pool): State<Arc<MySqlPool>>,
    cookies: Cookies,
    credentials: Json<Credentials>,
) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();

    // ユーザー有無を確認
    if sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = ?"#)
        .bind(&credentials.username)
        .fetch_one(&mut conn)
        .await
        .is_ok()
    {
        // 該当ユーザーが既に存在します
        tracing::warn!("this username is alreadly exist: {}", &credentials.username);
        return Json(LoginResponse {
            status: LoginStatus::Failed,
            id: 0,
            username: "".to_string(),
        });
    };

    // パスワードハッシュを生成
    let password_hash = crypt::compute_password_hash(&credentials.password).unwrap();

    // ユーザーをDBに登録する
    if sqlx::query(r#"INSERT INTO users (username, password_hash) VALUES (?, ?)"#)
        .bind(&credentials.username)
        .bind(&password_hash)
        .execute(&mut conn)
        .await
        .is_err() {
        // ユーザー登録に失敗
        tracing::warn!("failed to register user: {}", &credentials.username);
        return Json(LoginResponse {
            status: LoginStatus::Failed,
            id: 0,
            username: "".to_string(),
        });
    }

    // ユーザーを取得
    let user = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = ?"#)
        .bind(&credentials.username)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    // JWTの発行とCookieへのセット
    let claims = Claims {
        id: user.id,
        name: user.username.to_string(),
        exp: 10000000000, // TODO: 有効期限設定
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()), // JWTのシークレットキー
    )
    .unwrap();

    tracing::warn!("register jwt: {}", token);
    let jwt = Cookie::build("jwt", token)
        .path("/")
        .same_site(tower_cookies::cookie::SameSite::None)
        //.secure(true)
        //.http_only(true)
        .finish();
    cookies.add(jwt);
    tracing::info!("Logged in successfully ({})", user.username);
    Json(LoginResponse {
        status: LoginStatus::Success,
        id: user.id,
        username: user.username,
    })
}

// パスワードの検証処理
async fn validate_password(pool: Arc<MySqlPool>, username: &str, password: &str) -> Option<User> {
    let mut conn = pool.acquire().await.unwrap();
    // ユーザーを取得
    let Ok(user) = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = ?"#)
        .bind(username)
        .fetch_one(&mut conn)
        .await else {
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
pub fn validate_token(token: &str) -> Result<Claims, ()> {
    let decoding_key = jsonwebtoken::DecodingKey::from_secret("secret".as_ref());
    let validation = jsonwebtoken::Validation::default();
    match jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => {
            tracing::warn!("validate JWT error: {:?}", e);
            Err(())
        }
    }
}
