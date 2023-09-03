use sqlx::{Error, Pool, Sqlite, SqlitePool};

pub async fn connect() -> Result<Pool<Sqlite>, Error> {
    let url = std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string());
    SqlitePool::connect(&url).await
}
