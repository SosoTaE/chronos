use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use sysinfo::System;
use chrono::Utc;

use crate::services::timer_service;
use crate::services::ai_service;

const MONITORED_PROCESSES: &[&str] = &[
    "nvim", "vim", "code", "emacs",
    "cargo", "rustc", "npm", "node", "make", "gcc", "g++",
    "alacritty", "wezterm", "kitty",
    "python", "rust-analyzer",
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessNudge {
    pub message: String,
    pub detected_processes: Vec<String>,
    pub timestamp: String,
}

pub fn check_developer_processes() -> Vec<String> {
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let mut detected: Vec<String> = Vec::new();

    for process in sys.processes().values() {
        let name = process.name().to_string_lossy().to_lowercase();
        for &monitored in MONITORED_PROCESSES {
            if name.contains(monitored) && !detected.contains(&monitored.to_string()) {
                detected.push(monitored.to_string());
            }
        }
    }

    detected
}

pub async fn check_and_generate_nudge(
    db: &DatabaseConnection,
) -> Result<Option<ProcessNudge>, String> {
    let detected = check_developer_processes();

    if detected.is_empty() {
        return Ok(None);
    }

    // Check if any timer is currently active
    let has_timer = timer_service::has_active_timer(db).await?;
    if has_timer {
        return Ok(None);
    }

    // Generate AI nudge message (with fallback)
    let message = match ai_service::generate_nudge_message(&detected).await {
        Ok(msg) => msg,
        Err(_) => {
            let process_list = detected.join(" and ");
            format!(
                "I see you're using {}. Start a timer to track your progress!",
                process_list
            )
        }
    };

    Ok(Some(ProcessNudge {
        message,
        detected_processes: detected,
        timestamp: Utc::now().to_rfc3339(),
    }))
}
