use sqlx::{Error, Pool, Sqlite, SqlitePool};

pub async fn connect() -> Result<Pool<Sqlite>, Error> {
    let url = std::env::var("DATABASE_URL")
        .ok()
        .unwrap_or("sqlite://database.sqlite".to_string());
    SqlitePool::connect(&url).await
}
