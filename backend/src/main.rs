pub mod controllers;
pub mod lib_db;
pub mod models;
pub mod schema;
pub mod traits;

use crate::controllers::user_controllers::{
    create_user::create_user, get_all_users::get_all_users,
};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This api go crazy brrrrr!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_user)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("Error starting server: perhaps port");

    Ok(())
}
