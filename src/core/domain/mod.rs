use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: i64,
    pub name: String,
    // pub types: Vec<Type>,
    pub height: i64,
    pub weight: i64,
    // pub abilities: Vec<String>,
    pub gif: String,
    pub image: String,
}

// #[derive(FromRow, Serialize)]
// pub struct Type {
//     pub name: String,
//     pub url: String,
// }
