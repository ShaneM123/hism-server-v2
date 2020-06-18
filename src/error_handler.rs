use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use actix_session::Session;
use actix_http::error::Error as ActixHttpError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ResponseErrorWrapper {
    pub error_status_code: u16,
    pub error_message: String,
}

impl ResponseErrorWrapper {
    pub fn new(error_status_code: u16, error_message: String) -> ResponseErrorWrapper {
        ResponseErrorWrapper {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for ResponseErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}
impl From<ActixHttpError> for ResponseErrorWrapper{
    fn from(error: ActixHttpError) -> Self {
        ResponseErrorWrapper::new(500, error.to_string())    }
}
impl From<DieselError> for ResponseErrorWrapper {
    fn from(error: DieselError) -> ResponseErrorWrapper{
        match error {
            DieselError::DatabaseError(_, err) => ResponseErrorWrapper::new(409, err.message().to_string()),
            DieselError::NotFound => {
                ResponseErrorWrapper::new(404, "the user record was not found".to_string())
            }
            err => ResponseErrorWrapper::new(500, format!("Other unknown Diesel Error:{}", err)),

        }
    }
}

impl ResponseError for ResponseErrorWrapper {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.error_status_code){
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < 500 {
            true => self.error_message.clone(),
            false => "Internal server error".to_string(),
        };
        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}