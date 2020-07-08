use serde::{Deserialize, Serialize};
use crate::error_handler::ResponseErrorWrapper;
use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "user_junctions"]
pub struct Junction {
    pub user_id: String,
    pub inventory_id: i32,
    pub location_id: i32,
}
#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "user_junctions"]
pub struct Junctions {
    pub id: i32,
    pub user_id: String,
    pub inventory_id: i32,
    pub location_id: i32,
}

impl Junctions {
    pub fn create_junction(conn: &SqliteConnection, junction: Junction) -> Result<Self, ResponseErrorWrapper> {
        conn.transaction(|| {
            diesel::insert_into(user_junctions::table)
                .values((
                    user_junctions::user_id.eq(&junction.user_id.to_string()),
                    user_junctions::inventory_id.eq(junction.inventory_id),
                    user_junctions::location_id.eq(junction.location_id),
                )).execute(conn)?;
            let junction = user_junctions::table
                .filter(user_junctions::user_id.eq(&junction.user_id)).first(conn)?;
            Ok(junction)
        })
    }
}