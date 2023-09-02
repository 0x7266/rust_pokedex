use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, SqlitePool};

#[derive(Debug, Deserialize, FromRow, Serialize)]
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

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Types {
    pub slot: i64,
    #[serde(rename = "type")]
    pub type_field: Type,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Type {
    pub name: String,
    pub url: String,
}

pub async fn all_pokemons(State(pool): State<SqlitePool>) -> Json<Vec<Pokemon>> {
    let pokemons: Vec<Pokemon> = query_as("SELECT * FROM pokemon")
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("{:?}", pokemons);
    Json(pokemons)
}
