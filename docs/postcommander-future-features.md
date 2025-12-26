# PostCommander Future Features

Feature ideas to make PostCommander a joy to use for database admins.

---

## High-Impact Features

### 1. Query History
- Searchable history of all executed queries with timestamps
- One-click to re-run or copy to editor
- Persist across sessions
- Filter by date, connection, or search term

### 2. Saved Queries / Snippets
- Bookmark frequently used queries with names like "slow queries", "table sizes", "active connections"
- Organize in folders or tags
- Share across connections
- Import/export snippets

### 3. EXPLAIN ANALYZE Visualization
- Visual tree showing query execution plan
- Color-coded nodes by cost (green = fast, red = slow)
- Show time spent per node, rows estimated vs actual
- Highlight sequential scans, missing indexes
- Clickable nodes for details

### 4. Query Formatter / Beautify
- Auto-format messy SQL into properly indented, readable SQL
- Consistent keyword casing (uppercase keywords)
- Configurable style preferences
- Keyboard shortcut (Cmd+Shift+F)

---

## Table Context Menu

Right-click menu options for tables in the sidebar:

### Currently Implemented
- Select Top 100
- Copy Name
- Copy Qualified Name
- Count Rows
- Generate SELECT

### Proposed Additions

**Data Operations:**
- Select Top 1000 — when 100 isn't enough
- Truncate Table... — with scary confirmation dialog

**Schema Information:**
- View Structure — columns, types, nullability, defaults (may already exist via panel?)
- View Indexes — for performance troubleshooting
- View Constraints — PKs, FKs, unique, check constraints
- Table Size/Stats — disk usage, row count, bloat detection

**SQL Generation (submenu):**
- Generate INSERT Template — pre-filled column names
- Generate UPDATE Template
- Generate DELETE Template
- Generate CREATE TABLE Script — for documentation or migrations

**Export (submenu):**
- Export to CSV
- Export to JSON
- Export to SQL INSERTs

**Maintenance (submenu):**
- VACUUM
- VACUUM FULL
- ANALYZE
- REINDEX

**Navigation:**
- View Related Tables — show FK relationships
- Open in New Tab — compare tables side-by-side

### Suggested Menu Organization

```
Right-click on table:
├── Select Top 100
├── Select Top 1000
├── Count Rows
├── ─────────────
├── Copy Name
├── Copy Qualified Name
├── ─────────────
├── View Structure
├── View Indexes
├── View Constraints
├── Table Size/Stats
├── ─────────────
├── Generate SQL        ▶ SELECT, INSERT, UPDATE, DELETE, CREATE TABLE
├── Export              ▶ CSV, JSON, SQL Inserts
├── ─────────────
├── Maintenance         ▶ VACUUM, VACUUM FULL, ANALYZE, REINDEX
├── ─────────────
├── View Related Tables
├── Open in New Tab
├── ─────────────
├── Truncate Table...
```

---

## Quality of Life

### 5. Multiple Connections
- Save multiple connection profiles (dev, staging, prod) with nicknames
- Color-coded tabs per connection (red border = production!)
- Quick switcher dropdown
- Connection groups/folders

### 6. Table/Database Statistics
- Table sizes, row counts, index sizes in the sidebar
- Bloat indicator, last vacuum/analyze time
- Dead tuple count
- Visual indicator for tables needing maintenance

### 7. Column Quick Stats
- Right-click a column in results → "Analyze Column"
- Shows: distinct count, NULL %, min/max values
- Most common values with frequencies
- Data distribution histogram
- Useful for understanding unfamiliar data

### 8. Copy Enhancements
- Copy cell, row, or selection as:
  - CSV
  - JSON
  - SQL INSERT statement
  - Markdown table
  - Tab-separated (for Excel)
- Copy column names from results header
- Right-click context menu on results

---

## Power User Features

### 9. Keyboard Shortcuts
- Cmd+Enter = Execute query (done)
- Cmd+Shift+E = EXPLAIN current query
- Cmd+Shift+A = EXPLAIN ANALYZE current query
- Cmd+Shift+F = Format SQL
- Cmd+/ = Comment/uncomment line
- Cmd+D = Duplicate line
- Cmd+L = Go to line
- Cmd+? = Show shortcuts cheatsheet

### 10. Query Templates
- Common operations with placeholders:
  - Add Column
  - Create Index
  - Create Table
  - Grant Permissions
  - Common JOINs
- Fill-in-the-blanks style with tab navigation
- User-definable templates

### 11. Quick Filters on Results
- Filter results without re-running query
- Click column header to filter/sort
- Excel-style dropdown with distinct values
- Search within results

### 12. Schema Comparison / Diff
- Compare schema between two databases
- Show added/removed/modified tables, columns, indexes
- Generate migration SQL

---

## Visual / UX

### 13. ERD / Relationship Diagram
- Visual diagram showing table relationships via foreign keys
- Click a table to highlight its relationships
- Drag to rearrange, zoom in/out
- Export as image

### 14. Connection Health Indicator
- Show connection status in status bar
- Auto-reconnect on disconnect
- Show active queries, connection pool status

### 15. Results Pagination
- For large result sets, paginate instead of loading all
- "Load more" or page numbers
- Jump to specific page

---

## Priority Recommendations

If implementing incrementally:

| Priority | Feature | Effort | Impact | Status |
|----------|---------|--------|--------|--------|
| 1 | Query History | Medium | High | ✅ Done |
| 2 | Table Context Menu (basic) | Low | High | ✅ Done |
| 3 | Query Formatter | Low-Medium | High | ✅ Done |
| 4 | Keyboard Shortcuts | Low | Medium | ✅ Done |
| 5 | Copy Enhancements | Low | Medium | ✅ Done |
| 6 | EXPLAIN Visualization | High | High | |
| 7 | Saved Queries | Medium | High | ✅ Done |
| 8 | Multiple Connections | Medium | High | |
| 9 | Column Quick Stats | Medium | Medium | |
| 10 | Query Cancellation | Low | High | ✅ Done |

*Table Context Menu (basic) = Count Rows, Copy Name, Copy Qualified Name, Generate SELECT*

---

## Implementation Progress

### Sprint: Table Context Menu (Basic)
**Status:** ✅ Complete
**Started:** 2025-12-25
**Completed:** 2025-12-25

Implemented Priority #2 items:

| Item | Status | Notes |
|------|--------|-------|
| Count Rows | ✅ Done | Creates new tab with COUNT query, auto-executes |
| Copy Name | ✅ Done | Copies table name to clipboard |
| Copy Qualified Name | ✅ Done | Copies `"schema"."table"` to clipboard |
| Generate SELECT | ✅ Done | Inserts into active tab or creates new tab |

**Files Modified:**
- `src/postcommander/page.rs` - Extended `deploy_table_context_menu()` with 4 new items + separator
- `src/postcommander/tabs.rs` - Added `count_table_rows()` and `generate_select_statement()` methods

**Menu now includes:**
```
Select Top 100
─────────────
Copy Name
Copy Qualified Name
Count Rows
Generate SELECT
```

---

### Sprint: Query Cancellation
**Status:** ✅ Complete
**Started:** 2025-12-25
**Completed:** 2025-12-25

Implemented Priority #10 - DBA-requested feature: "Long-running queries need a kill button. Show elapsed time, let me abort."

| Item | Status | Notes |
|------|--------|-------|
| Show elapsed time during query | ✅ Done | Button shows "Running... (2.5s)" with real-time updates |
| Add cancel button | ✅ Done | Stop button (square icon) appears next to Execute during query |
| Implement query cancellation | ✅ Done | Drops async task, shows "Query cancelled" message |

**Files Modified:**
- `src/postcommander/types.rs` - Added `query_start_time: Option<Instant>` and `query_task: Option<Task<()>>` to QueryTab
- `src/postcommander/tabs.rs` - Initialize new fields in all QueryTab construction sites
- `src/postcommander/query_execution.rs` - Added `cancel_query()` method, store task handle instead of detaching, periodic UI refresh for elapsed time
- `src/postcommander/results.rs` - Display elapsed time in button text, added cancel button with stop icon

**Key Implementation Details:**
- Uses GPUI's `Task` type for cancellable async operations
- Periodic UI refresh (100ms) via background spawn task for smooth elapsed time display
- Cancel button positioned between Execute and AI buttons
- Clean state reset on cancellation (loading=false, error="Query cancelled")

---

### Sprint: Keyboard Shortcuts
**Status:** ✅ Complete
**Started:** 2025-12-25
**Completed:** 2025-12-25

Implemented Priority #4 - DBA feedback: "Cmd+Enter is muscle memory. The rest make it feel professional."

| Item | Status | Notes |
|------|--------|-------|
| Cmd+Enter = Execute query | ✅ Done | Already implemented in page.rs |
| Cmd+Shift+E = EXPLAIN query | ✅ Done | Wraps query in EXPLAIN and auto-executes |
| Cmd+Shift+A = EXPLAIN ANALYZE | ✅ Done | Wraps query in EXPLAIN ANALYZE and auto-executes |
| Cmd+/ = Comment/uncomment | ✅ Done | Toggles `-- ` prefix on current line |

**Files Modified:**
- `src/postcommander/page.rs` - Extended `on_key_down` handler with 3 new shortcuts
- `src/postcommander/query_execution.rs` - Added `explain_query()`, `explain_analyze_query()`, `toggle_comment()` methods

**Key Implementation Details:**
- EXPLAIN shortcuts prepend keyword to current query, reposition cursor, and execute
- Comment toggle preserves line indentation (leading whitespace)
- Handles both `--` and `-- ` comment prefixes when uncommenting
- All shortcuts check for active tab before proceeding

**Deferred to later:**
- Cmd+? (Shortcuts cheatsheet) → Lower priority

---

### Sprint: Query Formatter
**Status:** ✅ Complete
**Started:** 2025-12-25
**Completed:** 2025-12-25

Implemented Priority #3 - Auto-format messy SQL into properly indented, readable SQL.

| Item | Status | Notes |
|------|--------|-------|
| Add sqlformat crate | ✅ Done | Pure Rust SQL formatter (version 0.2) |
| Cmd+Shift+F shortcut | ✅ Done | Added to keyboard handler in page.rs |
| format_query() method | ✅ Done | Beautifies SQL with proper indentation |

**Files Modified:**
- `Cargo.toml` - Added `sqlformat = "0.2"` dependency
- `src/postcommander/page.rs` - Added Cmd+Shift+F keyboard shortcut
- `src/postcommander/query_execution.rs` - Added `format_query()` method

**Key Implementation Details:**
- Uses `sqlformat` crate for professional SQL formatting
- 2-space indentation for nested clauses
- Uppercase keywords (SELECT, FROM, WHERE, etc.)
- 1 blank line between multiple queries
- Handles comments and string literals correctly

---

### Sprint: Copy Enhancements
**Status:** ✅ Complete
**Started:** 2025-12-25
**Completed:** 2025-12-25

Implemented Priority #5 - DBA feedback: "Copying as INSERT statements alone is worth it."

| Item | Status | Notes |
|------|--------|-------|
| Right-click context menu on cells | ✅ Done | Opens context menu on right-click anywhere in results |
| Copy cell value | ✅ Done | Copies single cell to clipboard |
| Copy row as TSV | ✅ Done | Tab-separated for Excel paste |
| Copy row as JSON | ✅ Done | JSON object with column names as keys |
| Copy row as SQL INSERT | ✅ Done | Uses actual table name when available |

**Files Modified:**
- `src/components/data_table/types.rs` - Added `CellContextMenu` event struct
- `src/components/data_table/mod.rs` - Exported new event type
- `src/components/data_table/render.rs` - Added right-click handler in `render_cell()`
- `src/postcommander/export.rs` - Added `copy_cell_value()`, `copy_row_as_tsv()`, `copy_row_as_json()`, `copy_row_as_insert()`
- `src/postcommander/tabs.rs` - Subscribe to `CellContextMenu` event
- `src/postcommander/state.rs` - Added `PendingCellContextMenu` and overlay state
- `src/postcommander/cell_edit.rs` - Added `handle_cell_context_menu()` handler
- `src/postcommander/page.rs` - Added `deploy_cell_context_menu()` and render integration

**Key Implementation Details:**
- Uses GPUI's event-driven architecture with `EventEmitter`
- Deferred menu building pattern (stores pending request, builds on render)
- Proper z-ordering with `deferred()` and `.occlude()`
- TSV format (tab-separated) for better Excel compatibility
- SQL INSERT uses actual table name from table_context if available

---

### Sprint: Query History
**Status:** ✅ Complete
**Started:** 2025-12-25
**Completed:** 2025-12-25

Implemented Priority #1 - DBA feedback: "Can't live without this. I re-run queries constantly."

| Item | Status | Notes |
|------|--------|-------|
| QueryHistoryEntry struct + persistence | ✅ Done | Store in settings.json, cap at 100 entries |
| Hook query execution to capture history | ✅ Done | Capture SQL, timestamp, duration, status |
| Query history panel UI | ✅ Done | Scrollable list with timestamps |
| Sidebar toggle (Schema ↔ History) | ✅ Done | Tab buttons at top of sidebar |
| Search/filter for history | ✅ Done | Real-time filter by SQL text |
| Re-run and copy-to-editor actions | ✅ Done | Click to copy, double-click to run |

**Files Modified:**
- `src/settings.rs` - Added QueryHistoryEntry, QueryHistoryStatus, QueryHistorySettings structs
- `src/postcommander/query_execution.rs` - Capture history on success/error/cancel
- `src/postcommander/query_history_panel.rs` - New file with history panel UI
- `src/postcommander/sidebar.rs` - Added sidebar tabs (Schema/History toggle)
- `src/postcommander/page.rs` - Added state fields, history search input
- `src/postcommander/mod.rs` - Added module declaration

**Key Implementation Details:**
- Uses chrono for ISO 8601 timestamps and friendly time formatting
- Persists to data/settings.json via AppSettings
- Newest entries prepended (most recent first)
- Max 100 entries with automatic trimming
- Single-click copies SQL to editor, double-click executes
- Real-time search filtering as you type

---

### Sprint: Saved Queries / Snippets
**Status:** ✅ Complete
**Started:** 2025-12-25
**Completed:** 2025-12-25

Implemented Priority #7 - DBA feedback: "'Show me slow queries' and 'active connections' are scripts I run 20x/day"

| Item | Status | Notes |
|------|--------|-------|
| SavedQueryEntry + persistence | ✅ Done | Store in settings.json with UUID, name, SQL, folder, description |
| Sidebar tab for Saved queries | ✅ Done | Refactored from boolean to SidebarTab enum with Schema/History/Saved |
| Saved queries panel UI | ✅ Done | Searchable list with name, SQL preview, optional folder |
| Save Query dialog | ✅ Done | Modal dialog with name, folder, description inputs |
| Right-click context menu | ✅ Done | Run, Edit, Copy SQL, Delete actions |
| Click to load, double-click to run | ✅ Done | Consistent with Query History behavior |

**Files Created:**
- `src/postcommander/saved_queries_panel.rs` - Panel UI with search, list rendering
- `src/postcommander/save_query_dialog.rs` - Modal dialog for save/edit

**Files Modified:**
- `src/settings.rs` - Added SavedQueryEntry, SavedQueriesSettings structs
- `src/postcommander/types.rs` - Added SidebarTab enum
- `src/postcommander/page.rs` - State fields, dialog initialization, context menu deployment
- `src/postcommander/sidebar.rs` - Three-tab system, removed placeholder
- `src/postcommander/state.rs` - SaveQueryDialogState, overlay state
- `src/postcommander/results.rs` - Save button in toolbar
- `src/postcommander/mod.rs` - Module declarations
- `Cargo.toml` - Added uuid dependency

**Key Implementation Details:**
- Uses UUID for unique entry identification
- Supports optional folder organization
- Edit mode populates dialog with existing entry data
- Context menu shows on right-click with proper highlighting
- Persists to data/settings.json via AppSettings
- Max 500 saved queries
- Real-time search filtering

**Critical Gotcha Discovered:**

Encountered GPUI crash with nested `deferred()` calls:
```
assertion `left == right` failed: cannot call defer_draw during deferred drawing
  left: 1
 right: 0
```

**Root cause:** In `page.rs`, we wrapped the dialog render method in `deferred()`:
```rust
.when(show_save_dialog, |el| {
    el.child(deferred(self.render_save_query_dialog(cx)).with_priority(3))
})
```

And `save_query_dialog.rs` ALSO returned `deferred()` from the render method, causing a nested deferred context.

**Fix:** Remove inner `deferred()` from `save_query_dialog.rs`, only wrap at call site.

**Pattern:** Either the caller wraps in `deferred()` OR the method returns `deferred()`, but NEVER both. See `docs/gpui-guide.md` § "Modal Dialogs" for full explanation.

**Pattern: Enum-based Tab State**

Replaced boolean `show_history_panel: bool` with `SidebarTab` enum for cleaner multi-tab management:

```rust
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SidebarTab {
    #[default]
    Schema,
    History,
    Saved,
}
```

Benefits over boolean flags:
- Scales cleanly (adding 4th tab doesn't require new boolean)
- Impossible states eliminated (can't have both History and Saved "true")
- Single source of truth for active tab
- Pattern matching ensures all tabs handled

---

## Notes

- Current features already implemented:
  - SQL autocomplete with schema awareness
  - DDL/DML safety warnings
  - Auto SQL keyword capitalization
  - Foreign key navigation
  - Cell editing with save
  - Export to CSV/JSON
  - Table structure panel
  - Table context menu (Copy Name, Copy Qualified Name, Count Rows, Generate SELECT)
  - Query cancellation with elapsed time display
  - Keyboard shortcuts (Cmd+Enter, Cmd+Shift+E, Cmd+Shift+A, Cmd+/, Cmd+Shift+F)
  - Query Formatter / SQL beautification
  - Cell/row copy enhancements (right-click → Copy Cell, TSV, JSON, INSERT)
  - Query History with search, click to copy, double-click to execute
  - Saved Queries with folders, search, save/edit dialog, context menu

---

## DBA Feedback on Proposed Features

Overall: This is an excellent roadmap. Here's my take as someone who would use this daily:

**Strongly Agree (would use constantly):**
- Query History (#1) — Can't live without this. I re-run queries constantly.
- Saved Queries (#2) — "Show me slow queries" and "active connections" are scripts I run 20x/day
- EXPLAIN ANALYZE Visualization (#3) — This is the killer feature. Text EXPLAIN output is painful. Visual trees with cost highlighting would be *chef's kiss*.
- Multiple Connections (#5) — Critical. Red border for production is genius.
- Keyboard Shortcuts (#9) — Cmd+Enter is muscle memory. The rest make it feel professional.
- Copy Enhancements (#8) — Copying as INSERT statements alone is worth it.

**Agree (weekly use):**
- Query Formatter (#4) — Nice but I'd use it less than you'd think (I format as I type)
- Table Statistics (#6) — Bloat indicator and last vacuum time are valuable
- Column Quick Stats (#7) — Very useful for unfamiliar databases
- ERD Diagram (#13) — Helpful for onboarding and documentation

**Lower Priority for Me:**
- Schema Comparison (#12) — I use dedicated migration tools for this
- Results Pagination (#15) — I rarely need 1000+ rows; LIMIT is my friend

**Additional Wish:**
- **Query cancellation** — Long-running queries need a kill button. Show elapsed time, let me abort.
- **Multiple result sets** — Some queries return multiple result sets; show them as tabs
- **pg_stat_statements integration** — Show slow query stats if the extension is enabled
