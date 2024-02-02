use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::{get, State};
use serde::Serialize;
use sqlx::{MySql, Pool};

#[derive(Serialize)]
struct TeapotResponse {
    body: String,
}

#[derive(sqlx::FromRow)]
struct QueryResult {
    cd: String,
}

#[get("/")]
pub async fn test(pool: &State<Pool<MySql>>) -> status::Custom<String> {
    let rows_result =
        sqlx::query_as::<_, QueryResult>("SELECT DATE_FORMAT(CURRENT_DATE(), '%Y-%m-%d') as cd")
            .fetch_one(&**pool)
            .await;

    let rows = match rows_result {
        Ok(result) => result,
        Err(err) => panic!("Failed to fetch data: {}", err),
    };

    let cd_value: &str = &rows.cd;

    let teapot = TeapotResponse {
        body: (*cd_value).to_string(),
    };
    let json_str = serde_json::to_string(&teapot).expect("Failed to serialize to JSON");

    Custom(Status::Ok, json_str)
}
