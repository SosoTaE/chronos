# Chronos - Project Analysis

## Overview

**Chronos** is a cross-platform desktop application that combines task management, time tracking, and local AI assistance in a terminal-inspired interface. Built with Tauri and Rust, it emphasizes privacy (local-only AI), simplicity, and performance.

## Project Details

- **Project Name:** Chronos (internally "lifetime")
- **Type:** Cross-platform Desktop Application (Linux, Windows, macOS)
- **License:** GNU General Public License v3.0
- **Current State:** Early development (recently initialized)

## Tech Stack

### Frontend
- **UI Framework:** Vanilla JavaScript (no framework dependencies)
- **Desktop Framework:** Tauri 2.x WebView
- **Styling:**
  - TailwindCSS 3.x (CDN)
  - Custom dark theme with cyberpunk/terminal aesthetic
  - Google Fonts: Space Grotesk, Inter
  - Material Symbols icons
- **Build:** None required (files served directly from `/src`)

### Backend
- **Language:** Rust (Edition 2021)
- **Framework:** Tauri 2.x
- **ORM:** SeaORM 2.0 (Release Candidate)
- **Database:** SQLite with WAL (Write-Ahead Logging)
- **Async Runtime:** Tokio (full features)
- **HTTP Client:** Reqwest 0.12
- **Serialization:** Serde with JSON support
- **Date/Time:** Chrono 0.4

### AI Integration
- **Platform:** Ollama (local inference)
- **Models:** Gemma 4, Qwen3:14b (configurable)
- **Purpose:** Task analysis, productivity insights, chat interface

## Architecture

The project follows a modular architecture with clear separation:

```
chronos/                          # Tauri Desktop App
├── src/                          # Frontend (Vanilla JS)
│   ├── index.html                # Main UI (380 lines)
│   ├── main.js                   # App logic (567 lines)
│   ├── styles.css                # Base styles
│   └── assets/                   # SVG icons
│
├── src-tauri/                    # Rust Backend Wrapper
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   └── lib.rs                # Tauri commands bridge
│   ├── Cargo.toml                # Dependencies
│   ├── tauri.conf.json           # App configuration
│   ├── icons/                    # Platform icons
│   └── capabilities/             # Permissions

chronos_backend/                  # Separate Rust Library
├── src/
│   ├── lib.rs                    # Library exports
│   ├── db.rs                     # Database init
│   ├── entities/
│   │   └── task.rs               # SeaORM models
│   ├── services/
│   │   ├── task_service.rs       # Task CRUD
│   │   ├── timer_service.rs      # Time tracking
│   │   └── ai_service.rs         # Ollama integration
│   └── commands/
│       ├── tasks.rs              # Task handlers
│       ├── timer.rs              # Timer handlers
│       └── ai.rs                 # AI handlers
└── Cargo.toml
```

## Key Features

### 1. Task Management
- Create tasks with title, description, category, and estimated duration
- **Categories:** Work, Business, Coding, Personal, Health
- **Status tracking:** Todo, InProgress, Paused, Completed
- Full CRUD operations with filtering
- Click-to-edit interface

### 2. Time Tracking
- Start/Stop timers for tasks
- Multiple work sessions per task
- Automatic duration calculation
- Real-time progress visualization
- Session persistence in database
- Live timer updates (every second)

### 3. AI Features (Local-First)
- **AI Pulse:** Automatic periodic advice during active tasks
- **Chat interface:** Direct conversation with local AI
- **Achievement analysis:** AI insights on completed tasks
- **Privacy-first:** All processing happens locally via Ollama
- No cloud services or API costs

### 4. User Interface
- **Terminal View:** Active and pending tasks with live timer
- **Archive View:** Completed task history
- **Metrics View:** Statistics dashboard (tasks, duration, efficiency)
- **Chat View:** AI conversation interface
- **Visual Design:** Dark theme with cyber-lime accents, terminal aesthetic

## Data Models

### Task Entity
```rust
{
  id: String,                    // ULID-based unique identifier
  title: String,                 // Task title
  description: Option<String>,   // Optional description
  category: TaskCategory,        // Work|Business|Coding|Personal|Health
  status: TaskStatus,            // Todo|InProgress|Paused|Completed
  created_at: DateTime<Utc>,     // Creation timestamp
  estimated_duration_mins: u32,  // Estimated time in minutes
  actual_duration_secs: i64,     // Actual tracked time in seconds
  sessions: Vec<TimeSession>     // Array of time tracking sessions
}
```

### TimeSession
```rust
{
  start_time: DateTime<Utc>,
  end_time: Option<DateTime<Utc>>  // None if timer is running
}
```

## Entry Points

1. **Frontend:** `src/index.html` → `src/main.js`
   - HTML structure with TailwindCSS and Material Icons
   - JavaScript handles state management and UI updates
   - Communicates with backend via Tauri invoke commands

2. **Rust Backend:** `src-tauri/src/main.rs` → `src-tauri/src/lib.rs`
   - Initializes database on app startup
   - Registers all Tauri command handlers
   - Manages application state (DatabaseConnection)

3. **Backend Library:** `chronos_backend/src/lib.rs`
   - Exports public API for Tauri integration
   - Contains all business logic separated from UI layer

## Key Dependencies

### Tauri App (src-tauri/Cargo.toml)
```toml
tauri = "2"
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
chronos_lib = { package = "chronos", path = "../../chronos_backend" }
sea-orm = { version = "2.0.0-rc", features = ["sqlx-sqlite", "runtime-tokio-rustls", "macros"] }
```

### Backend Library (chronos_backend/Cargo.toml)
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sea-orm = { version = "2.0.0-rc", features = ["sqlx-sqlite", "runtime-tokio-rustls", "macros"] }
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
log = "0.4"
```

## Development Setup

### Prerequisites
- **Rust:** 1.70+ (install via [rustup](https://rustup.rs/))
- **SQLite:** 3.x
- **Node.js:** Not required (no build process for frontend)
- **Ollama:** Optional, for AI features ([ollama.ai](https://ollama.ai))

### Development Commands

```bash
# Navigate to Tauri app
cd chronos/src-tauri

# Build the application
cargo build

# Run in development mode with hot reload
cargo tauri dev

# Build for production (optimized)
cargo tauri build

# Test backend library
cd ../../chronos_backend
cargo test
cargo run
```

### Database Setup
- Database file created automatically at app data directory
- Managed by Tauri's `app.path().app_data_dir()`
- File: `chronos.db`
- Environment variable: `CHRONOS_DB_PATH` (optional override)
- SQLite with WAL mode for concurrent reads

### AI Setup (Optional)

```bash
# Install Ollama from ollama.ai

# Pull a model (choose one)
ollama pull gemma:4b
ollama pull qwen3:14b

# Start Ollama service
ollama serve
```

## Configuration

### tauri.conf.json
- **Product name:** "lifetime"
- **Identifier:** "com.sosotae.lifetime"
- **Default window:** 800x600
- **Frontend directory:** "../src"

### Cargo.toml (Tauri)
- **Package name:** "lifetime"
- **Backend path:** Relative path to `chronos_backend`

### Cargo.toml (Backend)
- **Package name:** "chronos"
- **Library name:** "chronos_lib"
- **Crate type:** Library

## Notable Technical Decisions

1. **Vanilla JavaScript** - No framework overhead, direct browser APIs, faster load times
2. **Separate backend library** - Clean separation of concerns, reusable business logic
3. **SQLite with WAL** - Optimized for desktop use, concurrent reads, single-file database
4. **Local AI via Ollama** - Privacy-first, no API costs, works offline
5. **File-based modules** - Modern Rust project structure with clear responsibilities
6. **JSON columns in SQLite** - Flexible storage for sessions and enums
7. **TailwindCSS via CDN** - No build process needed, rapid prototyping
8. **Material Design icons** - Consistent, professional iconography
9. **Tauri 2.x** - Smaller binary size, better security, modern WebView

## Project Purpose

Chronos addresses the need for a **privacy-focused, offline-capable productivity tool** that combines:
- Task management with realistic time tracking
- AI-powered insights without cloud dependencies
- A distraction-free, terminal-inspired interface
- Cross-platform desktop support with native performance

It's ideal for developers and professionals who value:
- **Privacy:** All data stays local
- **Simplicity:** No complex setup or subscriptions
- **Performance:** Rust backend with efficient SQLite storage
- **Aesthetics:** Terminal/cyberpunk theme for focused work

## Current Status

Based on git status:
- **Branch:** main
- **Recent commits:** "first init", "Initial commit"
- **Modified files:** src-tauri/Cargo.toml

The project is in **early development stages** with core architecture established and basic functionality implemented.

## Future Development Potential

- Task dependencies and subtasks
- Calendar integration
- Pomodoro timer integration
- Export functionality (CSV, JSON)
- Custom themes and color schemes
- Plugin system for extensibility
- Mobile companion app (read-only sync)
- Enhanced AI features (task suggestions, pattern recognition)
- Multi-language support
- Cloud sync option (optional, end-to-end encrypted)

---

**Generated:** 2026-04-19
**Analyzed by:** Claude Code (Sonnet 4.5)
