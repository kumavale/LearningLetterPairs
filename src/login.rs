use actix_web::{web, http::header, Error, HttpRequest, HttpResponse, HttpMessage};
use actix_identity::Identity;
use askama::Template;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    if let Some(user) = user {
        return Ok(HttpResponse::Found().append_header((header::LOCATION, "/")).finish());
    }
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
    params: web::Form<LoginParams>,
    request: HttpRequest,
    user: Option<Identity>,
) -> Result<HttpResponse, Error> {
    #[derive(sqlx::FromRow)]
    pub struct Check {
        username: String,
    }

    let username = &params.username;
    let password = &params.password;

    let result = sqlx::query_as::<_, Check>("
                SELECT
                    username
                FROM
                    users
                WHERE
                    username=$1 AND password=$2
                ")
        .bind(&username)
        .bind(&password)
        .fetch_one(&**pool)
        .await;

    if result.is_ok() {
        // attach a verified user identity to the active session
        Identity::login(&request.extensions(), username.into()).unwrap();
        let url = if let Some(url) = request.headers().get(header::REFERER) {
            url.to_str().unwrap()
        } else {
            "/"
        };
        Ok(HttpResponse::Found().append_header((header::LOCATION, url)).finish())
    } else {
        login("Incorrect username or password.".to_string(), user).await
    }
}

pub async fn process_logout(
    user: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if let Some(user) = user {
        user.logout();
    }
    Ok(HttpResponse::Found().append_header((header::LOCATION, "/")).finish())
}
