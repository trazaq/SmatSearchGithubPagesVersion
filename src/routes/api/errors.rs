use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde_json::json;

#[derive(Debug, Display)]
#[allow(unused)]
pub enum SearchError {
    #[display(fmt = "{_0}")]
    InternalError(String),

    #[display(fmt = "{_0}")]
    BadClientData(String),

    #[display(fmt = "{_0}")]
    Unauthorized(String),

    #[display(fmt = "{_0}")]
    NotFound(String),

    #[display(fmt = "{_0}")]
    Timeout(String),
}

impl ResponseError for SearchError {
    fn status_code(&self) -> StatusCode {
        match self {
            SearchError::InternalError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::BadClientData(_msg) => StatusCode::BAD_REQUEST,
            SearchError::Unauthorized(_msg) => StatusCode::UNAUTHORIZED,
            SearchError::NotFound(_msg) => StatusCode::NOT_FOUND,
            SearchError::Timeout(_msg) => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({"error" : htmlescape::encode_minimal(&self.to_string())}))
    }
}
