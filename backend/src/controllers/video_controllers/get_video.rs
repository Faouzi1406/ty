use crate::models::video::Video;
use crate::traits::get_db::GetFromDb;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/videos/select/{id}")]
pub async fn get_video(id: web::Path<i32>) -> impl Responder {
    let video = Video::get_by_id(id.into_inner());

    match video {
        Ok(video) => HttpResponse::Ok().json(video),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
