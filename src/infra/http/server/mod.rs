use super::routes::router;

pub async fn init() {
    let app = router().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
