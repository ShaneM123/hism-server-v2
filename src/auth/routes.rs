use crate::error_handler::ResponseErrorWrapper;
use crate::users::{User, Users, Profile, Profiles};
use actix_web::{delete, put, post, get, web, HttpResponse};
use actix_session::Session;
use serde_json::json;
use crate::Pool;
use uuid::Uuid;

// dd password verification //
#[post("/createusers")]
async fn create(pool: web::Data<Pool>, user: web::Json<User>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let user = Users::create_user(conn, user.into_inner())?;
    // NEED TO REFACTOR THIS TO MAKE USER AND PROFILE CREATION MORE STREAMLINED //
    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn finduser(pool: web::Data<Pool>, credentials: web::Json<User>, session: Session) -> Result<HttpResponse, ResponseErrorWrapper>{
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
        Err(ResponseErrorWrapper::new(401, "Credentials not valid!".to_string()))
    }
}

#[post("/logout")]
async fn sign_out(session: Session) -> Result<HttpResponse, ResponseErrorWrapper> {
    let id: Option<String> = session.get("user_id")?;

    if let Some(_) = id {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({ "message": "Successfully signed out" })))
    }
    else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

#[get("/whoami")]
async fn who_am_i(pool: web::Data<Pool>, session: Session) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let user = Users::find(conn,id)?;
        Ok(HttpResponse::Ok().json(user))
    }
    else {
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}
#[delete("/deleteme")]
async fn delete_user(pool: web::Data<Pool>,session: Session) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();
    let id: Option<String> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_user = Users::delete(conn, id)?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user})))
    }
    else{
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

#[get("/createprofiles/{id}")]
async fn create_profile(pool: web::Data<Pool>, id: web::Path<String>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let profile = Profiles::create_profile(conn, id.into_inner())?;
    Ok(HttpResponse::Ok().json(profile))
}

#[put("/updateprofiles")]
async fn update_profile(pool: web::Data<Pool>, profile: web::Json<Profile>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let profile = Profiles::update_profile(conn, profile.into_inner())?;
    Ok(HttpResponse::Ok().json(profile))
}

#[get("/viewprofile/{id}")]
async fn view_profile(pool: web::Data<Pool>, id: web::Path<String>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let profile = Profiles::find_profile(conn, id.into_inner())?;
    Ok(HttpResponse::Ok().json(profile))
}
#[delete("/deleteprofile")]
async fn delete_profile(pool: web::Data<Pool>,session: Session) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();
    let id: Option<String> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_user = Profiles::delete(conn, id)?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user})))
    }
    else{
        Err(ResponseErrorWrapper::new(401, "Unauthorized".to_string()))
    }
}

pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(finduser);
    config.service(who_am_i);
    config.service(sign_out);
    config.service(create);
    config.service(delete_user);
    config.service(create_profile);
    config.service(update_profile);
    config.service(view_profile);
    config.service(delete_profile);
}