use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{post, State};
use sqlx::{MySql, Pool};

use crate::services::GetTemperature;
use crate::structs::{
    TemperatureDeviceRequest, TemperatureDeviceResponse, TemperatureGroupRequest,
    TemperatureGroupResponse,
};

#[post("/GetTemperature/Device", format = "json", data = "<body>")]
pub async fn get_temperature_device_controller(
    pool: &State<Pool<MySql>>,
    body: Json<TemperatureDeviceRequest>,
) -> status::Custom<String> {
    let get_temperature_service = GetTemperature {
        start_date: body.startDate.clone(),
        end_date: body.endDate.clone(),
        serial_number: body.serialNumber.clone(),
        device_group_serial: "".to_string(),
    };

    let query_result = get_temperature_service.get_average_by_device(pool).await;

    let response = TemperatureDeviceResponse {
        msg: "success".to_string(),
        data: query_result,
    };
    let json_str = serde_json::to_string(&response).expect("Failed to serialize to JSON");

    Custom(Status::Ok, json_str)
}

/* **********************************************************************************/

#[post("/GetTemperature/Group", format = "json", data = "<body>")]
pub async fn get_temperature_group_controller(
    pool: &State<Pool<MySql>>,
    body: Json<TemperatureGroupRequest>,
) -> status::Custom<String> {
    let get_temperature_service = GetTemperature {
        start_date: body.startDate.clone(),
        end_date: body.endDate.clone(),
        serial_number: "".to_string(),
        device_group_serial: body.deviceGroupSerial.to_string(),
    };

    let query_result = get_temperature_service.get_average_by_group(pool).await;

    let response = TemperatureGroupResponse {
        msg: "success".to_string(),
        data: query_result,
    };
    let json_str = serde_json::to_string(&response).expect("Failed to serialize to JSON");

    Custom(Status::Ok, json_str)
}
