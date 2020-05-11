#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, App, HttpServer, };
use dotenv::dotenv;
use std::env;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use actix_web::error::PayloadError::Http2Payload;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod users;
mod schema;
mod error_handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let port = 8998;
    println!("starting http server at {:?}", port);
    HttpServer::new(move|| {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(users::init_routes)
    })
        .bind(("127.0.0.1",port))?
        .run()
        .await

}