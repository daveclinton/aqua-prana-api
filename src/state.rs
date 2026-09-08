#[derive(Clone)]
pub struct AppState(
    pub db: PgPool,
    pub redis: redis::Client,
    pub jwt_secret: String,
    pub ai_client: async_openai::Client<async_openai::config::OpenAIConfig>,
)