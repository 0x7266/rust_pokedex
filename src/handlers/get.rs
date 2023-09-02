use axum::{extract::Path, extract::State, Json};
use serde::Serialize;
use sqlx::{query_as, FromRow, SqlitePool};

#[derive(FromRow, Serialize)]
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

pub async fn all_pokemons(State(pool): State<SqlitePool>) -> Json<Vec<Pokemon>> {
    let pokemons: Vec<Pokemon> = query_as("SELECT * FROM pokemon")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(pokemons)
}

pub async fn by_id(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Json<Pokemon> {
    let pokemon = query_as("SELECT * FROM Pokemon WHERE id = ($1)")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(pokemon)
}
