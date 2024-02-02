use crate::structs::DeviceGroupBody;
use sqlx::MySqlPool;

pub struct CreateDevice {
    pub d_snumber: String,
    pub g_snumber: String,
}

pub struct GetDevice {
    pub did: String,
}

impl CreateDevice {
    pub async fn add(&self, pool: &MySqlPool) -> u64 {
        let query = format!("INSERT INTO Device (d_snumber, g_snumber) VALUES (?, ?)");
        let insert_query = sqlx::query(&query)
            .bind(&self.d_snumber)
            .bind(&self.g_snumber)
            .execute(pool)
            .await;

        let inserted_id = match insert_query {
            Ok(result) => result.last_insert_id(),
            Err(err) => panic!("Failed to fetch data: {}", err),
        };
        inserted_id
    }
}

impl GetDevice {
    pub async fn find_by_id(&self, pool: &MySqlPool) -> String {
        let query = format!("SELECT did as deviceGroupId, d_snumber, g_snumber as serialNumber, DATE_FORMAT(created, '%Y-%m-%dT%H:%i:%s') as createdAt FROM Device WHERE did = ?");
        // 다시조회.
        let recheck = sqlx::query_as::<_, DeviceGroupBody>(&query)
            .bind(&self.did)
            .fetch_one(pool)
            .await;
        let date = match recheck {
            Ok(result) => result.createdAt,
            Err(err) => panic!("Failed to fetch data: {}", err),
        };
        date
    }
}
