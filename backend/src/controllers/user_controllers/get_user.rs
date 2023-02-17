use crate::{models::user::User, traits::get_db::GetFromDb};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/user/select/{id}")]
pub async fn get_user(user: web::Path<i32>) -> impl Responder {
    let read_user = User::get_by_id(user.into_inner());

    match read_user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
