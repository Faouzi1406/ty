use crate::traits::auth::auth::Auth;
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

    match user_login {
        Ok(session) => {
            println!("Session: {:?}", session);
            HttpResponse::Ok().body(session.session_key)
        }
        Err(_) => {
            HttpResponse::Unauthorized().finish()
        }
    }
}
