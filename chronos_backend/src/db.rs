use sea_orm::{Database, DatabaseConnection, ConnectionTrait};

pub async fn init_db() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db_path = std::env::var("CHRONOS_DB_PATH")
        .unwrap_or_else(|_| "chronos.db".to_string());
    let db_url = format!("sqlite:{}?mode=rwc", db_path);

    let db = Database::connect(&db_url).await?;

    // Enable WAL mode for better concurrent read performance
    db.execute_unprepared("PRAGMA journal_mode=WAL;").await?;

    // Create tasks table
    db.execute_unprepared(
        "CREATE TABLE IF NOT EXISTS tasks (
            id              TEXT PRIMARY KEY NOT NULL,
            title           TEXT NOT NULL,
            description     TEXT,
            category        TEXT NOT NULL DEFAULT 'Work',
            status          TEXT NOT NULL DEFAULT 'Todo',
            created_at      TEXT NOT NULL,
            estimated_duration_mins INTEGER NOT NULL DEFAULT 30,
            actual_duration_secs    INTEGER NOT NULL DEFAULT 0,
            sessions        TEXT NOT NULL DEFAULT '[]',
            notes           TEXT NOT NULL DEFAULT '[]'
        );"
    ).await?;

    Ok(db)
}
