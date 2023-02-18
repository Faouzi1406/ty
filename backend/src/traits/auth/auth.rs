use crate::models::sessions::SessionKeyDb;

pub trait Auth {
    fn login(username: String, password: String) -> String;
    fn logout(username: String) -> bool;
    fn is_logged_in(sessions_key: String) -> bool;
}
