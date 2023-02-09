// relational database manager (rdb)
// uses MYSQL as the backend
// uses the sqlx crate

use sqlx::MySql;
use sqlx;

// new connention to kasten database
pub async fn new_connection() -> sqlx::MySqlConnection {
    let conn = sqlx::MySqlConnection::connect("mysql://root:root@localhost/kasten").await.unwrap();
    conn
}
