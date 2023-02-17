use crate::{models::user::NewUser, traits::db::Create};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/users/create")]
pub async fn create_user(user: web::Json<NewUser>) -> impl Responder {
    match user.clone().create() {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
