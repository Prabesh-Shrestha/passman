use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Connection, SqliteConnection};

// use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
pub struct PasswordManager {
    db_conn: SqliteConnection,
}
impl PasswordManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let mut db_conn = SqliteConnectOptions::new()
            .filename("passwords.db")
            .connect()
            .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS accounts (username TEXT PRIMARY KEY, password TEXT)",
        )
        .execute(&mut db_conn)
        .await?;
        Ok(PasswordManager { db_conn })
    }
    pub fn hash_password(&self, password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update("String data");
        String::from_utf8_lossy(&hasher.finalize()[..]).to_string()
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
