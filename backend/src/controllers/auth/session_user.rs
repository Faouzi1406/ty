use actix_web::{post, web, HttpResponse, Responder};
use crate::models::sessions::SessionKeyDb;

#[post("/users/auth/get_session_info")]
pub async fn get_session_info(session_key: web::Json<String>) -> impl Responder {
    let session_key = session_key.into_inner();
    let session_info  = SessionKeyDb::get_user(session_key);

    match session_info {
        Ok(session) => {
            HttpResponse::Ok().json(session)
        }
        Err(_) => {
            HttpResponse::Unauthorized().finish()
        }
    }
}

