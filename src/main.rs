#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let state = AppState::init().await;
    sqlx::migrate!().run(&state.db).await.unwrap();

    let app = Router::new()
        .nest("/auth", routes::auth::router())
        .nest("/payments", routes::payments::router())
        .nest("/ai", routes::ai::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);
    
        let listener = tokio::net::TcpListener::bind("0.0.0.0:7070").await.unwrap();
        axum::serve(listener, app).await.unwrap();
}