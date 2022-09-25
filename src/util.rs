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
    HttpResponse::Found().append_header((header::LOCATION, url)).finish()
}
