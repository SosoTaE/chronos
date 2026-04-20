use sea_orm::DatabaseConnection;
use crate::entities::task::{Model, TaskCategory, TaskStatus};
use crate::services::task_service;

pub async fn create_task_command(
    db: &DatabaseConnection,
    title: String,
    description: Option<String>,
    category: TaskCategory,
    estimated_duration_mins: u32,
) -> Result<Model, String> {
    task_service::create_task(db, title, description, category, estimated_duration_mins).await
}

pub async fn get_all_tasks_command(
    db: &DatabaseConnection,
) -> Result<Vec<Model>, String> {
    task_service::get_all_tasks(db).await
}

pub async fn get_task_command(
    db: &DatabaseConnection,
    task_id: String,
) -> Result<Model, String> {
    task_service::get_task(db, &task_id).await
}

pub async fn update_task_command(
    db: &DatabaseConnection,
    task_id: String,
    title: Option<String>,
    description: Option<String>,
    category: Option<TaskCategory>,
    status: Option<TaskStatus>,
    estimated_duration_mins: Option<u32>,
) -> Result<Model, String> {
    task_service::update_task(db, &task_id, title, description, category, status, estimated_duration_mins).await
}

pub async fn delete_task_command(
    db: &DatabaseConnection,
    task_id: String,
) -> Result<(), String> {
    task_service::delete_task(db, &task_id).await
}

pub async fn get_tasks_by_status_command(
    db: &DatabaseConnection,
    status: TaskStatus,
) -> Result<Vec<Model>, String> {
    task_service::get_tasks_by_status(db, status).await
}

pub async fn get_tasks_by_category_command(
    db: &DatabaseConnection,
    category: TaskCategory,
) -> Result<Vec<Model>, String> {
    task_service::get_tasks_by_category(db, category).await
}
