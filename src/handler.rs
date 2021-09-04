use actix_session::Session;
use actix_web::{get, http::header::LOCATION, post, route, web, HttpRequest, HttpResponse, Result};
use log::debug;
use mongodb::Client;

use crate::{auth::auth_with_password, model::user::User};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[post("/login")]
pub async fn login(
    client: web::Data<Client>,
    session: Session,
    form: web::Form<User>,
) -> Result<HttpResponse> {
    if auth_with_password(&client, &form.username, &form.password).await? {
        session.set("login", true)?;
    } else {
        session.remove("login");
    }
    Ok(
        HttpResponse::Found()
        .set_header(LOCATION, "/")
        .finish()
        .into_body()
      )
}

#[get("/logout")]
pub async fn logout(session: Session) -> HttpResponse {
    session.remove("login");
    HttpResponse::Found()
        .set_header(LOCATION, "/login")
        .finish()
        .into_body()
}

#[get("/login")]
pub async fn login_page(session: Session) -> HttpResponse {
    if session.get::<bool>("login").ok().unwrap().unwrap_or(false) {
        HttpResponse::Found()
            .set_header(LOCATION, "/")
            .finish()
            .into_body()
    } else {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../static/login.html"))
    }
}
