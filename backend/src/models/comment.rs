use crate::lib_db::db_connection::db_connection;
use crate::schema::comments;
use crate::traits::db::{Create, ReadWrite};
use chrono::{NaiveDateTime, Utc};
use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    pub id: i32,
    pub comment: String,
    pub user_id: i32,
    pub video_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Clone, Deserialize, Serialize)]
#[table_name = "comments"]
pub struct CommentCreate {
    pub comment: String,
    pub user_id: i32,
    pub video_id: i32,
}

impl Create<Comment> for CommentCreate {
    fn create(&self) -> Result<Comment, diesel::result::Error> {
        let mut connection = db_connection();

        let comment = diesel::insert_into(comments::table)
            .values((
                comments::comment.eq(&self.comment),
                comments::user_id.eq(&self.user_id),
                comments::video_id.eq(&self.video_id),
            ))
            .execute(&mut connection);

        match comment {
            Ok(_) => Ok(Comment {
                id: 0,
                comment: self.comment.clone(),
                user_id: self.user_id,
                video_id: self.video_id,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            }),
            Err(e) => Err(e),
        }
    }
}

impl ReadWrite for Comment {
    fn read(&self) -> Self {
        let mut connection = db_connection();

        let comment = comments::table
            .select((
                comments::id,
                comments::comment,
                comments::user_id,
                comments::video_id,
                comments::created_at,
                comments::updated_at,
            ))
            .filter(comments::id.eq(&self.id))
            .first::<Comment>(&mut connection)
            .unwrap();

        comment
    }

    fn update(&self) -> Result<(), diesel::result::Error> {
        let mut connection = db_connection();

        let comment = diesel::update(comments::table)
            .set((
                comments::comment.eq(&self.comment),
                comments::user_id.eq(&self.user_id),
                comments::video_id.eq(&self.video_id),
                comments::updated_at.eq(Utc::now().naive_utc()),
            ))
            .filter(comments::id.eq(&self.id))
            .execute(&mut connection);

        match comment {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn  delete(&self) -> Result<(), diesel::result::Error> {
        let mut connection = db_connection();

        let comment = diesel::delete(comments::table)
            .filter(comments::id.eq(&self.id))
            .execute(&mut connection);

        match comment {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn all() -> Result<Vec<Self>, diesel::result::Error> {
        let mut connection = db_connection();

        let comments = comments::table
            .select((
                comments::id,
                comments::comment,
                comments::user_id,
                comments::video_id,
                comments::created_at,
                comments::updated_at,
            ))
            .load::<Comment>(&mut connection);
            

        comments
    }
}
