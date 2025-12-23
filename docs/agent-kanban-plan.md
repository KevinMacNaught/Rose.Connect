# Claude Agent Kanban Board - Implementation Plan

## Overview

Transform the existing GPUI Kanban board into an agent management system that:
1. Spawns Claude Code CLI agents when cards enter "Processing"
2. Monitors agent progress and auto-moves cards on completion
3. Supports multiple target repositories
4. Persists state across restarts

## Current State

- Single `src/main.rs` (215 lines) with basic drag-drop Kanban
- 4 columns, 9 sample cards loaded from `data/kanban.json`
- `claude-agent-sdk = "0.1.1"` in Cargo.toml (unused)
- No git, process spawning, or agent integration

## Architecture

```
KanbanBoard (UI)  <-->  AgentManager (Entity)  <-->  Claude CLI (Subprocess)
     |                        |
     |-- on_drop triggers --> spawn_agent()
     |                        |
     |<-- cx.notify() ------- channel events from subprocess
```

**Key Pattern**: Use GPUI's `BackgroundExecutor` with `smol::process` to spawn Claude CLI. Communicate via `async_channel` to avoid blocking UI.

---

## File Structure (After Refactor)

```
src/
  main.rs                 # Entry point only
  kanban/
    mod.rs
    board.rs              # KanbanBoard component (from current main.rs)
    card.rs               # KanbanCard with extended fields
  agent/
    mod.rs
    manager.rs            # AgentManager - spawns/tracks agents
    process.rs            # ClaudeProcess - CLI subprocess wrapper
    events.rs             # AgentEvent enum, stream-json parsing
  data/
    mod.rs
    store.rs              # JSON persistence
    models.rs             # Task, Repository, AgentSession structs
```

---

## Data Model

### Enhanced Task
```rust
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,           // Prompt for Claude
    pub target_repo: Option<PathBuf>,  // Which repo to work in
    pub status: TaskStatus,
    pub agent_session: Option<AgentSession>,
    pub created_at: DateTime<Utc>,
}

pub enum TaskStatus {
    Queue,
    Processing,
    PendingReview,
    Done,
}

pub struct AgentSession {
    pub session_id: String,
    pub status: AgentStatus,
    pub started_at: DateTime<Utc>,
    pub cost_usd: f64,
    pub logs: Vec<LogEntry>,
}

pub enum AgentStatus {
    Starting,
    Running { current_tool: Option<String> },
    Succeeded,
    Failed { error: String },
}
```

### Repository Config
```rust
pub struct RepoConfig {
    pub name: String,
    pub path: PathBuf,
    pub build_command: String,  // e.g., "cargo build"
}
```

---

## Implementation Steps

### Step 1: Project Restructuring
- Create module directories: `src/kanban/`, `src/agent/`, `src/data/`
- Move existing `main.rs` logic into `kanban/board.rs`
- Update `main.rs` to import modules and set up window

### Step 2: Update Dependencies
```toml
# Cargo.toml changes
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
async-channel = "2.0"
smol = "2.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4"] }
# Remove: claude-agent-sdk (we'll spawn CLI directly)
```

### Step 3: Implement Data Models (`src/data/models.rs`)
- Define `Task`, `TaskStatus`, `AgentSession`, `AgentStatus`
- Define `RepoConfig`, `AppState`
- Implement JSON serialization

### Step 4: Implement Persistence (`src/data/store.rs`)
- Load/save `data/kanban.json` with extended schema
- Load/save `data/config.json` for repo settings
- Debounced auto-save on state changes

### Step 5: Implement Claude CLI Wrapper (`src/agent/process.rs`)
```rust
// Spawn Claude with stream-json output
let child = smol::process::Command::new("claude")
    .args(["-p", &prompt, "--output-format", "stream-json"])
    .current_dir(&repo_path)
    .stdout(Stdio::piped())
    .spawn()?;
```
- Parse NDJSON stream into `AgentEvent` enum
- Send events through channel to `AgentManager`

### Step 6: Implement Agent Manager (`src/agent/manager.rs`)
- `Entity<AgentManager>` with channel receiver
- `spawn_agent(card_id, prompt, repo_path)` - spawns CLI in background
- `poll_events()` - drains channel and updates state
- Emits `AgentCompleted { card_id, success }` event for board

### Step 7: Integrate with KanbanBoard (`src/kanban/board.rs`)
- Subscribe to `AgentManager` completion events
- On `on_drop` into Processing column → trigger `spawn_agent()`
- On `AgentCompleted(success=true)` → move card to Pending Review
- On `AgentCompleted(success=false)` → move card back to Queue
- Render agent status indicator on cards

### Step 8: Add Task Creation UI
- "Add Task" button opens modal/form
- Fields: title, description (prompt), target repo dropdown
- Creates card in Queue column

### Step 9: Add Repo Configuration
- Settings panel to add/edit repositories
- Each repo: name, path, build command
- Persist to `data/config.json`

---

## Critical Files to Modify

| File | Changes |
|------|---------|
| `src/main.rs` | Slim down to entry point, import modules |
| `Cargo.toml` | Add async-channel, smol, chrono, uuid; remove claude-agent-sdk |
| `data/kanban.json` | Extend schema with task fields |

## New Files to Create

| File | Purpose |
|------|---------|
| `src/kanban/mod.rs` | Module exports |
| `src/kanban/board.rs` | KanbanBoard component (moved from main.rs) |
| `src/kanban/card.rs` | KanbanCard with status rendering |
| `src/agent/mod.rs` | Module exports |
| `src/agent/manager.rs` | AgentManager entity |
| `src/agent/process.rs` | Claude CLI subprocess wrapper |
| `src/agent/events.rs` | AgentEvent enum and parsing |
| `src/data/mod.rs` | Module exports |
| `src/data/models.rs` | Task, RepoConfig structs |
| `src/data/store.rs` | Persistence layer |
| `data/config.json` | Repository configuration |

---

## Phase 2 (Future): Git Worktrees

After basic agent integration works:
1. Create worktree per task: `git worktree add -b task-{id} .worktrees/{id}`
2. Run agent in worktree directory
3. On approval, merge worktree branch to main
4. Cleanup worktree after merge
5. Support multiple simultaneous agents in parallel worktrees

---

## Success Criteria

- [ ] Cards dropped in Processing auto-spawn Claude agent
- [ ] Agent status visible on card (Starting, Running, Succeeded, Failed)
- [ ] Successful agent completion auto-moves card to Pending Review
- [ ] Failed agent moves card back to Queue with error info
- [ ] State persists across app restarts
- [ ] Multiple repos can be configured and targeted
