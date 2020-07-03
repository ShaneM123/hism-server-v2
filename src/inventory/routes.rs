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

#[put("/updateinventory/{id}")]
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

#[get("/inventory/{id}")]
async fn find_inventory(pool: web::Data<Pool>, session: Session, inventory_id: web::Path<i32>) -> Result<HttpResponse, ResponseErrorWrapper>{
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let item = Inventories::find_inventory(conn, inventory_id.into_inner())?;
        Ok(HttpResponse::Ok().json(item))
    } else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/deleteinventory/{id}")]
async fn delete_inventory(pool: web::Data<Pool>,session: Session, inventory_id: web::Path<i32>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();
    let id: Option<String> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_user = Inventories::delete_inventory(conn, inventory_id.into_inner())?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user})))
    }
    else{
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
async fn find_item(pool: web::Data<Pool>, session: Session, item_id: web::Path<i32>) -> Result<HttpResponse, ResponseErrorWrapper>{
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let item = Items::find_items(conn, item_id.into_inner())?;
        Ok(HttpResponse::Ok().json(item))
    } else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

#[put("/updateitem/{id}")]
async fn update_item(
    pool: web::Data<Pool>,
    session: Session, item_id: web::Path<i32>,
    item: web::Json<Item>)
    -> Result<HttpResponse, ResponseErrorWrapper>{
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let item = Items::update_items(conn, item_id.into_inner(),item.into_inner())?;
        Ok(HttpResponse::Ok().json(item))
    }
    else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/deleteitem/{id}")]
async fn delete_item(pool: web::Data<Pool>,session: Session, item_id: web::Path<i32>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();
    let id: Option<String> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_user = Items::delete_items(conn, item_id.into_inner())?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user})))
    }
    else{
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}


pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(create_inventory);
    config.service(find_inventory);
    config.service(update_inventory);
    config.service(delete_inventory);
    config.service(create_item);
    config.service(find_item);
    config.service(update_item);
    config.service(delete_item);
}