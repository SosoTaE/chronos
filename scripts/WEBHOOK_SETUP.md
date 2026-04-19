# Git Webhook Integration for Chronos

## Overview

Chronos now includes a local HTTP webhook server that accepts Git commit payloads. This allows automatic logging of your commits to the currently active task, creating a seamless integration between your coding workflow and time tracking.

## Features

- **Automatic commit logging**: Every commit is automatically appended to your active task
- **Local-only**: No external services, all data stays on your machine (port 3030)
- **Smart task detection**: Commits are logged to:
  1. Currently active task (with running timer)
  2. Most recently paused task (if no timer is active)
- **Non-intrusive**: Silent failures if Chronos isn't running, won't interrupt your git workflow

## Architecture

### Webhook Server
- **Framework**: Axum (Rust async web framework)
- **Port**: 3030 (localhost only)
- **Endpoint**: `POST /api/webhooks/git`
- **Health check**: `GET /health`

### Payload Format
```json
{
  "commit_message": "Your commit message",
  "repo_name": "repository-name",
  "author": "Your Name"
}
```

### Response Format
```json
{
  "success": true,
  "message": "Commit logged to task: Task Title",
  "task_id": "task_1234567890"
}
```

## Installation

### Option 1: Single Repository

Install the hook in a specific Git repository:

```bash
# Navigate to your repository
cd /path/to/your/repo

# Copy the hook script
cp /path/to/chronos/scripts/post-commit.sample .git/hooks/post-commit

# Make it executable
chmod +x .git/hooks/post-commit
```

### Option 2: Global Installation (Recommended)

Install the hook for all new and existing repositories:

```bash
# Create Git templates directory
mkdir -p ~/.git-templates/hooks

# Copy the hook script
cp /path/to/chronos/scripts/post-commit.sample ~/.git-templates/hooks/post-commit

# Make it executable
chmod +x ~/.git-templates/hooks/post-commit

# Configure Git to use the template
git config --global init.templatedir '~/.git-templates'

# Apply to existing repositories (run in each repo)
cd /path/to/existing/repo
git init
```

### Option 3: Arch Linux System-Wide

For system-wide installation on Arch Linux:

```bash
# Copy to system Git templates
sudo mkdir -p /usr/share/git-core/templates/hooks
sudo cp scripts/post-commit.sample /usr/share/git-core/templates/hooks/post-commit
sudo chmod +x /usr/share/git-core/templates/hooks/post-commit
```

## Usage

### Basic Workflow

1. **Start Chronos app**: The webhook server starts automatically on port 3030
2. **Start a task timer**: Activate a task in Chronos
3. **Make commits**: Work on your code and commit as usual
4. **Check task description**: Your commits will be appended automatically

### Example

```bash
# Start working on a task in Chronos
# Task: "Implement Git webhook feature"

# Make some commits
git add webhook_server.rs
git commit -m "Add Axum webhook server"

# The commit is automatically logged to your active task
```

**Task description after commit:**
```
Implement Git webhook server for automatic commit logging

--- Git Commit [chronos] ---
Author: Your Name
Message: Add Axum webhook server
Timestamp: 2026-04-19 22:30:15 UTC
```

## Testing

### Test the Webhook Server

```bash
# 1. Start Chronos app

# 2. Check if the server is running
curl http://127.0.0.1:3030/health

# Expected response:
# {"status":"ok","service":"chronos-webhook-server"}

# 3. Start a task timer in Chronos

# 4. Send a test webhook
curl -X POST http://127.0.0.1:3030/api/webhooks/git \
  -H "Content-Type: application/json" \
  -d '{
    "commit_message": "Test commit message",
    "repo_name": "test-repo",
    "author": "Test Author"
  }'

# Expected response:
# {
#   "success": true,
#   "message": "Commit logged to task: Your Task Title",
#   "task_id": "task_1234567890"
# }

# 5. Check the task in Chronos to see the logged commit
```

### Test the Git Hook

```bash
# In a repository with the hook installed
echo "test" >> README.md
git add README.md
git commit -m "Test webhook integration"

# You should see:
# ✓ Commit logged to Chronos: [repo-name] abc1234
```

## Troubleshooting

### Hook not working

```bash
# Check if the hook is executable
ls -la .git/hooks/post-commit

# Make it executable if needed
chmod +x .git/hooks/post-commit

# Test the hook manually
.git/hooks/post-commit
```

### Server not responding

```bash
# Check if Chronos is running
ps aux | grep chronos

# Check if port 3030 is in use
ss -tlnp | grep 3030

# Check server logs (in Chronos terminal output)
# Look for: "[Webhook Server] Starting on http://127.0.0.1:3030"
```

### No active task error

```bash
# Response: "No active or recent task found"
# Solution: Start a task timer in Chronos before committing
```

### Commits not appearing in task

1. Verify the webhook server is running: `curl http://127.0.0.1:3030/health`
2. Check that you have an active or paused task
3. Look at Chronos terminal output for webhook logs
4. Try sending a manual curl request (see Testing section)

## Configuration

### Change Webhook Port

Edit `chronos/src-tauri/src/webhook_server.rs`:

```rust
let addr = "127.0.0.1:3030"; // Change port here
```

Then update `scripts/post-commit.sample`:

```bash
CHRONOS_WEBHOOK_URL="http://127.0.0.1:YOUR_PORT/api/webhooks/git"
```

### Customize Commit Format

Edit `chronos_backend/src/services/webhook_service.rs`:

```rust
let commit_entry = format!(
    "\n\n--- Git Commit [{}] ---\nAuthor: {}\nMessage: {}\nTimestamp: {}",
    // Customize this format
);
```

### Disable Silent Failures

Edit `scripts/post-commit.sample` and uncomment:

```bash
# echo "⚠ Chronos webhook unavailable (is the app running?)"
```

## Security Notes

- The webhook server binds to `127.0.0.1` (localhost only)
- No external network access
- No authentication required (local-only service)
- CORS enabled for development flexibility
- All data stays on your machine

## Uninstallation

### Remove from single repository
```bash
rm .git/hooks/post-commit
```

### Remove global template
```bash
rm ~/.git-templates/hooks/post-commit
```

### Disable webhook server
Comment out the server startup in `chronos/src-tauri/src/lib.rs`:

```rust
// tauri::async_runtime::spawn(async move {
//     webhook_server::start_webhook_server(db_arc).await
// });
```

## Advanced Usage

### Multi-Repository Tracking

The hook automatically includes the repository name in each commit log, making it easy to track work across multiple projects.

### Commit Message Filtering

Modify the `post-commit.sample` script to filter certain commits:

```bash
# Skip commits with "[skip-chronos]" in the message
if [[ $COMMIT_MESSAGE == *"[skip-chronos]"* ]]; then
  exit 0
fi
```

### Integration with Other Tools

The webhook endpoint can be called from any tool that supports HTTP:

- **GitHub Actions**: Send workflow results
- **GitLab CI**: Log pipeline events
- **Custom scripts**: Any bash/python script
- **Other IDEs**: VS Code, IntelliJ extensions

## Future Enhancements

Potential improvements for future versions:

- Branch name tracking
- Commit hash/SHA logging
- File change statistics
- Integration with GitHub/GitLab APIs
- Customizable commit templates
- Webhook authentication (optional)
- Multiple webhook endpoints
- Webhook event filtering

---

**Documentation Version**: 1.0
**Last Updated**: 2026-04-19
**Chronos Version**: 0.1.0
