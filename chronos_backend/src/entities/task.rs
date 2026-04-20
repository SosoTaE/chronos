use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize, Serializer};

// ── Enums ──

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskCategory {
    Work,
    Business,
    Coding,
    Personal,
    Health,
}

impl std::fmt::Display for TaskCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Work => write!(f, "Work"),
            Self::Business => write!(f, "Business"),
            Self::Coding => write!(f, "Coding"),
            Self::Personal => write!(f, "Personal"),
            Self::Health => write!(f, "Health"),
        }
    }
}

impl std::str::FromStr for TaskCategory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Work" => Ok(Self::Work),
            "Business" => Ok(Self::Business),
            "Coding" => Ok(Self::Coding),
            "Personal" => Ok(Self::Personal),
            "Health" => Ok(Self::Health),
            other => Err(format!("Unknown category: {}", other)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Paused,
    Completed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Todo => write!(f, "Todo"),
            Self::InProgress => write!(f, "InProgress"),
            Self::Paused => write!(f, "Paused"),
            Self::Completed => write!(f, "Completed"),
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Todo" => Ok(Self::Todo),
            "InProgress" => Ok(Self::InProgress),
            "Paused" => Ok(Self::Paused),
            "Completed" => Ok(Self::Completed),
            other => Err(format!("Unknown status: {}", other)),
        }
    }
}

// ── Nested types ──

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSession {
    pub start_time: String,
    pub end_time: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

// Wrapper for Vec<Note> so we can use it as a serde alias
pub type NoteList = Vec<Note>;

// ── SeaORM Entity ──

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub category: String,
    #[sea_orm(column_type = "Text")]
    pub status: String,
    #[sea_orm(column_type = "Text")]
    pub created_at: String,
    pub estimated_duration_mins: i32,
    pub actual_duration_secs: i64,
    #[sea_orm(column_type = "Text")]
    pub sessions: String,
    #[sea_orm(column_type = "Text")]
    pub notes: String,
}

// Custom Serialize so that category/status serialize as their enum names
// and sessions/notes serialize as JSON arrays instead of raw strings.
impl Serialize for Model {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Model", 10)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("category", &self.category)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("estimated_duration_mins", &self.estimated_duration_mins)?;
        state.serialize_field("actual_duration_secs", &self.actual_duration_secs)?;

        let sessions: Vec<TimeSession> =
            serde_json::from_str(&self.sessions).unwrap_or_default();
        state.serialize_field("sessions", &sessions)?;

        let notes: Vec<Note> =
            serde_json::from_str(&self.notes).unwrap_or_default();
        state.serialize_field("notes", &notes)?;

        state.end()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// ── Helpers ──

impl Model {
    pub fn parsed_sessions(&self) -> Vec<TimeSession> {
        serde_json::from_str(&self.sessions).unwrap_or_default()
    }

    pub fn parsed_notes(&self) -> Vec<Note> {
        serde_json::from_str(&self.notes).unwrap_or_default()
    }

    pub fn category_enum(&self) -> TaskCategory {
        self.category.parse().unwrap_or(TaskCategory::Work)
    }

    pub fn status_enum(&self) -> TaskStatus {
        self.status.parse().unwrap_or(TaskStatus::Todo)
    }
}
