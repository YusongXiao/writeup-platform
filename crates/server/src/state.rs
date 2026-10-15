use anyhow::Result;
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
    pub r2s_db: PgPool,
    pub config: config::Config,
    pub http: reqwest::Client,
}

impl AppState {
    pub async fn new(cfg: config::Config) -> Result<Self> {
        tracing::info!("connecting to own db: postgresql://{}:***@{}:{}/{}", cfg.database.user, cfg.database.host, cfg.database.port, cfg.database.db);
        let db = database::connect(&cfg.database).await?;
        database::migrate(&db).await?;
        tracing::info!("own database connected and migrated");

        let r2s_db = database::connect(&cfg.r2s_database).await?;
        tracing::info!("ret2shell database connected (read-only)");

        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self { db, r2s_db, config: cfg, http })
    }
}
