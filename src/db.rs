use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};

pub async fn create_db_pool() -> Pool<MySql> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:a15070487625@localhost:3306/test")
        .await
        .expect("Failed to create db pool");

    pool
}