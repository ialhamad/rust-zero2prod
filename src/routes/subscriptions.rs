use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub(crate) struct FormData {
    pub(crate) email: String,
    pub(crate) name: String,
}

pub(crate) async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
