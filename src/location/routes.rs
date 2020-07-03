use crate::error_handler::ResponseErrorWrapper;
use crate::location::{Locations, Location};
use actix_web::{delete, put, post, get, web, HttpResponse};
use actix_session::Session;
use serde_json::json;
use crate::Pool;
use actix_web::web::ServiceConfig;


pub fn init_routes(){}