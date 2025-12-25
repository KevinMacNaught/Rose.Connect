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

| Priority | Feature | Effort | Impact |
|----------|---------|--------|--------|
| 1 | Query History | Medium | High |
| 2 | Table Context Menu (basic) | Low | High |
| 3 | Query Formatter | Low-Medium | High |
| 4 | Keyboard Shortcuts | Low | Medium |
| 5 | Copy Enhancements | Low | Medium |
| 6 | EXPLAIN Visualization | High | High |
| 7 | Saved Queries | Medium | High |
| 8 | Multiple Connections | Medium | High |
| 9 | Column Quick Stats | Medium | Medium |
| 10 | Query Cancellation | Low | High |

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

## Notes

- Current features already implemented:
  - SQL autocomplete with schema awareness
  - DDL/DML safety warnings
  - Auto SQL keyword capitalization
  - Foreign key navigation
  - Cell editing with save
  - Export to CSV/JSON
  - Table structure panel

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
