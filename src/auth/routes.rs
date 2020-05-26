use crate::error_handler::CustomError;
use crate::users::{User, Users};
use actix_web::{post, get, web, HttpResponse};
use actix_session::Session;
use serde_json;
use crate::Pool;
use uuid::Uuid;

#[post("/createusers")]
async fn create(pool: web::Data<Pool>, user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let conn = &pool.get().unwrap();

    let user = Users::create_user(conn, user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn finduser(pool: web::Data<Pool>, credentials: web::Json<User>) -> Result<HttpResponse, CustomError>{
    let conn = &pool.get().unwrap();
    let user = Users::findusername(conn, credentials.into_inner())?;

    Ok(HttpResponse::Ok().json(user))
}