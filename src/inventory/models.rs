use serde::{Deserialize, Serialize};
use crate::error_handler::ResponseErrorWrapper;
use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "inventories"]
pub struct Inventory {
    pub user_id: String,
    pub total_items: i32,
}
#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "inventories"]
pub struct Inventories {
    pub id: i32,
    pub user_id: String,
    pub total_items: i32,
}

impl Inventories {
    pub fn create_inventory(conn: &SqliteConnection, inventory: Inventory) -> Result<Self, ResponseErrorWrapper>{
        conn.transaction(|| {
            diesel::insert_into( inventories::table)
                .values((
                    inventories::user_id.eq(&inventory.user_id),
                    inventories::total_items.eq(inventory.total_items),
                ))
                .execute(conn)?;

            let inventory = inventories::table
                .filter(inventories::user_id.eq(&inventory.user_id)).first(conn)?;
            Ok(inventory)
        })
    }
    pub fn update_inventory(conn: &SqliteConnection, id: i32, inventory: Inventory) -> Result<Self, ResponseErrorWrapper> {
        conn.transaction(|| {
            diesel::update( inventories::table.filter(inventories::id.eq(&id)))
                .set((inventories::total_items.eq(&inventory.total_items), inventories::total_items.eq(&inventory.total_items)))
                .execute(conn)?;

            let inventory = inventories::table
                .filter(inventories::id.eq(&id)).first(conn)?;
            Ok(inventory)
        })
    }

    pub fn delete_inventory(conn: &SqliteConnection, id: i32) -> Result<usize, ResponseErrorWrapper> {
        let res = diesel::delete( inventories::table.filter(inventories::id.eq(id))).execute(conn)?;
        Ok(res)
    }
    pub fn find_inventory(conn: &SqliteConnection, id: i32) -> Result<Self, ResponseErrorWrapper> {
        let inventory = inventories::table.filter(inventories::id.eq(id)).first(conn)?;
        Ok(inventory)
    }
/*    pub fn find_inventory_by_user(conn: &SqliteConnection, user_id: i32) -> Result<Self, ResponseErrorWrapper> {
        let inventory = inventories::table.filter(inventories::user_id.eq(user_id)).first(conn)?;
        Ok(inventory)
    }*/

}
impl From<Inventory> for Inventories {
    fn from(inventory: Inventory) -> Self {
        Inventories {
            id: 0,
            user_id: inventory.user_id,
            total_items: inventory.total_items,
        }
    }
}


#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "items"]
pub struct Item {
    pub inventory_id: i32,
    pub dimensions: i32,
    pub weight: i32,
    pub value: i32,
    pub description: String,
}

#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, PartialEq, Insertable)]
#[table_name = "items"]
pub struct Items {
    pub id: i32,
    pub inventory_id: i32,
    pub dimensions: i32,
    pub weight: i32,
    pub value: i32,
    pub description: String,
}

impl Items {
    pub fn create_item(conn: &SqliteConnection, item: Item) -> Result<Self, ResponseErrorWrapper>{
        conn.transaction(|| {
            diesel::insert_into( items::table)
                .values((
                    items::inventory_id.eq(item.inventory_id),
                    items::dimensions.eq(item.dimensions),
                    items::weight.eq(item.weight),
                    items::value.eq(item.value),
                    items::description.eq(item.description),
                ))
                .execute(conn)?;

            let item = items::table
                .filter(items::inventory_id.eq(&item.inventory_id)).first(conn)?;
            Ok(item)
        })
    }
    pub fn update_items(conn: &SqliteConnection, id: i32, item: Item) -> Result<Self, ResponseErrorWrapper> {
        conn.transaction(|| {
            diesel::update( items::table.filter(items::id.eq(&id)))
                .set((items::inventory_id.eq(item.inventory_id),items::dimensions.eq(item.dimensions),items::weight.eq(item.weight),items::value.eq(item.value),items::description.eq(item.description),))
                .execute(conn)?;

            let item = items::table
                .filter(items::id.eq(&id)).first(conn)?;
            Ok(item)
        })
    }

    pub fn delete_items(conn: &SqliteConnection, id: i32) -> Result<usize, ResponseErrorWrapper> {
        let res = diesel::delete( items::table.filter(items::id.eq(id))).execute(conn)?;
        Ok(res)
    }
    pub fn find_items(conn: &SqliteConnection, id: i32) -> Result<Self, ResponseErrorWrapper> {
        let user = items::table.filter(items::id.eq(id)).first(conn)?;
        Ok(user)
    }

}

impl From<Item> for Items {
    fn from(item: Item) -> Self {
        Items {
            id: 0,
            inventory_id: item.inventory_id,
            dimensions: item.dimensions,
            weight: item.weight,
            value: item.value,
            description: item.description,
        }
    }
}

