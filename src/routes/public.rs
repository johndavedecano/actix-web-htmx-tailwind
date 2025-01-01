use actix_web::{web, Responder, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> Result<impl Responder> {
    let html = Index.render().expect("template should be valid");
    Ok(web::Html::new(html))
}

// Function to register routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
}