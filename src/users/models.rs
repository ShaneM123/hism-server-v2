use serde::{Deserialize, Serialize};
use crate::error_handler::CustomError;
use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: String,
}
#[derive(Queryable, PartialEq, Identifiable, Debug, Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub id: i32,
    pub username: String,
}

impl Users {
    pub fn find(conn: &SqliteConnection, id: i32) -> Result<Self, CustomError> {
        let user = user::table.filter(users::id.eq(id)).first(conn)?;
        Ok(user)
    }
    pub fn create_user(conn: &SqliteConnection, user: User) -> Result<Self, CustomerError> {
        conn.transaction(|| {
            diesel::insert_into( users::table)
                .values((
                    users::username.eq(&user.username.to_string()),
                    ))
                .execute(conn)?;

            let user = users::table
                .filter(users::username.eq(&user.username.to_string())).first(conn)?;
            Ok(user)
        })
    }

    pub fn update(conn: &SqliteConnection, id: i32, user: User) -> Result<Self, CustomerError> {
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
}

impl User {
    fn from(user: User) -> User {
        User {
            username: user.username,
}
}
}