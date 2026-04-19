use chronos_lib::{init_db, commands};
use sea_orm::DatabaseConnection;
use chronos_lib::entities::task::{Model as TaskModel, TaskCategory, TaskStatus, TimeSession, Note};
use chronos_lib::services::ai_service::ChatMessage;
use tauri::{Emitter, State};
use tauri_plugin_notification::NotificationExt;
use std::sync::Arc;
use std::time::Duration;

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

#[tauri::command]
async fn add_note_command(db: State<'_, DatabaseConnection>, task_id: String, content: String) -> Result<TaskModel, String> {
    commands::notes::add_note_command(&*db, task_id, content).await
}

#[tauri::command]
async fn get_notes_command(db: State<'_, DatabaseConnection>, task_id: String) -> Result<Vec<Note>, String> {
    commands::notes::get_notes_command(&*db, task_id).await
}

#[tauri::command]
async fn update_note_command(db: State<'_, DatabaseConnection>, task_id: String, note_id: String, content: String) -> Result<TaskModel, String> {
    commands::notes::update_note_command(&*db, task_id, note_id, content).await
}

#[tauri::command]
async fn delete_note_command(db: State<'_, DatabaseConnection>, task_id: String, note_id: String) -> Result<TaskModel, String> {
    commands::notes::delete_note_command(&*db, task_id, note_id).await
}

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Prevent WebKit/Wayland protocol errors
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Set up proper app data directory for the database to ensure persistence
            let mut db_path = app.path().app_data_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            std::fs::create_dir_all(&db_path).unwrap_or(());
            db_path.push("chronos.db");

            std::env::set_var("CHRONOS_DB_PATH", db_path.to_str().unwrap());

            let db = tauri::async_runtime::block_on(async {
                init_db().await.expect("Failed to initialize database")
            });

            // Start the process monitor daemon
            let db_arc = Arc::new(db.clone());
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                println!("[Process Monitor] Starting background daemon (60s interval)");
                let mut interval = tokio::time::interval(Duration::from_secs(60));

                loop {
                    interval.tick().await;

                    match chronos_lib::services::process_monitor::check_and_generate_nudge(&db_arc).await {
                        Ok(Some(nudge)) => {
                            println!(
                                "[Process Monitor] Detected processes: {:?} - Sending nudge",
                                nudge.detected_processes
                            );

                            // Send system notification
                            let process_list = nudge.detected_processes.join(", ");
                            if let Err(e) = app_handle.notification()
                                .builder()
                                .title("Chronos - Process Detected")
                                .body(format!("{}\n\nDetected: {}", nudge.message, process_list))
                                .show() {
                                eprintln!("[Process Monitor] Failed to show notification: {}", e);
                            }

                            // Also emit event for in-app notification
                            if let Err(e) = app_handle.emit("process-nudge", &nudge) {
                                eprintln!("[Process Monitor] Failed to emit event: {}", e);
                            }
                        }
                        Ok(None) => {
                            // No nudge needed (either no processes or timer is active)
                        }
                        Err(e) => {
                            eprintln!("[Process Monitor] Error: {}", e);
                        }
                    }
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
            add_note_command,
            get_notes_command,
            update_note_command,
            delete_note_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
