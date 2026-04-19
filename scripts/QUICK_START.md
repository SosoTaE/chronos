# Git Webhook - Quick Start Guide

## 5-Minute Setup

### Step 1: Start Chronos (2 min)

```bash
cd /home/sosotae/Documents/programming/chronos/src-tauri
cargo tauri dev
```

**Look for this message:**
```
[Webhook Server] Starting on http://127.0.0.1:3030
[Tauri] Webhook server started successfully
```

### Step 2: Verify Server (30 seconds)

```bash
curl http://127.0.0.1:3030/health
```

**Expected response:**
```json
{"status":"ok","service":"chronos-webhook-server"}
```

### Step 3: Install Git Hook (1 min)

**For a single repository:**
```bash
# Navigate to your Git repository
cd /path/to/your/repo

# Install the hook
cp /home/sosotae/Documents/programming/chronos/scripts/post-commit.sample .git/hooks/post-commit
chmod +x .git/hooks/post-commit
```

**For all repositories (recommended):**
```bash
# Create global template
mkdir -p ~/.git-templates/hooks
cp /home/sosotae/Documents/programming/chronos/scripts/post-commit.sample ~/.git-templates/hooks/post-commit
chmod +x ~/.git-templates/hooks/post-commit

# Configure Git
git config --global init.templatedir '~/.git-templates'

# Apply to existing repos
cd /path/to/existing/repo
git init
```

### Step 4: Test It (1 min)

```bash
# In Chronos UI: Start a task timer

# In your Git repository:
echo "test" >> README.md
git add README.md
git commit -m "Test webhook integration"
```

**Expected output:**
```
✓ Commit logged to Chronos: [your-repo] abc1234
```

### Step 5: Verify in Chronos (30 seconds)

Check your task description in Chronos. You should see:

```
--- Git Commit [your-repo] ---
Author: Your Name
Message: Test webhook integration
Timestamp: 2026-04-19 HH:MM:SS UTC
```

## That's It!

Every commit you make will now be automatically logged to your active Chronos task.

## Troubleshooting

### "Connection refused" error
→ Make sure Chronos is running: `curl http://127.0.0.1:3030/health`

### "No active or recent task" error
→ Start a task timer in Chronos before committing

### Hook not executing
→ Make sure it's executable: `chmod +x .git/hooks/post-commit`

### Commits not appearing
→ Check Chronos terminal output for webhook logs

## Manual Test

Start a task, then run:

```bash
curl -X POST http://127.0.0.1:3030/api/webhooks/git \
  -H "Content-Type: application/json" \
  -d '{
    "commit_message": "Manual test",
    "repo_name": "test-repo",
    "author": "Your Name"
  }'
```

Should return:
```json
{
  "success": true,
  "message": "Commit logged to task: Your Task Title",
  "task_id": "task_..."
}
```

## Advanced Configuration

See `WEBHOOK_SETUP.md` for:
- Changing the webhook port
- Customizing commit format
- Multi-repository tracking
- Integration with other tools

---

**Total Setup Time**: ~5 minutes
**Maintenance**: Zero (runs automatically)
