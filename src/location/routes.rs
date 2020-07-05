use crate::error_handler::ResponseErrorWrapper;
use crate::location::{Locations, Location};
use actix_web::{delete, put, post, get, web, HttpResponse};
use actix_session::Session;
use serde_json::json;
use crate::Pool;
use actix_web::web::ServiceConfig;


#[post("/createlocation")]
async fn create_location(pool: web::Data<Pool>, session: Session, location: web::Json<Location>) -> Result<HttpResponse, ResponseErrorWrapper> {

    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let location = Locations::create_location(conn, location.into_inner())?;
        Ok(HttpResponse::Ok().json(location))
    }
    else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

#[get("/location/{id}")]
async fn find_location(pool: web::Data<Pool>, session: Session, location_id: web::Path<i32>) -> Result<HttpResponse, ResponseErrorWrapper>{
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let location = Locations::find_location(conn, location_id.into_inner())?;
        Ok(HttpResponse::Ok().json(location))
    } else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

#[put("/updatelocation/{id}")]
async fn update_location(
    pool: web::Data<Pool>,
    session: Session, location_id: web::Path<i32>,
    location: web::Json<Location>)
    -> Result<HttpResponse, ResponseErrorWrapper>{
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let location = Locations::update_locations(conn, location_id.into_inner(),location.into_inner())?;
        Ok(HttpResponse::Ok().json(location))
    }
    else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/deletelocation/{id}")]
async fn delete_location(pool: web::Data<Pool>,session: Session, location_id: web::Path<i32>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();
    let id: Option<String> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_user = Locations::delete_locations(conn, location_id.into_inner())?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user})))
    }
    else{
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}



pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(create_location);
    config.service(find_location);
    config.service(update_location);
    config.service(delete_location);

}