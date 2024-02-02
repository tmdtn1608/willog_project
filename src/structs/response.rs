// Response body 구조에 대해 정의

use serde::Serialize;
use sqlx::FromRow;

/* Regist/Group Response */
#[derive(Serialize)]
pub struct GroupRegistResponse {
    pub msg: String,
    pub data: DeviceGroupBody,
}

#[derive(FromRow, Serialize)]
pub struct DeviceGroupBody {
    pub deviceGroupId: i64,
    pub serialNumber: String,
    pub createdAt: String,
}

/* Regist/Device Response */
#[derive(Serialize)]
pub struct DeviceRegistResponse {
    pub msg: String,
    pub data: DataResponseBody,
}

#[derive(Serialize)]
pub struct DataResponseBody {
    pub deviceId: u64,
    pub serialNumber: String,
    pub deviceGroup: DeviceGroupBody,
    pub createdAt: String,
}

/* Set/Temperature Response */
#[derive(Serialize)]
pub struct TemperaturRegistResponse {
    pub msg: String,
}

/* GetTemperature/Device Response */
#[derive(Serialize)]
pub struct TemperatureDeviceResponse {
    pub msg: String,
    pub data: TemperatureResponseBody,
}

#[derive(FromRow, Serialize)]
pub struct TemperatureResponseBody {
    pub id: i64,                 // did
    pub serialNumber: String,    // d_snumber
    pub averageTemperature: f64, // c 연산값
}

/* GetTemperature/Group Response */
#[derive(Serialize)]
pub struct TemperatureGroupResponse {
    pub msg: String,
    pub data: Vec<TemperatureResponseBody>,
}
