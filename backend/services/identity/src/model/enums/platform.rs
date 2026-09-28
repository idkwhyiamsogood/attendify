use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "platform")]
pub enum Platform {
    #[sea_orm(string_value = "android")]
    Android,

    #[sea_orm(string_value = "ios")]
    Ios,
}
