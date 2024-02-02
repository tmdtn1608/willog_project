use crate::services::SetTemperature;
use crate::structs::{TemperaturRegistRequest, TemperaturRegistResponse};
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{post, State};
use sqlx::{MySql, Pool};

#[post("/Set/Temperature", format = "json", data = "<body>")]
pub async fn set_temperature_controller(
    pool: &State<Pool<MySql>>,
    body: Json<TemperaturRegistRequest>,
) -> status::Custom<String> {
    let set_temperature_service = SetTemperature {
        d_snumber: body.serialNumber.clone(),
        temperature: body.temperatures.clone(),
        interval: body.interval.clone(),
        registered_at: body.registered_at.clone(),
    };

    let insert_result = set_temperature_service.add(pool).await;

    // 실행확인
    let result_msg = if insert_result {
        "success".to_string()
    } else {
        "failed".to_string()
    };

    let response = TemperaturRegistResponse { msg: result_msg };

    let json_str = serde_json::to_string(&response).expect("Failed to serialize to JSON");

    Custom(Status::Ok, json_str)
}
