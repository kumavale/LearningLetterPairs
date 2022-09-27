use actix_web::{web, Error, HttpRequest, HttpResponse, HttpMessage};
use actix_identity::Identity;
use actix_session::Session;
use askama::Template;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::crypt;
use crate::util;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginParams {
    username: String,
    password: String,
}

pub async fn login(
    message: String,
    user: Option<Identity>,
) -> Result<HttpResponse, Error> {
    let html = LoginTemplate {
        message,
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

pub async fn process_login(
    pool: web::Data<PgPool>,
    session: Session,
    params: web::Form<LoginParams>,
    request: HttpRequest,
    user: Option<Identity>,
) -> Result<HttpResponse, Error> {
    #[derive(sqlx::FromRow)]
    pub struct Check {
        password_hash: String,
    }

    let username = &params.username;
    let password = &params.password;

    let result = sqlx::query_as::<_, Check>("
                SELECT
                    password AS password_hash
                FROM
                    users
                WHERE
                    username=$1
                ")
        .bind(&username)
        .fetch_one(&**pool)
        .await;

    // パスワードを検証
    if let Ok(check) = result {
        if crypt::verify_password(password, &check.password_hash) {
            // 認証成功
            // attach a verified user identity to the active session
            Identity::login(&request.extensions(), username.into()).unwrap();
            // 前のページのURLを取得
            let url = if let Some(url) = session.get("current_url").unwrap() { url } else { "/".to_string() };
            return Ok(util::redirect(&url));
        }
    }

    // 認証失敗
    login("Incorrect username or password.".to_string(), user).await
}

pub async fn process_logout(
    user: Option<Identity>,
    session: Session,
) -> Result<HttpResponse, Error> {
    if let Some(user) = user {
        user.logout();
    }
    session.purge();
    Ok(util::redirect("/"))
}
