#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, App, HttpServer, HttpResponse, http, web, HttpRequest };
use dotenv::dotenv;
use std::env;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use actix_web::error::PayloadError::Http2Payload;
use actix_cors::Cors;
use std::fs::File;
use std::io::BufReader;
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use actix_redis::{RedisSession};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


mod users;
mod schema;
mod error_handler;
mod auth;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();


    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let port = 8443;

    let redis_port = env::var("REDIS_PORT").expect("Redis port not set");
    let redis_host = env::var("REDIS_HOST").expect("Redis port not set");

    println!("starting http server at {:?}", port);
    HttpServer::new(move|| {
        App::new()
            .data(pool.clone())
            .wrap(RedisSession::new(format!("{}:{}", redis_host, redis_port), &[0; 32]))
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish()
            )
            .wrap(middleware::Logger::default())
            .configure(auth::init_routes)
    })
        .bind_rustls("0.0.0.0:8443",config)?
        .run()
        .await

}