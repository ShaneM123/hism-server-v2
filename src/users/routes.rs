use crate::users::{User, Users, Profile, Profiles};
use crate::error_handler::ResponseErrorWrapper;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::Pool;

// CODE ON THIS PAGE IS NO LONGER USED. PLEASE SEE auth/routes inventory/routes or location/routes INSTEAD//
#[get("/users/{id}")]
async fn find(pool: web::Data<Pool>, id: web::Path<String>) -> Result<HttpResponse, ResponseErrorWrapper>{
    let conn = &pool.get().unwrap();
    let user = Users::find(conn, id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
// CODE ON THIS PAGE IS NO LONGER USED. PLEASE SEE auth/routes inventory/routes or location/routes INSTEAD//
#[post("/users")]
async fn finduser(pool: web::Data<Pool>, username: web::Json<User>) -> Result<HttpResponse, ResponseErrorWrapper>{
    let conn = &pool.get().unwrap();
    let credentials = username.into_inner();
    let user = Users::findusername(conn, credentials.username)?;
    Ok(HttpResponse::Ok().json(user))
}
// CODE ON THIS PAGE IS NO LONGER USED. PLEASE SEE auth/routes inventory/routes or location/routes INSTEAD//
#[post("/createusers")]
async fn create(pool: web::Data<Pool>, user: web::Json<User>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let user = Users::create_user(conn, user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
// CODE ON THIS PAGE IS NO LONGER USED. PLEASE SEE auth/routes inventory/routes or location/routes INSTEAD//
#[put("/users/{id}")]
async fn update(
    pool: web::Data<Pool>,
    id: web::Path<String>,
    user: web::Json<User>,
) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let user = Users::update(conn, id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
// CODE ON THIS PAGE IS NO LONGER USED. PLEASE SEE auth/routes inventory/routes or location/routes INSTEAD//
#[delete("/users/{id}")]
async fn delete(pool: web::Data<Pool>,id: web::Path<String>) -> Result<HttpResponse, ResponseErrorWrapper> {
    let conn = &pool.get().unwrap();

    let deleted_user = Users::delete(conn, id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user})))
}
// CODE ON THIS PAGE IS NO LONGER USED. PLEASE SEE auth/routes inventory/routes or location/routes INSTEAD//

#[allow(dead_code)]
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(finduser);
    config.service(create);
    config.service(update);
    config.service(delete);

}
// CODE ON THIS PAGE IS NO LONGER USED. PLEASE SEE auth/routes inventory/routes or location/routes INSTEAD//