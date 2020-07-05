use serde::{Deserialize, Serialize};
use crate::error_handler::ResponseErrorWrapper;
use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "locations"]
pub struct Location {
    pub user_id: String,
    pub home_location: i32,
    pub geo_location: i32,
}
#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "locations"]
pub struct Locations {
    pub id: i32,
    pub user_id: String,
    pub home_location: i32,
    pub geo_location: i32,
}

impl Locations {
    pub fn create_location(conn: &SqliteConnection, location: Location) -> Result<Self, ResponseErrorWrapper> {
       conn.transaction(|| {
           diesel::insert_into( locations::table)
               .values((
                   locations::user_id.eq(&location.user_id.to_string()),
                   locations::home_location.eq(location.home_location),
                   locations::geo_location.eq(location.geo_location),
                   )).execute(conn)?;
           let location = locations::table
               .filter( locations::user_id.eq(&location.user_id)).first(conn)?;
           Ok(location)

       })
    }

    pub fn find_location(conn: &SqliteConnection, id: i32) -> Result<Self, ResponseErrorWrapper> {
        let location = locations::table.filter(locations::id.eq(id)).first(conn)?;
        Ok(location)
    }

    pub fn update_locations(conn: &SqliteConnection, id: i32, location: Location) -> Result<Self, ResponseErrorWrapper> {
        conn.transaction(|| {
            diesel::update( locations::table.filter(locations::id.eq(&id)))
                .set((locations::user_id.eq(location.user_id),locations::geo_location.eq(location.geo_location),locations::home_location.eq(location.home_location),))
                .execute(conn)?;

            let location = locations::table
                .filter(locations::id.eq(&id)).first(conn)?;
            Ok(location)
        })
    }

    pub fn delete_locations(conn: &SqliteConnection, id: i32) -> Result<usize, ResponseErrorWrapper> {
        let res = diesel::delete( locations::table.filter(locations::id.eq(id))).execute(conn)?;
        Ok(res)
    }

}

impl From<Location> for Locations {
    fn from(location: Location) -> Self {
        Locations {
            id: 0,
            user_id: location.user_id,
            home_location: location.home_location,
            geo_location: location.geo_location,
        }
    }
}