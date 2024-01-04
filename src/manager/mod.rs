
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteConnectOptions, SqliteConnection};
struct PasswordManager {
    db_conn: SqliteConnection,
}
impl PasswordManager {
    async fn new() -> Result<Self, sqlx::Error> {
        let db_conn = SqliteConnectOptions::new()
            .filename("passwords.db")
            .connect()
            .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS accounts (username TEXT PRIMARY KEY, password TEXT)",
        )
        .execute(&db_conn)
        .await?;
        Ok(PasswordManager { db_conn })
    }
}
