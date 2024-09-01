use crate::{AppState, Configuration};
use actix_web::get;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse};
use std::io;

#[get("/api/all_threads")]
pub async fn all_threads(
    _req: HttpRequest,
    _config: web::Data<Configuration>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, io::Error> {
    // Refresh the threadlist if it's the next day
    state.refresh_threads();

    let body = { state.all_threads.lock().unwrap().as_str().to_owned() };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body))
}
