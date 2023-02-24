use crate::lib_db::db_connection::db_connection;
use crate::models::user::{self, User};
use crate::schema::{users, video};
use crate::traits::db::{Create, ReadWrite, Relational};
use crate::traits::get_db::GetFromDb;
use chrono::{NaiveDateTime, Utc};
use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Video {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub user_id: i32,
    pub thumbmail: String,
}

#[derive(Insertable, Clone, Deserialize, Serialize, Debug)]
#[table_name = "video"]
pub struct VideoCreate {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub user_id: i32,
    pub thumb_mail_url: Option<String>,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct VideoUser(User, Video);

impl Create<Video> for VideoCreate {
    fn create(&self) -> Result<Video, diesel::result::Error> {
        let mut connection = db_connection();

        let video = diesel::insert_into(video::table)
            .values((
                video::title.eq(&self.title),
                video::description.eq(&self.description),
                video::url.eq(&self.url),
                video::user_id.eq(&self.user_id),
                video::thumb_mail_url.eq(&self.thumb_mail_url.clone().unwrap_or("default.jpeg".to_string()))
            ))
            .execute(&mut connection);

        match video {
            Ok(_) => Ok(Video {
                id: 0,
                title: self.title.clone(),
                description: self.description.clone(),
                url: self.url.clone(),
                created_at: Utc::now().naive_utc(),
                thumbmail: "default.jpeg".to_string(),
                user_id: self.user_id,
            }),
            Err(e) => Err(e),
        }
    }
}

impl ReadWrite for Video {
    fn read(&self) -> Self {
        let mut connection = db_connection();

        let video = video::table
            .select((
                video::id,
                video::title,
                video::description,
                video::url,
                video::created_at,
                video::user_id,
                video::thumb_mail_url,
            ))
            .filter(video::id.eq(&self.id))
            .first::<Video>(&mut connection)
            .expect("Error reading video");

        video
    }

    fn update(&self) -> Result<(), diesel::result::Error> {
        let mut connection = db_connection();

        let video = diesel::update(video::table.filter(video::id.eq(&self.id))).set((
            video::title.eq(&self.title),
            video::description.eq(&self.description),
            video::url.eq(&self.url),
        ));

        match video.execute(&mut connection) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn delete(&self) -> Result<(), diesel::result::Error> {
        let mut connection = db_connection();

        let video = diesel::delete(video::table.filter(video::id.eq(&self.id)));

        match video.execute(&mut connection) {
            Ok(_) => Ok(()),
            Err(_) => Err(diesel::result::Error::NotFound),
        }
    }

    fn all() -> Result<Vec<Self>, diesel::result::Error> {
        let mut connection = db_connection();

        let videos = video::table
            .select((
                video::id,
                video::title,
                video::description,
                video::url,
                video::created_at,
                video::user_id,
                video::thumb_mail_url,
            ))
            .load::<Video>(&mut connection);

        videos
    }
}

impl GetFromDb for Video {
    fn get_by_id(id: i32) -> Result<Self, diesel::result::Error> {
        let mut connection = db_connection();
        let get_video = video::table
            .select((
                video::id,
                video::title,
                video::description,
                video::url,
                video::created_at,
                video::user_id,
                video::thumb_mail_url,
            ))
            .filter(video::id.eq(id))
            .first::<Video>(&mut connection);

        get_video
    }
}

impl Relational<VideoUser> for Video {
    fn get_with_relation() -> Result<Vec<VideoUser>, diesel::result::Error> {
        let mut connection = db_connection();

        let videos_with_users = video::table
            .inner_join(users::table)
            .select((
                (users::id, users::username, users::email, users::profile_pic),
                (
                    video::id,
                    video::title,
                    video::description,
                    video::url,
                    video::created_at,
                    video::user_id,
                    video::thumb_mail_url,
                ),
            ))
            .load::<VideoUser>(&mut connection);

        videos_with_users
    }
}
