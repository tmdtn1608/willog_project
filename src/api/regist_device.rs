use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{post, State};
use sqlx::{MySql, Pool};

use crate::services::{CreateDevice, GetDevice, GetGroup};
use crate::structs::{DataResponseBody, DeviceRegistRequest, DeviceRegistResponse};

#[post("/Regist/Device", format = "json", data = "<body>")]
pub async fn regist_device_controller(
    pool: &State<Pool<MySql>>,
    body: Json<DeviceRegistRequest>,
) -> status::Custom<String> {
    let transaction = pool.begin().await.expect("Failed to start a transaction");
    let get_group_service = GetGroup {
        gid: "".to_string(),
        g_snumber: body.deviceGroupSerial.clone(),
    };

    let get_group_result = get_group_service.find_by_serial(pool).await;

    let create_device_service = CreateDevice {
        d_snumber: body.serialNumber.clone(),
        g_snumber: body.deviceGroupSerial.clone(),
    };

    let inserted_id = create_device_service.add(pool).await;

    transaction
        .commit()
        .await
        .expect("Failed to commit the transaction");

    let get_device_service = GetDevice {
        did: inserted_id.to_string(),
    };

    let date = get_device_service.find_by_id(pool).await;

    let device_data = DataResponseBody {
        deviceId: inserted_id,
        serialNumber: body.serialNumber.clone(),
        deviceGroup: get_group_result,
        createdAt: date,
    };

    let response = DeviceRegistResponse {
        msg: "success".to_string(),
        data: device_data,
    };

    let json_str = serde_json::to_string(&response).expect("Failed to serialize to JSON");

    Custom(Status::Ok, json_str)
}
