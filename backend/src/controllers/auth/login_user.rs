use actix_web::{web, HttpResponse, Responder, post};
use crate::traits::auth::auth::Auth;

use crate::models::user::UserAuth;

// #[post("/users/auth/login")]
// pub async fn login_user(user:web::Json<UserAuth>) -> impl Responder {
//     let user = user.into_inner();
//     /let user_login =  user.login();
// }
