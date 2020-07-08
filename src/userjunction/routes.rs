use crate::error_handler::ResponseErrorWrapper;
use crate::userjunction::{Junctions, Junction};
use actix_web::{delete, put, post, get, web, HttpResponse};
use actix_session::Session;
use serde_json::json;
use crate::Pool;
use actix_web::web::ServiceConfig;


#[post("/createuserjunction")]
async fn create_location(pool: web::Data<Pool>, session: Session, junction: web::Json<Junction>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let junction = Junctions::create_junction(conn, junction.into_inner())?;
        Ok(HttpResponse::Ok().json(junction))
    }
    else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(create_location);
}