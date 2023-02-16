use crate::{models::video::VideoCreate, traits::db::Create};
use actix_web::{post, web, HttpResponse, Responder};

// Todo: Add sockets and validation:
#[post("/videos/create")]
pub async fn create_video(video: web::Json<VideoCreate>) -> impl Responder {
    let video = video.create();

    match video {
        Ok(video) => HttpResponse::Ok().json(video),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
