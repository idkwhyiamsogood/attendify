use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::entities::specialties::Model as SpecialtyModel;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSpecialtyDto {
    pub name: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSpecialtyDto {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialtyResponseDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SpecialtyModel> for SpecialtyResponseDto {
    fn from(m: SpecialtyModel) -> Self {
        Self {
            id: m.id,
            name: m.name,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}
