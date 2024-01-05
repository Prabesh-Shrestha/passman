
use sqlx::{sqlite::SqliteConnectOptions, SqliteConnection, ConnectOptions, Connection};

use digest::Digest;
use sha2::Sha256;
use rand::Rng;
use serde::{Deserialize, Serialize};
struct PasswordManager {
    db_conn: SqliteConnection,
}
impl PasswordManager {
    async fn new() -> Result<Self, sqlx::Error> {
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
    fn hash_password(&self, password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password);
        hasher.finalize().to_hex()
    }
    async fn add_account(&mut self, username: &str, password: &str) -> Result<(), sqlx::Error> {
        #[allow(unused)]
        let hashed_password = hash_password(password);
        #[allow(unused)]
        let _ = sqlx::query!(
            "INSERT OR REPLACE INTO accounts (username, password) VALUES (?, ?)",
            username,
            hashed_password
        )
        .execute(&self.db_conn)
        .await?;
        Ok(())
    }

}
