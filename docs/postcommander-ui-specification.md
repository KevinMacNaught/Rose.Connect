# PostCommander UI Specification - Implementation Checklist

This document tracks the implementation status of all PostCommander features. Check off items as they are completed.

---

## Implementation Status Legend
- ✅ = Fully implemented
- 🔶 = Partially implemented
- ⬜ = Not started

---

## 1. Core Layout & Basic Features

### Layout
- [x] ✅ Sidebar with tree view
- [x] ✅ Tabs bar for query tabs
- [x] ✅ Query editor area
- [x] ✅ Results area (below editor)
- [x] ✅ Status bar at bottom

### Sidebar Resizing
- [x] ✅ Drag handle between sidebar and main area
- [x] ✅ Min/max width constraints (180px - 500px)
- [x] ✅ Persist width to settings

### Editor/Results Split
- [x] ✅ Horizontal drag handle between editor and results
- [x] ✅ Min/max height constraints (100px - 600px)
- [x] ✅ Persist height to settings

### Query Tabs
- [x] ✅ Add new tab (+button)
- [x] ✅ Close tab (X button)
- [x] ✅ Switch between tabs
- [x] ✅ Tab shows name and database

---

## 2. Connection & Database

### Connection Dialog
- [x] ✅ Host input
- [x] ✅ Port input
- [x] ✅ Database input
- [x] ✅ Username input
- [x] ✅ Password input (masked)
- [ ] ⬜ Connection Name field
- [ ] ⬜ SSL toggle
- [x] ✅ Connect button
- [x] ✅ Cancel button
- [x] ✅ Backdrop click to close

### Connection Persistence
- [x] ✅ Save connection to settings
- [x] ✅ Auto-reconnect on app launch (if saved connection exists)
- [ ] ⬜ Password prompt flow (for connections without cached password)

### Multi-Connection Support
- [ ] ⬜ Support multiple connections simultaneously
- [ ] ⬜ Connection ID on each connection
- [ ] ⬜ Connection selection/switching
- [ ] ⬜ Multiple root nodes in tree

---

## 3. Sidebar - Connection Tree

### Tree Structure
- [x] ✅ Server node (host:port)
- [x] ✅ Database node
- [x] ✅ Schema nodes (lazy loaded)
- [x] ✅ Tables folder per schema
- [x] ✅ Views folder per schema
- [ ] ⬜ Functions folder per schema

### Tree Interactions
- [x] ✅ Click to expand/collapse nodes
- [x] ✅ Persist expanded state to settings
- [x] ✅ Lazy load schemas when database expanded
- [x] ✅ Double-click table/view → Open SELECT query tab + auto-execute
- [ ] ⬜ Drag tables/views (for dropping elsewhere)
- [ ] ⬜ Local connection badge indicator

### Context Menus
**Table Node:**
- [x] ✅ Select Top 100 (opens new tab, auto-executes)
- [ ] ⬜ New Query

**View Node:**
- [ ] ⬜ Select Top 100
- [ ] ⬜ New Query

**Connection Node:**
- [ ] ⬜ New Query
- [ ] ⬜ Refresh
- [ ] ⬜ Edit Connection
- [ ] ⬜ Delete Connection (destructive)

**Database Node:**
- [ ] ⬜ New Query
- [ ] ⬜ Refresh

**Schema Node:**
- [ ] ⬜ New Query
- [ ] ⬜ Refresh

**Function Node:**
- [ ] ⬜ New Query
- [ ] ⬜ Drop Function (destructive)

### Search/Filter
- [x] 🔶 Search input UI exists
- [ ] ⬜ Functional filtering of tree
- [ ] ⬜ Recursive matching (shows ancestors)
- [ ] ⬜ Auto-expand matching nodes
- [ ] ⬜ Clear button (X) when has value

---

## 4. Sidebar - Functions Panel

> **Status: ⬜ Not Started**

- [ ] ⬜ Functions panel layout
- [ ] ⬜ Search/filter input
- [ ] ⬜ Function list display
- [ ] ⬜ Function executor panel
  - [ ] ⬜ Argument input fields
  - [ ] ⬜ "Run in Editor" button
  - [ ] ⬜ "Copy SQL" button
  - [ ] ⬜ "View Definition" button
- [ ] ⬜ Loading state
- [ ] ⬜ Error state
- [ ] ⬜ Empty state

---

## 5. Query Editor

### Editor Component
- [x] ✅ SQL syntax highlighting (via gpui-component Input.code_editor("sql"))
- [x] ✅ Line numbers
- [x] ✅ Soft wrap
- [x] ✅ Theme sync with app dark/light mode

### Toolbar
- [x] ✅ Execute button (Run/Running state)
- [x] ✅ AI button (placeholder, no functionality)
- [ ] ⬜ Save button (⌘S)
- [ ] ⬜ Structure panel toggle button

### Editor Enhancements
- [ ] ⬜ SQL keyword auto-capitalization
- [ ] ⬜ Dangerous SQL detection warning (DROP, DELETE without WHERE, TRUNCATE)
- [ ] ⬜ Connection-aware autocomplete

---

## 6. Query Tabs System

### Tab State
- [x] ✅ id
- [x] ✅ name
- [x] ✅ database
- [x] ✅ editor (InputState entity)
- [x] ✅ table_state (DataTableState entity)
- [x] ✅ table_context (schema, table, primary keys)
- [x] ✅ result
- [x] ✅ error
- [x] ✅ is_loading
- [ ] ⬜ connectionId
- [ ] ⬜ tableContext.foreignKeys
- [ ] ⬜ autoExecute flag

### Tab Persistence
- [ ] ⬜ Persist tabs to settings (id, name, connectionId, database, query)
- [ ] ⬜ Restore tabs on app launch

### Drag & Drop
- [ ] ⬜ Drag .sql file over tabs area
- [ ] ⬜ Highlight ring on drag over
- [ ] ⬜ Parse file and show connection picker on drop

### Empty State
- [x] ✅ "Ready to query" / "Connect to database" message
- [x] ✅ "New Query" / "Connect" button
- [ ] ⬜ "or drag and drop a .sql file here" hint
- [ ] ⬜ Quick tips section (⌘K AI, ⌘O open, ⌘↵ run)

---

## 7. Results Table

### Basic Display
- [x] ✅ Column headers with name and type
- [x] ✅ Data rows with cells
- [x] ✅ Row alternating background
- [x] ✅ Hover highlight
- [x] ✅ Primary key indicator in column headers

### Column Features
- [x] ✅ Column resizing via drag handles
- [x] ✅ Text truncation with ellipsis
- [ ] ⬜ FK icon + light primary background for FK columns
- [ ] ⬜ Checkbox column for row selection

### Cell Display
- [x] ✅ NULL values shown as em-dash (—) in muted color
- [ ] ⬜ Expandable values button (for >50 chars or multiline)

### Results Header
- [x] ✅ Execution time (ms)
- [x] ✅ Row count
- [x] ✅ Export button with dropdown menu
- [ ] ⬜ "Editable" indicator when table has primary keys
- [x] ✅ Export dropdown menu
  - [x] ✅ Copy as CSV (to clipboard)
  - [x] ✅ Copy as JSON (to clipboard)
  - [x] ✅ Copy as Markdown (max 100 rows, to clipboard)
  - [ ] ⬜ Export as SQL INSERT

### Foreign Key Cells
- [ ] ⬜ Show external link icon on hover
- [ ] ⬜ Click opens new tab with SELECT for referenced record

### Row Selection
- [ ] ⬜ Checkbox column on left
- [ ] ⬜ Multi-select support
- [ ] ⬜ Selection state tracking

### Selection Footer
- [ ] ⬜ "N rows selected" count
- [ ] ⬜ Clear selection button
- [ ] ⬜ Bulk Edit button (if editable)
- [ ] ⬜ Delete button (if editable)
- [ ] ⬜ View Related button (if has FKs)

---

## 8. Cell Editing

### Activation
- [x] ✅ Double-click on cell opens edit modal
- [ ] ⬜ If not editable, copy value to clipboard instead

### Edit Modal
- [x] ✅ Column name in header
- [x] ✅ Keyboard shortcuts display (⌘↵ save, esc cancel)
- [x] ✅ Textarea input
- [x] ✅ Cancel button
- [x] ✅ Save button
- [x] ✅ Saving state indicator
- [x] ✅ Error display
- [x] ✅ Backdrop click to cancel
- [x] ✅ Escape key to cancel
- [x] ✅ ⌘↵ to save
- [ ] ⬜ Expand button for large values (>300 chars or >3 newlines)
- [ ] ⬜ Type "NULL" to set null value detection

### Database Update
- [x] ✅ Build UPDATE query with WHERE using primary keys
- [x] ✅ Execute UPDATE
- [x] ✅ Update local cell value on success
- [x] ✅ Show error on failure

### Expanded Edit Dialog
- [ ] ⬜ Full modal with larger textarea (300px min-height, 60vh max)

---

## 9. Structure Panel

> **Status: ⬜ Not Started**

- [ ] ⬜ Panel layout (right side of editor)
- [ ] ⬜ Toggle button in toolbar
- [ ] ⬜ Resizable width (15% - 40%)

### List View
- [ ] ⬜ Collapsible sections for each table in query
- [ ] ⬜ Column list with name and type
- [ ] ⬜ PK icon for primary key columns
- [ ] ⬜ FK icon for foreign key columns
- [ ] ⬜ FK tooltip showing referenced table.column

### Diagram View
- [ ] ⬜ Visual boxes for tables
- [ ] ⬜ Lines connecting FK relationships
- [ ] ⬜ Interactive (click to focus/open query)

### Query Parsing
- [ ] ⬜ Parse SQL to extract tables from FROM/JOIN
- [ ] ⬜ Support schema-qualified names
- [ ] ⬜ Support aliases
- [ ] ⬜ Debounce parsing (400ms)

### Empty State
- [ ] ⬜ "Table structure appears here" message
- [ ] ⬜ "Add FROM or JOIN to your query" hint

---

## 10. AI SQL Assistant

> **Status: ⬜ Not Started**

### Activation
- [ ] ⬜ ⌘K keyboard shortcut
- [ ] ⬜ AI button in toolbar triggers overlay

### Overlay Layout
- [ ] ⬜ Header with title and close button
- [ ] ⬜ Table selection area
- [ ] ⬜ AI suggestions pills
- [ ] ⬜ Prompt input
- [ ] ⬜ Generate button
- [ ] ⬜ Hide suggestions toggle

### Table Selection
- [ ] ⬜ Add table button with dropdown picker
- [ ] ⬜ @mention autocomplete in prompt
- [ ] ⬜ Selected tables as pills with X to remove
- [ ] ⬜ FK expansion (auto-add related tables)
- [ ] ⬜ "via column_name" indicator for FK-added tables

### AI Suggestions
- [ ] ⬜ Generate prompt suggestions after tables selected
- [ ] ⬜ Display as clickable pills
- [ ] ⬜ Insert into prompt on click

### Generation
- [ ] ⬜ Call AI to generate SQL
- [ ] ⬜ Stream result into editor
- [ ] ⬜ Dangerous SQL warning toast
- [ ] ⬜ Auto-execute if safe

### Cancel
- [ ] ⬜ Escape key closes overlay
- [ ] ⬜ X button closes overlay
- [ ] ⬜ Abort generation if in progress

---

## 11. Status Bar

- [x] ✅ Connection status indicator (dot + text)
- [x] ✅ Host:port when connected
- [x] ✅ Database name when connected
- [x] ✅ Version number
- [ ] ⬜ Keyboard shortcuts hints (⌘K AI, ⌘O open, ⌘↵ run)

---

## 12. Dialogs & Modals

### Implemented
- [x] ✅ Connection dialog
- [x] ✅ Cell edit modal

### Not Implemented
- [ ] ⬜ Password prompt dialog
- [ ] ⬜ Open file dialog (for .sql files)
- [ ] ⬜ Expanded cell dialog (for viewing/editing large values)
- [ ] ⬜ Delete confirmation dialog
- [ ] ⬜ Bulk edit dialog
- [ ] ⬜ Batch preview dialog (FK-related records)

---

## 13. Keyboard Shortcuts

### Global
| Shortcut | Action | Status |
|----------|--------|--------|
| ⌘O | Open SQL file | ⬜ |
| Escape | Close AI assistant / Cancel edit | 🔶 (edit only) |

### Query Editor
| Shortcut | Action | Status |
|----------|--------|--------|
| ⌘↵ | Execute query | ✅ |
| ⌘K | Open AI SQL Assistant | ⬜ |
| ⌘S | Save query to file | ⬜ |

### AI Assistant
| Shortcut | Action | Status |
|----------|--------|--------|
| @ | Trigger table mention | ⬜ |
| ↑/↓ | Navigate suggestions | ⬜ |
| Tab/Enter | Select suggestion | ⬜ |
| Enter | Generate (when ready) | ⬜ |
| Escape | Close | ⬜ |

### Cell Editing
| Shortcut | Action | Status |
|----------|--------|--------|
| Double-click | Enter edit mode | ✅ |
| ⌘↵ | Save edit | ✅ |
| Escape | Cancel edit | ✅ |

---

## 14. Data Types & State Management

### QueryTab (Enhanced)
```
Current fields:
✅ id: String
✅ name: String
✅ database: String
✅ editor: Entity<InputState>
✅ table_state: Entity<DataTableState>
✅ table_context: Option<TableContext>
✅ result: Option<QueryResult>
✅ error: Option<String>
✅ is_loading: bool

Missing fields:
⬜ connectionId: String
⬜ autoExecute: bool
```

### TableContext
```
Current fields:
✅ schema: String
✅ table: String
✅ primary_keys: Vec<String>

Missing fields:
⬜ foreign_keys: Vec<ForeignKey>
```

### ConnectionConfig (Enhanced)
```
Current fields:
✅ name: String (hardcoded "Local PostgreSQL")
✅ host: String
✅ port: u16
✅ database: String
✅ username: String
✅ password: String

Missing fields:
⬜ id: String
⬜ ssl: bool
⬜ is_local: bool
```

### State Persistence
```
What persists (in settings):
✅ Connection (host, port, database, username, password)
✅ Expanded nodes
✅ Sidebar width
✅ Editor height
⬜ Tab definitions (not persisted)

What clears on reload:
✅ Query results
✅ Execution state
✅ Schema data (re-fetched on expand)
✅ Cell editing state
```

---

## Summary

### Completed Features
- Basic layout (sidebar, tabs, editor, results, status bar)
- Connection dialog and database connection
- Query execution and results display
- Tree view with schemas, tables, views
- Table context menu with "Select Top 100"
- Resizable sidebar and editor/results split
- Query tabs (add, close, switch)
- SQL code editor with syntax highlighting
- Cell editing with modal and UPDATE queries
- Settings persistence (connection, expanded nodes, sizes)

### Priority Next Steps (Suggested Order)
1. ~~**Keyboard shortcuts** (⌘↵ for execute at minimum)~~ ✅
2. ~~**Double-click table** → open SELECT query~~ ✅
3. ~~**Export functionality** (CSV at minimum)~~ ✅
4. **Search/filter in sidebar**
5. ~~**NULL display as em-dash**~~ ✅
6. **Connection Name + SSL in dialog**
7. **Tab persistence across reload**
8. **Structure Panel**
9. **AI SQL Assistant**
10. **Multi-connection support**

---

*Last updated: December 2024*
