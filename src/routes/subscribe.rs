use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_formdata: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
