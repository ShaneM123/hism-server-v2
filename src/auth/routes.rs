use crate::error_handler::CustomError;
use crate::users::{User, Users};
use actix_web::{post, get, web, HttpResponse};
use actix_session::Session;
use serde_json::json;
use crate::Pool;
use uuid::Uuid;

#[post("/createusers")]
async fn create(pool: web::Data<Pool>, user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let conn = &pool.get().unwrap();

    let user = Users::create_user(conn, user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn finduser(pool: web::Data<Pool>, credentials: web::Json<User>, session: Session) -> Result<HttpResponse, CustomError>{
    let conn = &pool.get().unwrap();
    let credentials = credentials.into_inner();
    let user = Users::findusername(conn, credentials.username)?;
    let is_valid= user.verify_password(credentials.password.as_bytes())?;

    if is_valid == true {
        session.set("user_id", &user.id)?;
        session.renew();

        Ok(HttpResponse::Ok().json(user))
    }
    else{
        Err(CustomError::new(401, "Credentials not valid!".to_string()))
    }
}

#[post("/logout")]
async fn sign_out(session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<String> = session.get("user_id")?;

    if let Some(_) = id {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({ "message": "Successfully signed out" })))
    }
    else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[get("/whoami")]
async fn who_am_i(pool: web::Data<Pool>, session: Session) -> Result<HttpResponse, CustomError> {
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let user = Users::find(conn,id)?;
        Ok(HttpResponse::Ok().json(user))
    }
    else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(finduser);
    config.service(who_am_i);
    config.service(sign_out);
    config.service(create);

}