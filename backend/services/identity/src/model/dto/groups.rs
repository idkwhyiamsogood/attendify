use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::dto::double_option;
use crate::model::entities::groups::Model as GroupModel;
use crate::model::enums::group_status::GroupStatus;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupDto {
    pub code: String,
    pub specialty_id: Uuid,
    pub course: i16,
    pub curator_id: Option<Uuid>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGroupDto {
    pub code: Option<String>,
    pub specialty_id: Option<Uuid>,
    pub course: Option<i16>,
    /// `null` снимает куратора
    #[serde(default, deserialize_with = "double_option")]
    pub curator_id: Option<Option<Uuid>>,
    pub status: Option<GroupStatus>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponseDto {
    pub id: Uuid,
    pub code: String,
    pub specialty_id: Uuid,
    pub course: i16,
    pub curator_id: Option<Uuid>,
    pub status: GroupStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<GroupModel> for GroupResponseDto {
    fn from(m: GroupModel) -> Self {
        Self {
            id: m.id,
            code: m.code,
            specialty_id: m.specialty_id,
            course: m.course,
            curator_id: m.curator_id,
            status: m.status,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}
