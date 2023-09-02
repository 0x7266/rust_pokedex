use axum::{routing::get, Router};

use crate::{
    handlers::{get::all_pokemons, index},
    infra::database,
};

pub async fn router() -> Router {
    let connection = database::connect()
        .await
        .expect("Failed to connect to the database");
    // let connection_state = std::sync::Arc::new(connection);
    Router::new()
        .route("/", get(index))
        .route("/pokemon", get(all_pokemons))
        .with_state(connection)
}
