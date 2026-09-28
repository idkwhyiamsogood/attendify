use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::model::enums::group_status::GroupStatus;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub specialty_id: Uuid, // FK to table(specialties)
    pub course: i16,        // 1..=6

    pub curator_id: Option<Uuid>, // FK to table(staff)
    pub status: GroupStatus,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::specialties::Entity",
        from = "Column::SpecialtyId",
        to = "super::specialties::Column::Id"
    )]
    Specialties,
    #[sea_orm(
        belongs_to = "super::staff::Entity",
        from = "Column::CuratorId",
        to = "super::staff::Column::Id"
    )]
    Curator,
    #[sea_orm(has_many = "super::students::Entity")]
    Students,
}

impl Related<super::specialties::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Specialties.def()
    }
}

impl Related<super::staff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Curator.def()
    }
}

impl Related<super::students::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Students.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
