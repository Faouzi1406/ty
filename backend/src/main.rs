pub mod controllers;
pub mod lib_db;
pub mod models;
pub mod schema;
pub mod traits;
pub mod video_uploading;

use crate::controllers::user_controllers::{
    create_user::create_user, get_all_users::get_all_users,
};
use actix_web::{web,get, App, HttpResponse, HttpServer, Responder};
use controllers::{video_controllers::{
    delete_video::delete_video,
    create_video::create_video, get_video::get_video,
}, user_controllers::get_user::get_user};
use video_uploading::video_upload_socket::video_upload_socket;


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_user)
            .service(get_all_users)
            .service(get_user)
            .service(delete_video)
            .service(create_video)
            .service(get_video)
            .route("/videos/sockets/upload", web::get().to(video_upload_socket))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("Error starting server: perhaps port");

    Ok(())
}
