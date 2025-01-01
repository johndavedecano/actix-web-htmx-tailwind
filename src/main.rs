use actix_files::Files;
use actix_web::{middleware, App, HttpServer};

use crate::routes::init_routes;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "./static"))
            .configure(init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
