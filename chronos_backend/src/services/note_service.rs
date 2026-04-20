use sea_orm::*;
use chrono::Utc;
use crate::entities::task::{ActiveModel, Entity as TaskEntity, Model, Note};

pub async fn add_note(
    db: &DatabaseConnection,
    task_id: &str,
    content: String,
) -> Result<Model, String> {
    let task = TaskEntity::find_by_id(task_id.to_string())
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task not found: {}", task_id))?;

    let mut notes = task.parsed_notes();
    let now = Utc::now().to_rfc3339();

    notes.push(Note {
        id: format!("note_{}", Utc::now().timestamp_millis()),
        content,
        created_at: now.clone(),
        updated_at: now,
    });

    let mut active: ActiveModel = task.into();
    active.notes = Set(serde_json::to_string(&notes).map_err(|e| e.to_string())?);

    let result = active.update(db).await.map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn get_notes(
    db: &DatabaseConnection,
    task_id: &str,
) -> Result<Vec<Note>, String> {
    let task = TaskEntity::find_by_id(task_id.to_string())
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task not found: {}", task_id))?;

    Ok(task.parsed_notes())
}

pub async fn update_note(
    db: &DatabaseConnection,
    task_id: &str,
    note_id: &str,
    content: String,
) -> Result<Model, String> {
    let task = TaskEntity::find_by_id(task_id.to_string())
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task not found: {}", task_id))?;

    let mut notes = task.parsed_notes();
    let now = Utc::now().to_rfc3339();

    let note = notes
        .iter_mut()
        .find(|n| n.id == note_id)
        .ok_or_else(|| format!("Note not found: {}", note_id))?;

    note.content = content;
    note.updated_at = now;

    let mut active: ActiveModel = task.into();
    active.notes = Set(serde_json::to_string(&notes).map_err(|e| e.to_string())?);

    let result = active.update(db).await.map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn delete_note(
    db: &DatabaseConnection,
    task_id: &str,
    note_id: &str,
) -> Result<Model, String> {
    let task = TaskEntity::find_by_id(task_id.to_string())
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task not found: {}", task_id))?;

    let mut notes = task.parsed_notes();
    let original_len = notes.len();

    notes.retain(|n| n.id != note_id);

    if notes.len() == original_len {
        return Err(format!("Note not found: {}", note_id));
    }

    let mut active: ActiveModel = task.into();
    active.notes = Set(serde_json::to_string(&notes).map_err(|e| e.to_string())?);

    let result = active.update(db).await.map_err(|e| e.to_string())?;
    Ok(result)
}
