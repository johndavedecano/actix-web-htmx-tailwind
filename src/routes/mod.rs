use actix_web::web;

mod public;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    public::configure(cfg);
}
