use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// Подключение к БД с настройками, одинаковыми для всех сервисов.
pub async fn connect(database_url: &str) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.max_connections(10)
        .acquire_timeout(Duration::from_secs(5));
    Ok(Database::connect(opt).await?)
}