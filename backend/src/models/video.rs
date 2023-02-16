use crate::lib_db::db_connection::db_connection;
use crate::schema::video;
use crate::traits::db::{Create, ReadWrite};
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
}

#[derive(Insertable, Clone, Deserialize, Serialize)]
#[table_name = "video"]
pub struct VideoCreate {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub user_id: i32,
}

impl Create<Video> for VideoCreate {
    fn create(&self) -> Result<Video, diesel::result::Error> {
        let mut connection = db_connection();

        let video = diesel::insert_into(video::table)
            .values((
                video::title.eq(&self.title),
                video::description.eq(&self.description),
                video::url.eq(&self.url),
                video::user_id.eq(&self.user_id),
            ))
            .execute(&mut connection);

        match video {
            Ok(_) => Ok(Video {
                id: 0,
                title: self.title.clone(),
                description: self.description.clone(),
                url: self.url.clone(),
                created_at: Utc::now().naive_utc(),
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
            ))
            .load::<Video>(&mut connection);

        videos
    }
}
