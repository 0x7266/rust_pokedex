use axum::{extract::Path, extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, SqlitePool};

use crate::core::domain::Pokemon;

#[derive(FromRow, Serialize, Deserialize)]
pub struct PokemonQuery {
    pub id: Option<i64>,
    pub name: Option<String>,
}

pub async fn index() -> String {
    "Working on it\nneed a Rust professor btw 8)".to_string()
}

// pub async fn query(
//     State(pool): State<SqlitePool>,
//     Query(pokemon): Query<PokemonQuery>,
// ) -> Json<Pokemon> {
//     if let Some(id) = pokemon.id {
//         let pokemon: Pokemon = query_as("SELECT * FROM pokemon WHERE id = ($1)")
//             .bind(id)
//             .fetch_one(&pool)
//             .await
//             .unwrap();
//         Json(pokemon)
//     } else {
//         Json(Pokemon {
//             gif: "".to_string(),
//             height: 23,
//             id: 3333,
//             image: "".to_string(),
//             name: "pokemon".to_string(),
//             weight: 34,
//         })
//     }
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
