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

pub async fn login(mut message: String, user: Option<Identity>) -> Result<HttpResponse, Error> {
    if let Some(user) = user {
        message = format!("Login with {}", user.id().unwrap());
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
        // TODO: 前のURLに飛ぶ
        Ok(HttpResponse::Found().append_header((header::LOCATION, "/")).finish())
        //login("".to_string()).await
    } else {
        //login("Incorrect username or password.".to_string()).await
        Ok(HttpResponse::Found().append_header((header::LOCATION, "/login")).finish())
    }
}

pub async fn process_logout(
    user: Identity
) -> Result<HttpResponse, Error> {
    user.logout();
    Ok(HttpResponse::Found().append_header((header::LOCATION, "/")).finish())
}
