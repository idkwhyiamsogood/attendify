use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::model::enums::staff_status::StaffStatus;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "staff")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    // ПДн, шифровано AES-256-GCM
    pub last_name: Vec<u8>,
    pub first_name: Vec<u8>,
    pub middle_name: Option<Vec<u8>>,
    pub last_name_bidx: Vec<u8>,

    pub status: StaffStatus,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::users::Entity")]
    Users,
    #[sea_orm(has_many = "super::groups::Entity")]
    Groups,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Groups.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
