pub mod db;
pub mod dao;
mod models;
mod services;

use actix_web::{get, web, App, HttpServer, Responder};
use db::create_db_pool;
use sqlx::{Row, Pool, MySql};
use futures::stream::TryStreamExt;
use crate::services::handler::user::{ get_user, add_user };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(crate::services::handler::user::get_user)
            .service(add_user)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
