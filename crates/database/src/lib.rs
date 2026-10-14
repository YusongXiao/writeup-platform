pub mod writeup;

use anyhow::Result;
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::str::FromStr;

pub async fn connect(dsn: &str) -> Result<PgPool> {
    // Parse DSN then force SSL off — musl static binary can't do SSL negotiation
    let opts = PgConnectOptions::from_str(dsn)?.ssl_mode(PgSslMode::Disable);
    let pool = PgPool::connect_with(opts).await?;
    Ok(pool)
}

pub async fn migrate(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("src/migrations")
        .run(pool)
        .await?;
    Ok(())
}
