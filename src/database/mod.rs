use sqlx::sqlite::SqlitePool;


pub enum DatabaseType {
    SQLITE,
    POSTGRE,
    SQLSERVER,
}
pub struct Database {
    url: String,
    db_type: DatabaseType,
}

impl Database {
    pub fn new(url: String) -> Database {
        Database {
            url,
            db_type: DatabaseType::SQLITE,
        }
    }

    pub async fn connect(&self) -> {
        match &self.db_type {
            DatabaseType::SQLITE => return SqlitePool::connect("sqlite::memory:").await?,
            DatabaseType::POSTGRE => (),
            DatabaseType::SQLSERVER => (),
        }
    }
}
