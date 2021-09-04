use std::{fmt::Error, io};

use actix_redis::RedisSession;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv;
use middleware::login;
use mongodb::{options::ClientOptions, Client};

mod appconfig;
mod auth;
mod error;
mod handler;
mod middleware;
mod model;


#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let mongo_addr = dotenv::var("MONGODB").unwrap_or("127.0.0.1:27017".to_owned());
    let mongo_client = Client::with_options(
        ClientOptions::parse(&format!("mongodb://{}", mongo_addr))
            .await
            .map_err(|err: mongodb::error::Error| {
                io::Error::new(io::ErrorKind::Other, err.to_string())
            })?,
    ).unwrap();
    let mongo_client = web::Data::new(mongo_client);

    HttpServer::new(move || {
        let pk = dotenv::var("COOKIE_PK").unwrap_or(String::from_utf8(vec![0; 32]).unwrap());
        let redis_addr = dotenv::var("REDIS").unwrap_or("127.0.0.1:6379".to_owned());
        App::new()
            .wrap(Logger::default())
            .wrap(login::LoginRequired)
            .wrap(
                RedisSession::new(&redis_addr, &pk.as_bytes())
                    .cookie_name("session")
                    .cookie_secure(false),
            )
            .app_data(mongo_client.clone())
            .configure(appconfig::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
