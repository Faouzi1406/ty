use crate::{models::video::Video, traits::db::ReadWrite};
use actix_web::{get, HttpResponse, Responder};

#[get("/videos/all")]
pub async fn get_all_videos() -> impl Responder {
    match Video::all() {
        Ok(videos) => HttpResponse::Ok().json(videos),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
