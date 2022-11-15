#![deny(clippy::all)]

// #[macro_use]
extern crate diesel;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

mod api;
mod repository;
mod services;
mod types;

use api::{post::routes_posts, test::routes_tests};
// use repository::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initiate logger
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    log::info!("starting HTTP server at http://localhost:80");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(routes_tests())
            .service(routes_posts())
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
