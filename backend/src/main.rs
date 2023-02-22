pub mod controllers;
pub mod lib_db;
pub mod models;
pub mod schema;
pub mod traits;
pub mod video_uploading;

use crate::controllers::user_controllers::{
    create_user::create_user, get_all_users::get_all_users,
};
use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use controllers::auth::{login_user, session_user};
use controllers::video_controllers::serve_video::serve_video;
use controllers::{
    user_controllers::get_user::get_user,
    video_controllers::{
        create_video::create_video, delete_video::delete_video, get_video::get_video,
        get_all_videos::get_all_videos,
    },
};
use video_uploading::video_upload_socket::video_upload_socket;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(index)
            .service(create_user)
            .service(get_all_users)
            .service(get_user)
            .service(delete_video)
            .service(create_video)
            .service(get_video)
            .service(login_user::login_user)
            .service(session_user::get_session_info)
            .service(get_all_videos)
            .service(serve_video)
            .route("/videos/sockets/upload", web::get().to(video_upload_socket))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("Error starting server: perhaps port");

    Ok(())
}
