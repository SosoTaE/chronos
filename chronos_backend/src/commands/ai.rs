use sea_orm::DatabaseConnection;
use crate::services::{ai_service, task_service};
use crate::services::ai_service::ChatMessage;
use crate::entities::task::TaskStatus;

pub async fn analyze_with_local_ai_command(
    db: &DatabaseConnection,
    task_id: String,
) -> Result<String, String> {
    let task = task_service::get_task(db, &task_id).await?;
    ai_service::analyze_task(
        &task.title,
        &task.category,
        task.estimated_duration_mins,
        task.actual_duration_secs,
    )
    .await
}

pub async fn check_ollama_health_command() -> Result<bool, String> {
    ai_service::check_health().await
}

pub async fn analyze_achievements_command(
    db: &DatabaseConnection,
) -> Result<String, String> {
    let tasks = task_service::get_tasks_by_status(db, TaskStatus::Completed).await?;

    if tasks.is_empty() {
        return Ok("No completed tasks to analyze.".to_string());
    }

    let summary = tasks
        .iter()
        .map(|t| {
            format!(
                "- {} (Category: {}, Est: {}m, Actual: {}m)",
                t.title,
                t.category,
                t.estimated_duration_mins,
                t.actual_duration_secs / 60
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    ai_service::analyze_achievements(&summary).await
}

pub async fn chat_with_ai_command(
    history: Vec<ChatMessage>,
) -> Result<ChatMessage, String> {
    ai_service::chat(history).await
}
