use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BaseConfig {
    pub service_name: String,
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    pub database_url: String,
}

fn default_http_port() -> u16 {
    8080
}

/// Собирает источники (toml < env), но НЕ извлекает конкретный тип —
/// это делает каждый сервис сам, добавляя свои поля через #[serde(flatten)].
pub fn figment() -> Figment {
    Figment::new()
        .merge(Toml::file("config/default.toml"))
        .merge(Env::raw())
}