use chronos_lib::{init_db, commands};
use sea_orm::DatabaseConnection;
use chronos_lib::entities::task::{Model as TaskModel, TaskCategory, TaskStatus, TimeSession};
use chronos_lib::services::ai_service::ChatMessage;
use tauri::State;
use std::sync::Arc;

mod webhook_server;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_task_command(
    db: State<'_, DatabaseConnection>,
    title: String,
    description: Option<String>,
    category: TaskCategory,
    estimated_duration_mins: u32,
) -> Result<TaskModel, String> {
    commands::tasks::create_task_command(&*db, title, description, category, estimated_duration_mins).await
}

#[tauri::command]
async fn get_all_tasks_command(db: State<'_, DatabaseConnection>) -> Result<Vec<TaskModel>, String> {
    commands::tasks::get_all_tasks_command(&*db).await
}

#[tauri::command]
async fn get_task_command(db: State<'_, DatabaseConnection>, task_id: String) -> Result<TaskModel, String> {
    commands::tasks::get_task_command(&*db, task_id).await
}

#[tauri::command]
async fn update_task_command(
    db: State<'_, DatabaseConnection>,
    task_id: String,
    title: Option<String>,
    description: Option<String>,
    category: Option<TaskCategory>,
    status: Option<TaskStatus>,
    estimated_duration_mins: Option<u32>,
) -> Result<TaskModel, String> {
    commands::tasks::update_task_command(&*db, task_id, title, description, category, status, estimated_duration_mins).await
}

#[tauri::command]
async fn delete_task_command(db: State<'_, DatabaseConnection>, task_id: String) -> Result<(), String> {
    commands::tasks::delete_task_command(&*db, task_id).await
}

#[tauri::command]
async fn get_tasks_by_status_command(db: State<'_, DatabaseConnection>, status: TaskStatus) -> Result<Vec<TaskModel>, String> {
    commands::tasks::get_tasks_by_status_command(&*db, status).await
}

#[tauri::command]
async fn get_tasks_by_category_command(db: State<'_, DatabaseConnection>, category: TaskCategory) -> Result<Vec<TaskModel>, String> {
    commands::tasks::get_tasks_by_category_command(&*db, category).await
}

#[tauri::command]
async fn start_timer_command(db: State<'_, DatabaseConnection>, task_id: String) -> Result<TaskModel, String> {
    commands::timer::start_timer_command(&*db, task_id).await
}

#[tauri::command]
async fn stop_timer_command(db: State<'_, DatabaseConnection>, task_id: String) -> Result<TaskModel, String> {
    commands::timer::stop_timer_command(&*db, task_id).await
}

#[tauri::command]
async fn get_timer_status_command(db: State<'_, DatabaseConnection>, task_id: String) -> Result<Option<TimeSession>, String> {
    commands::timer::get_timer_status_command(&*db, task_id).await
}

#[tauri::command]
async fn analyze_with_local_ai_command(db: State<'_, DatabaseConnection>, task_id: String) -> Result<String, String> {
    commands::ai::analyze_with_local_ai_command(&*db, task_id).await
}

#[tauri::command]
async fn check_ollama_health_command() -> Result<bool, String> {
    commands::ai::check_ollama_health_command().await
}

#[tauri::command]
async fn analyze_achievements_command(db: State<'_, DatabaseConnection>) -> Result<String, String> {
    commands::ai::analyze_achievements_command(&*db).await
}

#[tauri::command]
async fn chat_with_ai_command(history: Vec<ChatMessage>) -> Result<ChatMessage, String> {
    commands::ai::chat_with_ai_command(history).await
}

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Prevent WebKit/Wayland protocol errors
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Set up proper app data directory for the database to ensure persistence
            let mut db_path = app.path().app_data_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            std::fs::create_dir_all(&db_path).unwrap_or(());
            db_path.push("chronos.db");

            std::env::set_var("CHRONOS_DB_PATH", db_path.to_str().unwrap());

            let db = tauri::async_runtime::block_on(async {
                init_db().await.expect("Failed to initialize database")
            });

            // Start the webhook server in the background
            let db_arc = Arc::new(db.clone());
            tauri::async_runtime::spawn(async move {
                if let Err(e) = webhook_server::start_webhook_server(db_arc).await {
                    eprintln!("[Tauri] Failed to start webhook server: {}", e);
                } else {
                    println!("[Tauri] Webhook server started successfully");
                }
            });

            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            create_task_command,
            get_all_tasks_command,
            get_task_command,
            update_task_command,
            delete_task_command,
            get_tasks_by_status_command,
            get_tasks_by_category_command,
            start_timer_command,
            stop_timer_command,
            get_timer_status_command,
            analyze_with_local_ai_command,
            check_ollama_health_command,
            analyze_achievements_command,
            chat_with_ai_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
