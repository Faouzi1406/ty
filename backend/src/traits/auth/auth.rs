use crate::models::{sessions::SessionKeyDb, user::User};

pub trait Auth {
    fn login(username: String, password: String) -> Result<SessionKeyDb, diesel::result::Error>;
    fn get_user_session_info(session_key: String) -> Result<User, diesel::result::Error>;
    // fn logout(username: String) -> bool;
    // fn is_logged_in(sessions_key: String) -> bool;
}
