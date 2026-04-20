use sea_orm::*;
use chrono::Utc;
use crate::entities::task::{self, ActiveModel, Entity as TaskEntity, Model, TaskCategory, TaskStatus};

fn generate_id() -> String {
    let now = Utc::now();
    format!("task_{}", now.timestamp_millis())
}

pub async fn create_task(
    db: &DatabaseConnection,
    title: String,
    description: Option<String>,
    category: TaskCategory,
    estimated_duration_mins: u32,
) -> Result<Model, String> {
    let id = generate_id();
    let now = Utc::now().to_rfc3339();

    let model = ActiveModel {
        id: Set(id),
        title: Set(title),
        description: Set(description),
        category: Set(category.to_string()),
        status: Set(TaskStatus::Todo.to_string()),
        created_at: Set(now),
        estimated_duration_mins: Set(estimated_duration_mins as i32),
        actual_duration_secs: Set(0),
        sessions: Set("[]".to_string()),
        notes: Set("[]".to_string()),
    };

    let result = model.insert(db).await.map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn get_all_tasks(db: &DatabaseConnection) -> Result<Vec<Model>, String> {
    TaskEntity::find()
        .all(db)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_task(db: &DatabaseConnection, task_id: &str) -> Result<Model, String> {
    TaskEntity::find_by_id(task_id.to_string())
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task not found: {}", task_id))
}

pub async fn update_task(
    db: &DatabaseConnection,
    task_id: &str,
    title: Option<String>,
    description: Option<String>,
    category: Option<TaskCategory>,
    status: Option<TaskStatus>,
    estimated_duration_mins: Option<u32>,
) -> Result<Model, String> {
    let task = get_task(db, task_id).await?;

    let mut active: ActiveModel = task.into();

    if let Some(t) = title {
        active.title = Set(t);
    }
    if let Some(d) = description {
        active.description = Set(Some(d));
    }
    if let Some(c) = category {
        active.category = Set(c.to_string());
    }
    if let Some(s) = status {
        active.status = Set(s.to_string());
    }
    if let Some(e) = estimated_duration_mins {
        active.estimated_duration_mins = Set(e as i32);
    }

    let result = active.update(db).await.map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn delete_task(db: &DatabaseConnection, task_id: &str) -> Result<(), String> {
    let task = get_task(db, task_id).await?;
    let active: ActiveModel = task.into();
    active.delete(db).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_tasks_by_status(
    db: &DatabaseConnection,
    status: TaskStatus,
) -> Result<Vec<Model>, String> {
    TaskEntity::find()
        .filter(task::Column::Status.eq(status.to_string()))
        .all(db)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_tasks_by_category(
    db: &DatabaseConnection,
    category: TaskCategory,
) -> Result<Vec<Model>, String> {
    TaskEntity::find()
        .filter(task::Column::Category.eq(category.to_string()))
        .all(db)
        .await
        .map_err(|e| e.to_string())
}
