use super::routes::router;

pub async fn init() {
    let app = router().await;
    axum::Server::bind(&"0.0.0.0:7878".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
