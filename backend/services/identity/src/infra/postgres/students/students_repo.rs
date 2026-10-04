use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::{NotSet, Set};
use uuid::Uuid;

use crate::infra::postgres::none_if_not_found;
use crate::model::dto::students::{CreateStudentDto, UpdateStudentDto};
use crate::model::entities::students::{
    ActiveModel, Column, Entity as Student, Model as StudentModel,
};

use super::students_repo_interface::IStudentsRepo;

pub struct StudentsRepo {
    db: DatabaseConnection,
}

impl StudentsRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl IStudentsRepo for StudentsRepo {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<StudentModel>, DbErr> {
        Student::find_by_id(id).one(&self.db).await
    }

    async fn get_by_group_id(&self, group_id: Uuid) -> Result<Vec<StudentModel>, DbErr> {
        Student::find()
            .filter(Column::GroupId.eq(group_id))
            .all(&self.db)
            .await
    }

    async fn create(&self, student: CreateStudentDto) -> Result<StudentModel, DbErr> {
        ActiveModel {
            id: Set(Uuid::now_v7()),
            last_name: Set(student.last_name),
            first_name: Set(student.first_name),
            middle_name: Set(student.middle_name),
            group_id: Set(student.group_id),
            status: Set(student.status),

            ..Default::default()
        }
        .insert(&self.db)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        student: UpdateStudentDto,
    ) -> Result<Option<StudentModel>, DbErr> {
        let active_model = ActiveModel {
            id: Set(id),
            last_name: student.last_name.map_or(NotSet, Set),
            first_name: student.first_name.map_or(NotSet, Set),
            middle_name: student.middle_name.map_or(NotSet, Set),
            group_id: student.group_id.map_or(NotSet, Set),
            status: student.status.map_or(NotSet, Set),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        none_if_not_found(active_model.update(&self.db).await)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        let res = Student::delete_by_id(id).exec(&self.db).await?;
        Ok(res.rows_affected > 0)
    }
}
