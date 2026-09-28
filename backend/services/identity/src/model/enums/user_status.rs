use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_status")]
pub enum UserStatus {
    #[sea_orm(string_value = "pending_activation")]
    PendingActivation,

    #[sea_orm(string_value = "active")]
    Active,

    #[sea_orm(string_value = "blocked")]
    Blocked,

    #[sea_orm(string_value = "archived")]
    Archive,
}
