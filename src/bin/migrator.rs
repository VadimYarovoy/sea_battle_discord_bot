use color_eyre::Report;
use sqlx::{postgres::PgPoolOptions, ConnectOptions};
use tracing_subscriber::util::SubscriberInitExt;

use sea_battle_discord_bot::config::{AppConfig, tracing_subscriber};

#[tokio::main]
async fn main() -> Result<(), Report> {
    tracing_subscriber()?.init();
    let config = AppConfig::from(AppConfig::figment())?;
    let pool = PgPoolOptions::new()
        .max_connections(config.db.max_connections)
        .connect_with(config.db.sqlx_config())
        .await?;
    sqlx::migrate!()
        .run(&pool)
        .await?;
    Ok(())
}

