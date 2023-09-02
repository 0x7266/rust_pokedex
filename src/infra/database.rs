use sqlx::{Pool, Sqlite, SqlitePool};

pub async fn connect() -> Result<Pool<Sqlite>, anyhow::Error> {
    let url = std::env::var("DATABASE_URL")
        .ok()
        .unwrap_or("sqlite://database.sqlite".to_string());
    Ok(SqlitePool::connect(&url).await?)
}
