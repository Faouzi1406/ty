extern crate bcrypt;
use crate::schema::*;
use crate::traits::db::Create;
use crate::{lib_db::db_connection::db_connection, traits::db::ReadWrite};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
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

    fn delete(&self) -> bool {
        let mut connection = db_connection();
        let delete = diesel::delete(users::table.filter(users::username.eq(&self.username)))
            .execute(&mut connection)
            .expect("Error deleting user");

        match delete {
            0 => false,
            _ => true,
        }
    }

    /// Returns all users in the database
    fn all() -> Result<Vec<User>, diesel::result::Error> {
        let mut connection = db_connection();

        let users = users::table
            .select((users::id, users::username, users::email))
            .load::<User>(&mut connection);

        match users {
            Ok(users) => Ok(users),
            Err(e) => Err(e),
        }
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

        // hash self.password and store in db in thread
        let hash_pass = Arc::new(self.clone());
        std::thread::spawn(move || {
            let hash_pass = hash_pass.clone();
            let hashed = hash(&hash_pass.password, DEFAULT_COST).unwrap();

            diesel::update(users::table.filter(users::username.eq(&hash_pass.username)))
                .set(users::password.eq(hashed))
                .execute(&mut connection)
                .expect_err("Error hashing password in db");

        });

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }
}
