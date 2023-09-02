use axum::{routing::get, Router};

use crate::handlers::index;

pub fn router() -> Router {
    Router::new().route("/", get(index()))
}
