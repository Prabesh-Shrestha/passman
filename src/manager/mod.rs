#![allow(unused_variables, unused_imports, dead_code)]

use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Connection, SqliteConnection};

// use serde::{Deserialize, Serialize};
pub struct PasswordManager {
    db_conn: SqliteConnection,
}
impl PasswordManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let mut db_conn = SqliteConnectOptions::new()
            .filename("passwords.db")
            .create_if_missing(true)
            .connect()
            .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS accounts (username TEXT PRIMARY KEY, password TEXT)",
        )
        .execute(&mut db_conn)
        .await?;
        Ok(PasswordManager { db_conn })
    }
    // async fn add_account(&mut self, username: &str, password: &str) -> Result<(), sqlx::Error> {
    //     #[allow(unused)]
    //     let hashed_password = self.hash_password(password);
    //     #[allow(unused)]
    //     let _ = sqlx::query!(
    //         "INSERT OR REPLACE INTO accounts (username, password) VALUES (?, ?)",
    //         username,
    //         hashed_password
    //     )
    //     .execute(&self.db_conn)
    //     .await?;
    //     Ok(())
    // }
}
