use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::model::enums::{user_kind::UserKind, user_status::UserStatus};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "devices")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub kind: UserKind,
    pub status: UserStatus,

    pub student_id: Option<Uuid>,
    pub staff_id: Option<Uuid>,

    pub login: String,
    pub password_hash: Option<String>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,
}

// TODO
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let ok = match self.kind.as_ref() {
            UserKind::Student => {
                self.student_id.as_ref().is_some() && self.staff_id.as_ref().is_none()
            }
            UserKind::Staff | UserKind::Admin => {
                self.staff_id.as_ref().is_some() && self.student_id.as_ref().is_none()
            }
        };

        if !ok {
            return Err(DbErr::Custom(
                "users: student_id/staff_id должны соответствовать kind".into(),
            ));
        };

        return Ok(self);
    }
}
