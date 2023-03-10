use actix_web::{web, get, post, Responder};
use sqlx::{Pool, MySql, Row};
use crate::models::user::{User, NewUser};

/// 查询用户
#[get("/user/{user_id}")]
pub async fn get_user(pool: web::Data<Pool<MySql>>, user_id: web::Path<i32>) -> impl Responder {

    let row = sqlx::query("select * from user where id = ?")
        .bind(user_id.into_inner())
        .fetch_optional(pool.get_ref()).await.unwrap().unwrap();

    let user = User {id: row.try_get("id").unwrap(), name: row.try_get("name").unwrap()};

    user
}

/// 创建用户
#[post("/user/create")]
pub async fn add_user(pool: web::Data<Pool<MySql>>, new_user: web::Json<NewUser>) -> impl Responder {

    let todo_id = sqlx::query("insert into user (name) values (?)")
        .bind(&new_user.name)
        .execute(pool.get_ref())
        .await.unwrap().last_insert_id();

    format!("创建成功: {}, name: {}", todo_id, &new_user.name)
}