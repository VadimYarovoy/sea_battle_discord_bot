use color_eyre::Report;
use figment::{
    providers::{Env, Format, Toml},
    Figment, Provider,
};
use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;
use tracing::instrument;

#[derive(Debug, PartialEq, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub max_connections: u32,
}

impl DbConfig {
    pub fn sqlx_config(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .application_name("sea_battle_bot")
            .host(self.host.as_ref())
            .port(self.port)
            .username(self.user.as_ref())
            .password(self.password.as_ref())
            .database(self.database.as_ref())
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AppConfig {
    pub db: DbConfig,
}

impl AppConfig {
    pub fn figment() -> Figment {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::prefixed("APP_"))
    }

    pub fn from<T: Provider>(provider: T) -> Result<Self, Report> {
        Ok(Figment::from(provider).extract()?)
    }
}

#[instrument]
pub fn tracing_subscriber() -> Result<impl tracing::Subscriber, Report> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;
    use tracing_tree::HierarchicalLayer;

    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    let subscriber = tracing_subscriber::Registry::default()
        .with(EnvFilter::from_default_env())
        .with(
            HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .with(ErrorLayer::default());
    Ok(subscriber)
}

