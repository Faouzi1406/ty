use crate::models::{user::User, video::Video};
use crate::traits::get_db::GetFromDb;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct UserVideo {
    video: Video,
    user: User,
}

#[get("/videos/select/{id}")]
pub async fn get_video(id: web::Path<i32>) -> impl Responder {
    let video = Video::get_by_id(id.into_inner());

    match video {
        Ok(video) => {
            let user = User::get_by_id(video.user_id);
            match user {
                Ok(user) => {
                    let user_video = UserVideo { user, video };
                    HttpResponse::Ok().json(user_video)
                }
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
