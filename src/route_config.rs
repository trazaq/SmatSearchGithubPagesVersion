use crate::routes::api::all_sites::all_sites;
use crate::routes::api::all_threads::all_threads;
use crate::routes::api::download::download;
use crate::routes::api::search_results::search_results;
use actix_files::Files;
use actix_web::web;

pub fn route_config(cfg: &mut web::ServiceConfig) {
    // Note that the root path should always be defined as the last item

    // Routes for frontend: /api/...
    cfg.service(all_threads);
    cfg.service(all_sites);
    cfg.service(search_results);
    cfg.service(download);

    cfg.service(Files::new("/", "./static/root/build/").index_file("app.html"));
}
