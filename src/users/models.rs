use serde::{Deserialize, Serialize};
use crate::error_handler::ResponseErrorWrapper;
use crate::schema::*;
use diesel::prelude::*;
use argon2::{Config, verify_encoded};
use rand::Rng;
use uuid::Uuid;
//TODO: figure out how to change uuid to binary safely, implement profile and community pages//

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
    pub id: String,
    pub username: String,
   #[serde(skip_serializing)]
    pub password: String,
}

impl Users {
    pub fn find(conn: &SqliteConnection, id: String) -> Result<Self, ResponseErrorWrapper> {
        let user = users::table.filter(users::id.eq(id)).first(conn)?;
        Ok(user)
    }
    pub fn findusername(conn: &SqliteConnection, user: String) -> Result<Self, ResponseErrorWrapper> {
        let theuser = users::table
            .filter(users::username.eq(user))
            //.filter(users::password.eq(user.password))
            .select((users::id, users::username, users::password,))
            .first::<Users>(conn)?;
        /*argon2::verify_encoded(&theuser.password,&user.password.as_bytes())
            .map_err(|e| ResponseErrorWrapper::new(500, format!("Failed to verify password: {}", e)));*/
        Ok(theuser)
    }
    pub fn create_user(conn: &SqliteConnection, user: User) -> Result<Self, ResponseErrorWrapper> {
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
    /*pub fn hash_password(&mut self) -> Result<(), ResponseErrorWrapper> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ResponseErrorWrapper::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())

    }*/


    pub fn update(conn: &SqliteConnection, id: String, user: User) -> Result<Self, ResponseErrorWrapper> {
        conn.transaction(|| {
            diesel::insert_into( users::table)
                .values((users::id.eq(&id), users::username.eq(&user.username)))
                .execute(conn)?;

            let user = users::table
                .filter(users::id.eq(&id)).first(conn)?;
            Ok(user)
        })
    }

    pub fn delete(conn: &SqliteConnection, id: String) -> Result<usize, ResponseErrorWrapper> {
        let res = diesel::delete( users::table.filter(users::id.eq(id))).execute(conn)?;
        Ok(res)
    }


    pub fn verify_password(&self, password: &[u8]) -> Result<bool, ResponseErrorWrapper> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| ResponseErrorWrapper::new(500, format!("Failed to verify password: {}", e)))
    }

    pub fn hash_password(&mut self) -> Result<(), ResponseErrorWrapper> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ResponseErrorWrapper::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())

    }
}


impl User {
/*    fn from(user: User) -> User {
        User {
            username: user.username,
            password: user.password,
        }
}*/
    pub fn hash_password(&mut self) -> Result<(), ResponseErrorWrapper> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ResponseErrorWrapper::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())

    }

}

impl From<User> for Users {
    fn from(user: User) -> Self {
        Users {
            id: Uuid::new_v4().to_string(),
            username: user.username,
            password: user.password,
        }
    }
}


#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "profiles"]
pub struct Profile {
    pub id: String,
    pub bio: String,
    pub age: i32,
    pub community: String,
}
#[derive(Queryable, PartialEq, Identifiable, Debug, Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "profiles"]
pub struct Profiles {
    pub profile_id: i32,
    pub id: String,
    pub bio: String,
    pub age: i32,
    pub community: String,
}

impl Profiles {
    pub fn create_profile(conn: &SqliteConnection, user: Users) -> Result<Self, ResponseErrorWrapper>{
          let mut profile = Profiles::from(user);
        conn.transaction(|| {
            diesel::insert_into( profiles::table)
                .values((
                    profiles::id.eq(&profile.id.to_string()),
                    profiles::bio.eq(&profile.bio.to_string()),
                    profiles::age.eq(&profile.age),
                    profiles::community.eq(&profile.community.to_string()),
                ))
                .execute(conn)?;

            let profile = profiles::table
                .filter(profiles::id.eq(&profile.id.to_string())).first(conn)?;
            Ok(profile)
        })
    }

    pub fn update_profile(conn: &SqliteConnection, profile: Profile) -> Result<Self, ResponseErrorWrapper> {
        let mut profile = Profiles::from(profile);
        conn.transaction(|| {
            diesel::update( profiles::table.filter(profiles::id.eq(&profile.id)))
                .set((profiles::bio.eq(&profile.bio), profiles::age.eq(&profile.age), (profiles::community.eq(&profile.community)
                )))
                .execute(conn)?;

            let profile = profiles::table
                .filter(profiles::id.eq(&profile.id)).first(conn)?;
            Ok(profile)
        })
    }

}

impl From<Profile> for Profiles {
    fn from(profile: Profile) -> Self {
        Profiles {
            profile_id: 0,
            id: profile.id,
            bio: profile.bio,
            age: profile.age,
            community: profile.community,
        }
    }
}
impl From<Users> for Profiles {
    fn from(user: Users) -> Self {
        Profiles {
            profile_id: 0,
            id: user.id,
            bio: "".to_string(),
            age: 0,
            community: "".to_string(),
        }
    }

}