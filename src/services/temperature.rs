use crate::{
    function::{hex_to_decimal, string_to_arr},
    structs::TemperatureResponseBody,
};
use sqlx::MySqlPool;

pub struct SetTemperature {
    pub d_snumber: String,
    pub temperature: String,
    pub interval: u64,
    pub registered_at: String,
}

pub struct GetTemperature {
    pub start_date: String,
    pub end_date: String,
    pub serial_number: String,
    pub device_group_serial: String,
}

impl SetTemperature {
    pub async fn add(&self, pool: &MySqlPool) -> bool {
        let arr = string_to_arr(&self.temperature);
        let temp_arr = hex_to_decimal(arr);
        let mut transaction = pool.begin().await.expect("Failed to start a transaction");
        for (index, hex_str) in temp_arr.iter().enumerate() {
            let interval_time: i64 = ((index as u64) * &self.interval).try_into().unwrap();
            let interval_str = format!(
                "'{}' + INTERVAL {} SECOND",
                &self.registered_at, interval_time
            );
            let query = format!(
                "INSERT INTO Temperature (d_snumber, temperature, registered) VALUES ('{}', {}, {}); ",
                &self.d_snumber, hex_str, interval_str
            );

            let insert_query = sqlx::query(&query).execute(&mut transaction).await;

            if let Err(err) = insert_query {
                println!("SQLx Error: {:?}", err);
                transaction
                    .rollback()
                    .await
                    .expect("Failed to rollback the transaction");
                // return;
                panic!("Error: {:?}", err)
            }
        }
        transaction
            .commit()
            .await
            .expect("Failed to commit the transaction");
        true
    }
}

impl GetTemperature {
    pub async fn get_average_by_device(&self, pool: &MySqlPool) -> TemperatureResponseBody {
        let query = format!(
            "select d.did as id, t.d_snumber as serialNumber, t.averageTemperature
            FROM Device d left join 
            (SELECT d_snumber, ROUND(CAST(AVG(temperature) AS DOUBLE),1) as averageTemperature 
            FROM Temperature 
            WHERE registered BETWEEN ? AND ? 
            group by d_snumber) as t 
            on d.d_snumber = t.d_snumber 
            WHERE d.d_snumber = ?"
        );

        let avg_result = sqlx::query_as::<_, TemperatureResponseBody>(&query)
            .bind(&self.start_date)
            .bind(&self.end_date)
            .bind(&self.serial_number)
            .fetch_one(pool)
            .await;

        let query_result = match avg_result {
            Ok(result) => result,
            Err(err) => {
                println!("{:?}", err);
                TemperatureResponseBody {
                    id: 0,
                    serialNumber: "".to_string(),
                    averageTemperature: 0.0,
                }
            }
        };

        query_result
    }

    pub async fn get_average_by_group(&self, pool: &MySqlPool) -> Vec<TemperatureResponseBody> {
        let query = format!(
            "select d.did as id, t.d_snumber as serialNumber, t.averageTemperature
            FROM Device d left join 
            (SELECT d_snumber, ROUND(CAST(AVG(temperature) AS DOUBLE),1) as averageTemperature 
            FROM Temperature 
            WHERE registered BETWEEN ? AND ? 
            group by d_snumber) as t 
            on d.d_snumber = t.d_snumber 
            WHERE d.g_snumber = ?"
        );

        let avg_result = sqlx::query_as::<_, TemperatureResponseBody>(&query)
            .bind(&self.start_date)
            .bind(&self.end_date)
            .bind(&self.device_group_serial)
            .fetch_all(pool)
            .await;

        let query_result = match avg_result {
            Ok(result) => result,
            Err(err) => {
                println!("{:?}", err);
                Vec::new()
            }
        };

        query_result
    }
}
