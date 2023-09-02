use axum::{routing::get, Router};

use crate::handlers::{get::all_pokemons, index};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index()))
        .route("/pokemon", get(all_pokemons()))
}
