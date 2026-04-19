# Notes System for Tasks

## Overview

Chronos now includes a comprehensive notes system that allows you to add, edit, and delete notes for each task. Notes are perfect for tracking progress, documenting decisions, logging issues, or adding context to your work.

## Features

- **✅ Add Notes**: Attach multiple notes to any task
- **✏️ Edit Notes**: Update note content inline
- **🗑️ Delete Notes**: Remove notes with confirmation
- **📅 Timestamps**: Track when notes were created and last updated
- **📝 Rich Text**: Supports multi-line notes with whitespace preservation
- **🔄 Real-time Sync**: Notes update immediately across the UI
- **💾 Persistent Storage**: Stored in SQLite as JSON

## Architecture

### Backend Components

#### 1. Task Entity (Database Schema)
**File**: `chronos_backend/src/entities/task.rs`

**New Fields:**
```rust
pub struct Model {
    // ... existing fields
    pub notes: NoteList,  // JSON column storing notes array
}

pub struct Note {
    pub id: String,           // Unique identifier
    pub content: String,      // Note text
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

#### 2. Note Service
**File**: `chronos_backend/src/services/note_service.rs`

**Functions:**
- `add_note(db, task_id, input)` - Create new note
- `get_notes(db, task_id)` - Retrieve all notes for a task
- `update_note(db, task_id, note_id, input)` - Update note content
- `delete_note(db, task_id, note_id)` - Delete a note

#### 3. Note Commands
**File**: `chronos_backend/src/commands/notes.rs`

Tauri command wrappers for all note operations.

#### 4. Tauri Commands
**File**: `chronos/src-tauri/src/lib.rs`

Exposed commands:
- `add_note_command`
- `get_notes_command`
- `update_note_command`
- `delete_note_command`

### Frontend Components

#### UI Location
Notes appear in the **Edit Task Modal** at the bottom, below the task form.

#### UI Elements
1. **Notes Section Header** - Title + Add Note button
2. **Add Note Input** - Textarea with Save/Cancel buttons (hidden by default)
3. **Notes List** - All notes for the current task, sorted newest first
4. **Note Cards** - Individual notes with edit/delete buttons

## Usage Guide

### Adding a Note

1. **Click on any task** to open the Edit Task modal
2. **Scroll to the Notes section** at the bottom
3. **Click "+ Add Note"** button
4. **Type your note** in the textarea
5. **Click "Save"** (or Cancel to discard)

Example:
```
┌──────────────────────────────────────────┐
│ Edit Execution Task                       │
│ [Task form fields...]                     │
│                                           │
│ ─── NOTES ────────────────────────────── │
│                     [+ Add Note]         │
│                                           │
│ ┌────────────────────────────────────┐  │
│ │ Write your note here...            │  │
│ │                                    │  │
│ └────────────────────────────────────┘  │
│ [Save] [Cancel]                          │
└──────────────────────────────────────────┘
```

### Viewing Notes

Notes are displayed automatically when you open a task for editing:

```
┌──────────────────────────────────────────┐
│ ─── NOTES ────────────────────────────── │
│                     [+ Add Note]         │
│                                           │
│ ┌────────────────────────────────────┐  │
│ │ 4/19/2026, 10:30:00 PM   [✏️] [🗑️] │  │
│ │                                    │  │
│ │ Fixed the bug in timer calculation │  │
│ │ Need to test edge cases            │  │
│ └────────────────────────────────────┘  │
│                                           │
│ ┌────────────────────────────────────┐  │
│ │ 4/19/2026, 10:15:00 PM   [✏️] [🗑️] │  │
│ │                                    │  │
│ │ Started working on the feature     │  │
│ └────────────────────────────────────┘  │
└──────────────────────────────────────────┘
```

### Editing a Note

1. **Click the edit icon (✏️)** on any note
2. **Textarea appears** with current content
3. **Modify the text**
4. **Click "Save"** to update (or "Cancel" to discard changes)

The note will show "(edited: timestamp)" after the creation date if it's been modified.

### Deleting a Note

1. **Click the delete icon (🗑️)** on any note
2. **Confirm deletion** in the popup dialog
3. **Note is removed** immediately

## Data Model

### Note Structure

```rust
{
  "id": "note_1713567890123",
  "content": "This is my note content",
  "created_at": "2026-04-19T22:30:00Z",
  "updated_at": "2026-04-19T22:30:00Z"
}
```

### Task Storage

Notes are stored as a JSON array in the `notes` column of the `tasks` table:

```json
{
  "id": "task_1713567890000",
  "title": "Implement Notes System",
  "notes": [
    {
      "id": "note_1713567890123",
      "content": "First note",
      "created_at": "2026-04-19T22:30:00Z",
      "updated_at": "2026-04-19T22:30:00Z"
    },
    {
      "id": "note_1713567891234",
      "content": "Second note",
      "created_at": "2026-04-19T22:31:00Z",
      "updated_at": "2026-04-19T22:31:00Z"
    }
  ]
}
```

## API Reference

### Backend (Rust)

```rust
// Add a note
let task = add_note(
    &db,
    "task_123",
    CreateNoteInput { content: "My note".to_string() }
).await?;

// Get all notes
let notes = get_notes(&db, "task_123").await?;

// Update a note
let task = update_note(
    &db,
    "task_123",
    "note_456",
    UpdateNoteInput { content: "Updated content".to_string() }
).await?;

// Delete a note
let task = delete_note(&db, "task_123", "note_456").await?;
```

### Frontend (JavaScript)

```javascript
// Add a note
await invoke("add_note_command", {
    taskId: "task_123",
    content: "My note"
});

// Get notes
const notes = await invoke("get_notes_command", {
    taskId: "task_123"
});

// Update a note
await invoke("update_note_command", {
    taskId: "task_123",
    noteId: "note_456",
    content: "Updated content"
});

// Delete a note
await invoke("delete_note_command", {
    taskId: "task_123",
    noteId: "note_456"
});
```

## Use Cases

### 1. Progress Tracking
```
Note 1: "Started implementing authentication module"
Note 2: "Finished login page, working on registration"
Note 3: "Registration complete, need to add validation"
```

### 2. Bug Documentation
```
Note 1: "Bug found: Timer doesn't stop when window is minimized"
Note 2: "Root cause: Event listener not handling visibility change"
Note 3: "Fixed: Added document.addEventListener('visibilitychange')"
```

### 3. Decision Log
```
Note 1: "Considering SQLite vs PostgreSQL for storage"
Note 2: "Decision: Going with SQLite for local-first approach"
Note 3: "Implementation plan: Use SeaORM with SQLite backend"
```

### 4. Context Preservation
```
Note 1: "Working on user authentication"
Note 2: "Paused to fix critical timer bug"
Note 3: "Resuming authentication work - left off at JWT validation"
```

### 5. Meeting Notes
```
Note 1: "Team meeting 4/19: Discussed new features"
Note 2: "Action items: Add notes system, improve timer accuracy"
Note 3: "Next meeting: Review progress and plan mobile app"
```

## Styling

Notes UI follows the Chronos cyber-lime/terminal aesthetic:

- **Background**: Dark grey (#0a0a0a)
- **Borders**: Surface container (#353534)
- **Timestamps**: Muted text
- **Edit Button**: Cyber-lime (#CCFF00)
- **Delete Button**: Error red
- **Text**: Monospace font for timestamps, body font for content

## Performance

- **Storage**: Efficient JSON storage in SQLite
- **Loading**: Notes load only when task is opened
- **Rendering**: Sorted client-side (newest first)
- **Memory**: Minimal impact (~1KB per 10 notes)

## Database Migration

The notes system requires a new column in the `tasks` table:

```sql
ALTER TABLE tasks ADD COLUMN notes TEXT DEFAULT '[]';
```

For existing tasks, the notes field is automatically initialized to an empty array `[]` when first accessed.

## Keyboard Shortcuts (Future Enhancement)

Potential shortcuts to add:
- `Ctrl+N` - Add new note
- `Ctrl+S` - Save note edit
- `Esc` - Cancel note edit
- `Delete` - Delete note (with confirmation)

## Mobile Compatibility

The notes system works on mobile with:
- Touch-friendly buttons
- Responsive textarea
- Swipe-to-delete (future)
- Haptic feedback on delete (future)

## Security Considerations

- **Input Sanitization**: HTML escaping prevents XSS
- **SQL Injection**: Protected by SeaORM parameterized queries
- **Data Privacy**: All notes stored locally in SQLite
- **No External Sync**: Notes never leave your device

## Limitations

- **No Rich Text**: Currently plain text only (no markdown, formatting)
- **No Attachments**: Can't attach files or images
- **No Search**: No global note search (yet)
- **No Tags**: Can't tag or categorize notes
- **No Export**: Can't export notes separately

## Future Enhancements

### Short Term
- [ ] Markdown support for rich formatting
- [ ] Note search within task
- [ ] Note count badge on tasks
- [ ] Keyboard shortcuts

### Medium Term
- [ ] Global note search across all tasks
- [ ] Note tags and categories
- [ ] Note templates
- [ ] Note attachments (files/images)

### Long Term
- [ ] Voice-to-text notes
- [ ] AI-powered note summarization
- [ ] Shared notes (team collaboration)
- [ ] Note version history

## Troubleshooting

### Notes not saving

**Symptom**: Click Save but note doesn't appear

**Solutions**:
1. Check browser console for errors
2. Verify Chronos app is running
3. Check database file permissions
4. Try restarting the app

### Notes not loading

**Symptom**: Open task but notes section is empty

**Solutions**:
1. Check if notes were actually added
2. Verify database connection
3. Check console for errors
4. Try adding a new note

### Edit/Delete buttons not working

**Symptom**: Click buttons but nothing happens

**Solutions**:
1. Check JavaScript console for errors
2. Verify modal is fully loaded
3. Try closing and reopening the modal
4. Check if currentTaskId is set

## Testing

### Manual Test Cases

#### Test 1: Add Note
1. Open a task for editing
2. Click "+ Add Note"
3. Type "Test note"
4. Click Save
5. ✅ Verify note appears in list

#### Test 2: Edit Note
1. Open task with existing note
2. Click edit icon (✏️)
3. Modify text
4. Click Save
5. ✅ Verify note shows updated content and "(edited)" timestamp

#### Test 3: Delete Note
1. Open task with existing note
2. Click delete icon (🗑️)
3. Confirm deletion
4. ✅ Verify note is removed from list

#### Test 4: Multiple Notes
1. Add 3 notes to a task
2. ✅ Verify notes appear in reverse chronological order (newest first)
3. Edit the middle note
4. ✅ Verify edited note stays in correct position

#### Test 5: Empty Note Validation
1. Click "+ Add Note"
2. Leave textarea empty
3. Click Save
4. ✅ Verify error message: "Note content cannot be empty"

---

**Implementation Date**: 2026-04-19
**Version**: 1.0
**Status**: Production Ready
**Dependencies**: SeaORM, Chrono, Serde
