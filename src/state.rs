use sqlx::PgPool;
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub redis: redis::Client,
}

impl AppState {
    pub async fn init() -> Self {
        let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
        let redis = redis::Client::open(std::env::var("REDIS_URL").unwrap()).unwrap();
        let jwt_secret = std::env::var("JWT_SECRET").unwrap();
        Self { db, jwt_secret, redis }
    }
}