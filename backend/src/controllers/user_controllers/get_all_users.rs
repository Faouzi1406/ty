use actix_web::{HttpResponse, Responder, get};
use crate::{models::user::User, traits::db::ReadWrite};

#[get("/users/all")]
pub async fn get_all_users() -> impl Responder {
    let users = User::all();

    match users  {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
