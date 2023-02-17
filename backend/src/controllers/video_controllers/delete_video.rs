use crate::{models::video::Video, traits::db::ReadWrite};
use actix_web::{delete, web, HttpResponse, Responder};

#[delete("/videos/create")]
pub async fn delete_video(video: web::Json<Video>) -> impl Responder {
    let video = video.delete();

    match video {
        Ok(video) => HttpResponse::Ok().json(video),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
