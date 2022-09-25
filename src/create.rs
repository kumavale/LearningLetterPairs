use actix_web::{web, http::header, Error, HttpRequest, HttpResponse};
use actix_identity::Identity;
use askama::Template;
use serde::{Deserialize, Serialize};
use regex::Regex;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "create_account.html")]
struct CreateTemplate {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateParams {
    username: String,
    password: String,
}

pub async fn create(
    message: String,
    user: Option<Identity>,
) -> Result<HttpResponse, Error> {
    let html = CreateTemplate {
        message,
    };
    let view = html.render().expect("failed to render html");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(view))
}

pub async fn create_account(
    pool: web::Data<PgPool>,
    params: web::Form<CreateParams>,
    request: HttpRequest,
    user: Option<Identity>,
) -> Result<HttpResponse, Error> {
    #[derive(sqlx::FromRow)]
    pub struct Check {
        username: String,
    }

    let username = &params.username;
    let password = &params.password;

    // 同名アカウントチェック
    let result = sqlx::query_as::<_, Check>("
                SELECT
                    username
                FROM
                    users
                WHERE
                    username=$1
                ")
        .bind(&username)
        .fetch_one(&**pool)
        .await;
    if result.is_ok() {
        return create("Already exists account.".to_string(), user).await;
    }

    // 文字種/文字長チェック
    let username_re = Regex::new(r"[_0-9A-Za-z]{1,50}").unwrap();
    if !username_re.is_match(&username) {
        return create("Invalid username.".to_string(), user).await;
    }
    let password_re = Regex::new(r"[@#$%&_:;0-9A-Za-z]{8,50}").unwrap();
    if !password_re.is_match(&password) {
        return create("Invalid password.".to_string(), user).await;
    }

    // TODO: #30 ソルト＋ハッシュ

    // アカウント登録処理
    sqlx::query(r#"
        INSERT INTO
            users (username, password)
        VALUES
            ($1, $2)
        "#)
        .bind(&username)
        .bind(&password)
        .execute(&**pool)
        .await
        .unwrap();

    // ログイン画面に推移
    Ok(HttpResponse::Found().append_header((header::LOCATION, "/login")).finish())
}
