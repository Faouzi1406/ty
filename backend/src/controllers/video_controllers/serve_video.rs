use crate::models::video::Video;
use crate::traits::get_db::GetFromDb;
use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, web, Error};
use std::path::PathBuf;

#[get("/videos/select/watch/{id}")]
pub async fn serve_video(video_id: web::Path<i32>) -> Result<fs::NamedFile, Error> {
    let video = Video::get_by_id(video_id.into_inner());
    match video {
        Ok(video) => {
            let mut path = PathBuf::from("./videos/");
            path.push(video.url);
            let file = fs::NamedFile::open(path)?;

            Ok(file
                .use_last_modified(true)
                .set_content_disposition(ContentDisposition {
                    disposition: DispositionType::Attachment,
                    parameters: vec![],
                }))
        }
        Err(value) => panic!("Couldn't find video ${value}"),
    }
}

#[get("/videos/select/thumbmail/{id}")]
pub async fn serve_thumbmail(video_id: web::Path<i32>) -> Result<fs::NamedFile, Error> {
    let video = Video::get_by_id(video_id.into_inner());

    match video {
        Ok(video) => {
            let mut path = String::from("./thumbmails/");
            path.push_str(video.thumbmail.replace("./thumbmails/", "").as_str());
            let file = fs::NamedFile::open(path)?;

            Ok(file
                .use_last_modified(true)
                .set_content_disposition(ContentDisposition {
                    disposition: DispositionType::Attachment,
                    parameters: vec![],
                }))
        }
        Err(value) => panic!("Couldn't find video ${value}"),
    }
}
