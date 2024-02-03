use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{post, State};
use sqlx::{MySql, Pool};

use crate::services::{CreateGroup, GetGroup};
use crate::structs::{GroupRegistRequest, GroupRegistResponse};

/**
 * 그룹등록
 */
#[post("/Regist/Group", format = "json", data = "<body>")]
pub async fn regist_group_controller(
    pool: &State<Pool<MySql>>,
    body: Json<GroupRegistRequest>,
) -> status::Custom<String> {
    let transaction = pool.begin().await.expect("Failed to start a transaction");
    let create_group_service = CreateGroup {
        g_snumber: body.deviceGroupSerial.clone(),
    };
    let insert_id = create_group_service.add(pool).await;
    transaction
        .commit()
        .await
        .expect("Failed to commit the transaction");

    let get_group_service = GetGroup {
        gid: insert_id,
        g_snumber: body.deviceGroupSerial.clone(),
    };

    let get_group_result = get_group_service.find_by_id(pool).await;
    let response = GroupRegistResponse {
        msg: "Success".to_string(),
        data: get_group_result,
    };
    let json_str = serde_json::to_string(&response).expect("Failed to serialize to JSON");

    Custom(Status::Ok, json_str)
}
