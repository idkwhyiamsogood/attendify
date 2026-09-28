use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "device_status")]
pub enum DeviceStatus {
    #[sea_orm(string_value = "active")]
    Active,

    #[sea_orm(string_value = "reset")]
    Reset,

    #[sea_orm(string_value = "revoked")]
    Revoked,
}
