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
#[template(path = "forgot.html")]
struct Forgot;

async fn forgot() -> Result<impl Responder> {
    let html = Forgot.render().expect("Failed to render forgot.html");
    Ok(web::Html::new(html))
}

#[derive(Template)]
#[template(path = "login.html")]
struct Login;

async fn login() -> Result<impl Responder> {
    let html = Login.render().expect("Failed to render login.html");
    Ok(web::Html::new(html))
}

#[derive(Template)]
#[template(path = "register.html")]
struct Register;

async fn register() -> Result<impl Responder> {
    let html = Register.render().expect("Failed to render register.html");
    Ok(web::Html::new(html))
}

#[derive(Template)]
#[template(path = "reset.html")]
struct Reset;

async fn reset() -> Result<impl Responder> {
    let html = Reset.render().expect("Failed to render reset.html");
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
    cfg.route("/login", web::get().to(login));
    cfg.route("/register", web::get().to(register));
    cfg.route("/forgot", web::get().to(forgot));
    cfg.route("/reset", web::get().to(reset));
    cfg.default_service(web::to(not_found));
}