// Request body 구조에 대해 정의

use rocket::form::FromForm;
use serde::Deserialize;

/* Regist/Group Request */
#[derive(Debug, Deserialize, FromForm)]
pub struct GroupRegistRequest {
    pub deviceGroupSerial: String,
}

/* Regist/Device Reqeust */
#[derive(Debug, Deserialize, FromForm)]
pub struct DeviceRegistRequest {
    pub serialNumber: String,
    pub deviceGroupSerial: String,
}

/* Set/Temperature Request */
#[derive(Debug, Deserialize, FromForm)]
pub struct TemperaturRegistRequest {
    pub serialNumber: String,
    pub interval: u64,
    pub temperatures: String,
    pub registered_at: String,
}

/* GetTemperature/Device Request */

#[derive(Debug, Deserialize, FromForm)]
pub struct TemperatureDeviceRequest {
    pub serialNumber: String,
    pub startDate: String,
    pub endDate: String,
}

/* GetTemperature/Group Request */

#[derive(Debug, Deserialize, FromForm)]
pub struct TemperatureGroupRequest {
    pub deviceGroupSerial: String,
    pub startDate: String,
    pub endDate: String,
}
