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
    pub fn find_location(conn: &SqliteConnection, id: i32) -> Result<Self, ResponseErrorWrapper> {
        let location = locations::table.filter(locations::id.eq(id)).first(conn)?;
        Ok(location)
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