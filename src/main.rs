extern crate dotenv;

use dotenv::dotenv;

pub mod api; // api 파일
pub mod db; // db connection
pub mod function; // 유틸 및 함수
pub mod services; // 서비스
pub mod structs; // 구조체
use crate::db::db_connect;
use rocket::routes;

#[rocket::main]
async fn main() {
    dotenv().ok();
    let pool: sqlx::Pool<sqlx::MySql> = db_connect().await.expect("Failed to create database pool");

    rocket::build()
        .manage(pool)
        .mount(
            "/",
            routes![
                api::regist_device_controller,
                api::regist_group_controller,
                api::set_temperature_controller,
                api::get_temperature_device_controller,
                api::get_temperature_group_controller
            ],
        )
        .launch()
        .await
        .expect("Failed to launch Rocket application");
}
