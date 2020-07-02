use crate::error_handler::ResponseErrorWrapper;
use crate::inventory::{Inventory, Inventories, Item, Items};
use actix_web::{delete, put, post, get, web, HttpResponse};
use actix_session::Session;
use serde_json::json;
use crate::Pool;
use uuid::Uuid;
use actix_web::web::ServiceConfig;

#[post("/createinventory")]
async fn create_inventory(pool: web::Data<Pool>, session: Session, inventory: web::Json<Inventory>) -> Result<HttpResponse, ResponseErrorWrapper> {

    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let inventory = Inventories::create_inventory(conn, inventory.into_inner())?;
        Ok(HttpResponse::Ok().json(inventory))
    }
    else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}
#[put("/updateinventory")]
async fn update_inventory(
    pool: web::Data<Pool>,
    session: Session, inventory_id: web::Path<i32>,
    inventory: web::Json<Inventory>)
    -> Result<HttpResponse, ResponseErrorWrapper>{
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let inventory = Inventories::update_inventory(conn, inventory_id.into_inner(),inventory.into_inner())?;
        Ok(HttpResponse::Ok().json(inventory))
    }
    else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }

}

#[post("/createitem")]
async fn create_item(pool: web::Data<Pool>, session: Session, item: web::Json<Item>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let item = Items::create_item(conn, item.into_inner())?;
        Ok(HttpResponse::Ok().json(item))
    } else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}
#[get("/finditem/{id}")]
async fn find(pool: web::Data<Pool>, session: Session, item_id: web::Path<i32>) -> Result<HttpResponse, ResponseErrorWrapper>{
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let item = Items::find_items(conn, item_id.into_inner())?;
        Ok(HttpResponse::Ok().json(item))
    } else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}



pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(create_inventory);
    config.service(create_item);

}