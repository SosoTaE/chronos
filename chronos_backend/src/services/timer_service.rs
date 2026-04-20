use sea_orm::*;
use chrono::Utc;
use crate::entities::task::{self, ActiveModel, Entity as TaskEntity, Model, TimeSession, TaskStatus};

pub async fn start_timer(
    db: &DatabaseConnection,
    task_id: &str,
) -> Result<Model, String> {
    let task = TaskEntity::find_by_id(task_id.to_string())
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task not found: {}", task_id))?;

    let mut sessions = task.parsed_sessions();

    // Check if there's already a running session
    if sessions.iter().any(|s| s.end_time.is_none()) {
        return Err("Timer already running for this task".to_string());
    }

    let now = Utc::now().to_rfc3339();
    sessions.push(TimeSession {
        start_time: now,
        end_time: None,
    });

    let mut active: ActiveModel = task.into();
    active.sessions = Set(serde_json::to_string(&sessions).map_err(|e| e.to_string())?);
    active.status = Set(TaskStatus::InProgress.to_string());

    let result = active.update(db).await.map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn stop_timer(
    db: &DatabaseConnection,
    task_id: &str,
) -> Result<Model, String> {
    let task = TaskEntity::find_by_id(task_id.to_string())
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task not found: {}", task_id))?;

    let mut sessions = task.parsed_sessions();
    let now = Utc::now().to_rfc3339();

    // Find and close the open session
    let mut session_duration: i64 = 0;
    for session in sessions.iter_mut() {
        if session.end_time.is_none() {
            session.end_time = Some(now.clone());

            // Calculate this session's duration
            if let Ok(start) = chrono::DateTime::parse_from_rfc3339(&session.start_time) {
                if let Ok(end) = chrono::DateTime::parse_from_rfc3339(&now) {
                    session_duration = (end - start).num_seconds();
                }
            }
        }
    }

    let new_actual = task.actual_duration_secs + session_duration;

    let mut active: ActiveModel = task.into();
    active.sessions = Set(serde_json::to_string(&sessions).map_err(|e| e.to_string())?);
    active.actual_duration_secs = Set(new_actual);
    active.status = Set(TaskStatus::Paused.to_string());

    let result = active.update(db).await.map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn get_timer_status(
    db: &DatabaseConnection,
    task_id: &str,
) -> Result<Option<TimeSession>, String> {
    let task = TaskEntity::find_by_id(task_id.to_string())
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task not found: {}", task_id))?;

    let sessions = task.parsed_sessions();

    // Return the currently running session (one with no end_time)
    let active_session = sessions.into_iter().find(|s| s.end_time.is_none());
    Ok(active_session)
}

pub async fn has_active_timer(db: &DatabaseConnection) -> Result<bool, String> {
    let tasks = TaskEntity::find()
        .filter(task::Column::Status.eq(TaskStatus::InProgress.to_string()))
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(!tasks.is_empty())
}
