use sea_orm::DatabaseConnection;
use crate::entities::task::{Model, Note};
use crate::services::note_service;

pub async fn add_note_command(
    db: &DatabaseConnection,
    task_id: String,
    content: String,
) -> Result<Model, String> {
    note_service::add_note(db, &task_id, content).await
}

pub async fn get_notes_command(
    db: &DatabaseConnection,
    task_id: String,
) -> Result<Vec<Note>, String> {
    note_service::get_notes(db, &task_id).await
}

pub async fn update_note_command(
    db: &DatabaseConnection,
    task_id: String,
    note_id: String,
    content: String,
) -> Result<Model, String> {
    note_service::update_note(db, &task_id, &note_id, content).await
}

pub async fn delete_note_command(
    db: &DatabaseConnection,
    task_id: String,
    note_id: String,
) -> Result<Model, String> {
    note_service::delete_note(db, &task_id, &note_id).await
}
