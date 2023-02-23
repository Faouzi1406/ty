use crate::{models::video::VideoCreate, traits::db::Create};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/videos/create")]
pub async fn create_video(video_receive: web::Json<VideoCreate>) -> impl Responder {
    let video = video_receive.create();
    
    if video_receive.thumb_mail_url.is_some() {
        
    }

    match video {
        Ok(video) => HttpResponse::Ok().json(video),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
