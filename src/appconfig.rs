use actix_web::web;

use crate::handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::index)
        .service(handler::login_page)
        .service(handler::login)
        .service(handler::logout);
}
