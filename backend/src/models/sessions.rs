use crate::models::user::User;
use crate::lib_db::db_connection::db_connection;
use crate::schema::*;
use crate::traits::db::Read;
use crate::traits::get_db::GetFromDb;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct SessionKeyDb {
    pub id: i32,
    pub session_key: String,
    pub user_id: i32,
    pub date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "sessions"]
pub struct CreateSessionKey {
    pub sessions_key: String,
    pub user_id: i32,
    pub date: chrono::NaiveDateTime,
}

impl  CreateSessionKey {
    pub fn create(user_id:i32) -> Result<SessionKeyDb, diesel::result::Error> {
        let mut connection = db_connection();

        let create_session = CreateSessionKey {
            sessions_key: Uuid::new_v4().to_string(),
            user_id,
            date: chrono::Local::now().naive_local(),
        }; 

        let session = diesel::insert_into(sessions::table)
            .values(&create_session)
            .get_result::<SessionKeyDb>(&mut connection);

        session
    }
}

impl Read<SessionKeyDb> for SessionKeyDb {
    fn read(&self) -> Result<SessionKeyDb, diesel::result::Error> {
        let mut connection = db_connection();

        let session = sessions::table
            .filter(sessions::id.eq(self.id))
            .first::<SessionKeyDb>(&mut connection);

        session
    }
}

impl SessionKeyDb {
    pub fn get_user(session_key:String) -> Result<User, diesel::result::Error> {
        let mut connection = db_connection();

        let session = sessions::table
            .filter(sessions::sessions_key.eq(session_key))
            .first::<SessionKeyDb>(&mut connection);

        let session = match session {
            Ok(session) => session,
            Err(_) => return Err(diesel::result::Error::NotFound),
        };
        
        let user = User::get_by_id(session.user_id);

        user 
    }

}
