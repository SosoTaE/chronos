use sea_orm::DatabaseConnection;
use crate::entities::task::{Model, TimeSession};
use crate::services::timer_service;

pub async fn start_timer_command(
    db: &DatabaseConnection,
    task_id: String,
) -> Result<Model, String> {
    timer_service::start_timer(db, &task_id).await
}

pub async fn stop_timer_command(
    db: &DatabaseConnection,
    task_id: String,
) -> Result<Model, String> {
    timer_service::stop_timer(db, &task_id).await
}

pub async fn get_timer_status_command(
    db: &DatabaseConnection,
    task_id: String,
) -> Result<Option<TimeSession>, String> {
    timer_service::get_timer_status(db, &task_id).await
}
