use serde::{Deserialize, Serialize};
use crate::error_handler::CustomError;
use crate::schema::users;
use diesel::prelude::*;
use argon2::{Config, verify_encoded};
use rand::Rng;
use uuid::Uuid;
//TODO: introduce Uuid//

#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}
#[derive(Queryable, PartialEq, Identifiable, Debug, Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub id: Uuid,
    pub username: String,
   #[serde(skip_serializing)]
    pub password: String,
}

impl Users {
    pub fn find(conn: &SqliteConnection, id: i32) -> Result<Self, CustomError> {
        let user = users::table.filter(users::id.eq(id)).first(conn)?;
        Ok(user)
    }
    pub fn findusername(conn: &SqliteConnection, user: User) -> Result<Self, CustomError> {
        let theuser = users::table
            .filter(users::username.eq(user.username))
            //.filter(users::password.eq(user.password))
            .select((users::id, users::username, users::password,))
            .first::<Users>(conn)?;
        /*argon2::verify_encoded(&theuser.password,&user.password.as_bytes())
            .map_err(|e| CustomError::new(500, format!("Failed to verify password: {}", e)));*/
        Ok(theuser)
    }
    pub fn create_user(conn: &SqliteConnection, user: User) -> Result<Self, CustomError> {
        let mut user = Users::from(user);
        user.hash_password()?;
        conn.transaction(|| {
            diesel::insert_into( users::table)
                .values((
                    users::id.eq(&user.id),
                    users::username.eq(&user.username.to_string()),
                    users::password.eq(&user.password.to_string()),
                    ))
                .execute(conn)?;

            let user = users::table
                .filter(users::username.eq(&user.username.to_string())).first(conn)?;
            Ok(user)
        })
    }
    /*pub fn hash_password(&mut self) -> Result<(), CustomError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| CustomError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())

    }*/


    pub fn update(conn: &SqliteConnection, id: i32, user: User) -> Result<Self, CustomError> {
        conn.transaction(|| {
            diesel::insert_into( users::table)
                .values((users::id.eq(&id), users::username.eq(&user.username)))
                .execute(conn)?;

            let user = users::table
                .filter(users::id.eq(&id)).first(conn)?;
            Ok(user)
        })
    }

    pub fn delete(conn: &SqliteConnection, id: i32) -> Result<usize, CustomError> {
        let res = diesel::delete( users::table.filter(users::id.eq(id))).execute(conn)?;
        Ok(res)
    }


    pub fn verify_password(&self, password: &[u8]) -> Result<bool, CustomError> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| CustomError::new(500, format!("Failed to verify password: {}", e)))
    }
}


impl User {
/*    fn from(user: User) -> User {
        User {
            username: user.username,
            password: user.password,
        }
}*/
    pub fn hash_password(&mut self) -> Result<(), CustomError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| CustomError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())

    }

}

impl From<User> for Users {
    fn from(user: User) -> Self {
        Users {
            id: Uuid::new_v4(),
            username: user.username,
            password: user.password,
        }
    }
}
