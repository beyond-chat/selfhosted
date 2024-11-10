use anyhow::Context;
use axum_api::{
    http,
    utils::{db::Db, init, telemetry},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init_tracing();

    tracing::debug!("Initializing configuration");
    let env_vars = init::EnvVarsConfig::new();
    init::toml_data_paths()?;

    tracing::debug!("Initializing db pool");
    let db = Db::new(&env_vars.db_dsn, env_vars.db_pool_max_size)
        .await
        .context("Failed to initialize db pool")?;
    tracing::debug!("Running migrations");
    db.migrate().await.context("Failed to run migrations")?;

    tracing::info!("Starting backend server on port 5000");
    http::serve(db.pool).await;

    Ok(())
}
