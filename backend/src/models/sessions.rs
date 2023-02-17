use crate::lib_db::db_connection::db_connection;
use crate::schema::*;
use crate::traits::db::{Create, Read};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct SessionKeyDb {
    pub id: i32,
    pub session_key: String,
    pub user_id: i32,
    pub date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "sessions"]
pub struct CreateSessionKey {
    pub id: i32,
    pub sessions_key: String,
    pub user_id: i32,
    pub date: chrono::NaiveDateTime,
}

impl Create<SessionKeyDb> for SessionKeyDb {
    fn create(&self) -> Result<SessionKeyDb, diesel::result::Error> {
        let mut connection = db_connection();

        let new_session = CreateSessionKey {
            id: self.id,
            sessions_key: self.session_key.clone(),
            user_id: self.user_id,
            date: chrono::Local::now().naive_local(),
        };

        let create_session = diesel::insert_into(sessions::table)
            .values(&new_session)
            .get_result::<SessionKeyDb>(&mut connection);

        create_session
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
