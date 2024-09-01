#![allow(unused_imports)]
use crate::route_config::route_config;
use crate::{get_config, AppState};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};
use chrono::Local;
use std::io;
use std::sync::Mutex;

pub async fn spawn_server(port: u16) -> Result<Server, io::Error> {
    let state = Data::new(AppState::default());

    let server = HttpServer::new(move || {
        let config = get_config();

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .app_data(Data::new(config))
            .app_data(state.clone())
            .configure(route_config)
    })
    .bind(("127.0.0.1", port))?
    .run();

    Ok(server)
}
