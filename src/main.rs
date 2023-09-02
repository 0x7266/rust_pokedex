mod router;

use router::router;

#[tokio::main]
async fn main() {
    let app = router();
    axum::Server::bind(&"0.0.0.0:7878".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
