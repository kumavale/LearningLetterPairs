use actix_web::{http::header, HttpResponse};

#[allow(clippy::iter_nth_zero)]
pub fn split_pair(pair: &str) -> Result<(String, String), &str> {
    if pair.chars().count() != 2 {
        return Err("invalid letter pair");
    }

    let initial = pair.chars().nth(0).unwrap().to_string();
    let next    = pair.chars().nth(1).unwrap().to_string();

    Ok((initial, next))
}

pub fn redirect(url: &str) -> HttpResponse {
    HttpResponse::SeeOther().append_header((header::LOCATION, url)).finish()
}

pub fn database_url() -> String {
    let postgres_host     = std::env::var("POSTGRES_HOST").unwrap();
    let postgres_port     = std::env::var("POSTGRES_PORT").unwrap();
    let postgres_user     = std::env::var("POSTGRES_USER").unwrap();
    let postgres_password = std::env::var("POSTGRES_PASSWORD").unwrap();
    let postgres_database = std::env::var("POSTGRES_DATABASE").unwrap();

    format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_user,
        postgres_password,
        postgres_host,
        postgres_port,
        postgres_database,
    )
}
