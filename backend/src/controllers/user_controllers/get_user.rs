use actix_web::{HttpResponse, Responder, get, web};
use crate::{models::user::User, traits::db::ReadWrite};

#[get("/user/get_user")]
pub async fn get_user(user: web::Json<User>) -> impl Responder  {
    let read_user = user.read();
    HttpResponse::Ok().json(read_user)
}

