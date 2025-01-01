use actix_web::{web, Responder, Result};
use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> Result<impl Responder> {
    let html = Index.render().expect("Failed to render index.html");
    Ok(web::Html::new(html))
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct Dashboard;

async fn dashboard() -> Result<impl Responder> {
    let html = Dashboard.render().expect("Failed to render dashboard.html");
    Ok(web::Html::new(html))
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFound;

async fn not_found() -> Result<impl Responder> {
    let html = NotFound.render().expect("Failed to render 404.html");
    Ok(web::Html::new(html))
}

// Function to register routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
    cfg.route("/dashboard", web::get().to(dashboard));
    cfg.default_service(web::to(not_found));
}