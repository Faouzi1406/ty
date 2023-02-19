use crate::traits::auth::auth::Auth;
use actix_session::{Session, SessionExt};
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::models::user::UserAuth;

#[derive(Deserialize, Serialize)]
pub struct UserAuthLogin {
    username: String,
    password: String,
}

#[post("/users/auth/login")]
pub async fn login_user(user: web::Json<UserAuthLogin>) -> impl Responder {
    let user = user.into_inner();
    let user_login = UserAuth::login(user.username, user.password);

    println!("user_login: {:?}", user_login);

    match user_login {
        Ok(session) => {
            println!("Session: {:?}", session);
            HttpResponse::Ok().body(session.session_key)
        }
        Err(_) => {
            println!("Error logging in user.");
            HttpResponse::Unauthorized().finish()
        }
    }
}
