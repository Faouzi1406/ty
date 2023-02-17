extern crate bcrypt;
use crate::schema::*;
use crate::traits::db::Create;
use crate::traits::get_db::GetFromDb;
use crate::{lib_db::db_connection::db_connection, traits::db::ReadWrite};
//use actix_session::storage::SessionKey as Session;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::traits::auth::auth::Auth; 
use crate::models::sessions::SessionKeyDb;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct UserAuth {
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> Self {
        NewUser {
            username,
            email,
            password,
        }
    }
}

impl ReadWrite for User {
    /// Reads a single user.
    /// with the username as the parameter.
    fn read(&self) -> User {
        let mut connection = db_connection();

        let user = users::table
            .select((users::id, users::username, users::email))
            .filter(users::username.eq(&self.username))
            .first::<User>(&mut connection)
            .expect("Error reading user");

        user
    }

    /// Updates a single user.
    fn update(&self) -> Result<(), diesel::result::Error> {
        let mut connection = db_connection();

        let user = diesel::update(users::table.filter(users::username.eq(&self.username))).set((
            users::username.eq(&self.username),
            users::email.eq(&self.email),
        ));

        match user.execute(&mut connection) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn delete(&self) -> Result<(), diesel::result::Error> {
        let mut connection = db_connection();

        let delete = diesel::delete(users::table.filter(users::username.eq(&self.username)))
            .execute(&mut connection);

        match delete {
            Ok(_) => Ok(()),
            _ => Err(diesel::result::Error::NotFound),
        }
    }

    /// Returns all users in the database
    fn all() -> Result<Vec<User>, diesel::result::Error> {
        let mut connection = db_connection();

        let users = users::table
            .select((users::id, users::username, users::email))
            .load::<User>(&mut connection);

        users
    }
}

/// Implements the Db trait for the NewUser struct.
/// This allows us to use the same methods for the User struct.
impl Create<User> for NewUser {
    /// Creates a new user.
    fn create(&self) -> Result<User, diesel::result::Error> {
        let mut connection = db_connection();

        let user = diesel::insert_into(users::table)
            .values(self)
            .returning((users::id, users::username, users::email))
            .get_result::<User>(&mut connection);

        let hash_pass = Arc::new(self.clone());
        if user.is_ok() {
            std::thread::spawn(move || {
                let hash_pass = hash_pass.clone();
                let hashed = hash(&hash_pass.password, DEFAULT_COST).expect("Error hashing password.");

                let mut connection = db_connection();
                let _ =
                    diesel::update(users::table.filter(users::username.eq(&hash_pass.username)))
                        .set((users::password.eq(hashed),))
                        .execute(&mut connection)
                        .expect("Error updating password");
            });
        }

        user
    }
}

impl GetFromDb for User {
    fn get_by_id(id: i32) -> Result<Self, diesel::result::Error> {
        let mut connection = db_connection();

        let user = users::table
            .select((users::id, users::username, users::email))
            .filter(users::id.eq(id))
            .first::<User>(&mut connection);

        user
    }
}

impl Auth for User {
    fn login(username: String, password: String) -> SessionKeyDb {
        let mut connection = db_connection();

        let user = users::table
            .select((users::username, users::password))
            .filter(users::username.eq(&username))
            .first::<UserAuth>(&mut connection)
            .expect("Error reading user");

        let verify = bcrypt::verify(&password, &user.password);

        // if verify.is_ok() {
        //     let session_key = Sessionkew;
        //     return session_key;
        // }
        
        todo!();
    }

    fn logout(username: String) -> bool {
       todo!() 
    }

    fn is_logged_in(sessions_key: String) -> bool {
       todo!() 
    }
}
