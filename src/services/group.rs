use crate::structs::DeviceGroupBody;
use sqlx::MySqlPool;

/**
 * 그룹생성 구조체
 */
pub struct CreateGroup {
    pub g_snumber: String,
}

/**
 * 그룹조회 구조체
 */
pub struct GetGroup {
    pub g_snumber: String,
    pub gid: String,
}

impl CreateGroup {
    pub async fn add(&self, pool: &MySqlPool) -> String {
        let query = format!("INSERT INTO DeviceGroup (g_snumber) VALUES (?)");
        let rows_result = sqlx::query(&query)
            .bind(&self.g_snumber)
            .execute(pool)
            .await;

        let inserted_id = match rows_result {
            Ok(result) => result.last_insert_id(),
            Err(err) => {
                panic!("Failed to fetch data: {}", err);
            }
        };
        inserted_id.to_string()
    }
}

impl GetGroup {
    pub async fn find_by_id(&self, pool: &MySqlPool) -> DeviceGroupBody {
        let query = format!("SELECT gid as deviceGroupId, g_snumber as serialNumber, DATE_FORMAT(created, '%Y-%m-%dT%H:%i:%s') as createdAt FROM DeviceGroup WHERE gid = ?");
        let recheck = sqlx::query_as::<_, DeviceGroupBody>(&query)
            .bind(&self.gid)
            .fetch_one(pool)
            .await;

        let select: DeviceGroupBody = match recheck {
            Ok(result) => result,
            Err(err) => panic!("Failed to fetch data: {}", err),
        };
        select
    }

    pub async fn find_by_serial(&self, pool: &MySqlPool) -> DeviceGroupBody {
        let query = format!("SELECT gid as deviceGroupId, g_snumber as serialNumber, DATE_FORMAT(created, '%Y-%m-%dT%H:%i:%s') as createdAt FROM DeviceGroup WHERE g_snumber = ?");
        let group_info = sqlx::query_as::<_, DeviceGroupBody>(&query)
            .bind(&self.g_snumber)
            .fetch_one(pool)
            .await;

        let device_group = match group_info {
            Ok(result) => result,
            Err(err) => panic!("Failed to fetch data: {}", err),
        };
        device_group
    }
}
