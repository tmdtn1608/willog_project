// connection 관리

use sqlx::MySqlPool;

pub async fn db_connect() -> Result<MySqlPool, sqlx::Error> {
    let db_url = "mysql://root@localhost:3306/willog";
    MySqlPool::connect(db_url).await
}
