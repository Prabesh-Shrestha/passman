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
    pub fn new(&self, url: String) -> Database {
        Database {
            url,
            db_type: DatabaseType::SQLITE,
        }
    }
    
    pub fn 
}
