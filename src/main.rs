use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use env_logger::Env;
use smatdb_search_actixweb::route_config::route_config;
use smatdb_search_actixweb::{get_config, AppState};
use std::io;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt;

// Building for other environments:
// Docker
// cross build --target x86_64-unknown-linux-musl --release
// Podman
// CROSS_CONTAINER_ENGINE=podman cross build --target x86_64-unknown-linux-musl --release

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    //env_logger::init();
    dotenvy::dotenv().expect("Error Loading .env file");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .pretty()
        .with_max_level(tracing::Level::INFO)
        // Enable if you want color output in the CONSOLE (will cause weird characters in the log)
        .with_ansi(false)
        .with_timer(fmt::time::OffsetTime::local_rfc_3339().unwrap())
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let state = Data::new(AppState::default());

    HttpServer::new(move || {
        let config = get_config();

        App::new()
            .wrap(Cors::default().allow_any_origin())
            .wrap(middleware::Compress::default())
            //.wrap(Logger::default())
            .wrap(TracingLogger::default())
            .app_data(Data::new(config))
            .app_data(state.clone())
            .configure(route_config)
    })
    .bind((
        "0.0.0.0",
        dotenvy::var("PORT")
            .expect("Error: Set the 'PORT' key in .env")
            .parse()
            .unwrap(),
    ))?
    .run()
    .await
}
