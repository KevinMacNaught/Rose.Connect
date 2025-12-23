# PostCommander UI Specification - Remaining Features

This document lists the remaining unimplemented features for PostCommander. Basic layout, connection dialog, query execution, and results display are already implemented.

---

## Table of Contents

1. [Sidebar - Connection Tree Enhancements](#sidebar---connection-tree-enhancements)
2. [Sidebar - Functions Panel](#sidebar---functions-panel)
3. [Query Tabs System Enhancements](#query-tabs-system-enhancements)
4. [Query Editor Enhancements](#query-editor-enhancements)
5. [Results Table Enhancements](#results-table-enhancements)
6. [Structure Panel](#structure-panel)
7. [AI SQL Assistant](#ai-sql-assistant)
8. [Status Bar Enhancements](#status-bar-enhancements)
9. [Dialogs and Modals](#dialogs-and-modals)
10. [Keyboard Shortcuts](#keyboard-shortcuts)
11. [State Management Enhancements](#state-management-enhancements)

---

## Sidebar - Connection Tree Enhancements

### Missing Features

**Functions Folder:**
- Add "Functions" folder under each schema (collapsed by default)
- Show function nodes with Code icon
- Lazy load functions when folder is expanded

**Local Connection Badge:**
- If a connection is marked as "local", display a small badge: `LOCAL` (outline style, 10px font)

**Double Click on Table/View:**
- Opens new query tab with `SELECT * FROM "schema"."table" LIMIT 100;`
- Auto-executes the query

**Drag Start (Tables/Views only):**
- Enable dragging with `cursor-grab` / `cursor-grabbing`
- Set drag data: `{ schema, name, connectionId, database }`

**Context Menus:**

**Connection Node:**
- New Query
- Refresh
- ---
- Edit Connection
- Delete Connection (destructive/red text)

**Database Node:**
- New Query
- Refresh

**Schema Node:**
- New Query
- Refresh

**Table/View Node:** (PARTIALLY IMPLEMENTED - Table context menu done)
- Select Top 100 ✓ (opens new tab with `SELECT * FROM "schema"."table" LIMIT 100;` and auto-executes)
- New Query

**Function Node:**
- New Query
- ---
- Drop Function (destructive/red text)

**Search/Filter Functionality:**
- Search input at top of sidebar (UI exists, functionality missing)
- Filters tree recursively - shows matching nodes and their ancestors
- Matching nodes expand automatically
- X clear button on right (when has value)

**Password Prompt Flow:**
1. When user expands a connection node without cached password
2. Show password dialog modal
3. On submit, test connection
4. If success, cache password and continue loading
5. If fail, show error in dialog, allow retry

**Multi-Connection Support:**
- Support multiple connections simultaneously
- Each connection appears as separate root node in tree
- Connection selection/switching

---

## Sidebar - Functions Panel

### Purpose
Lists all PostgreSQL functions from a schema manifest, allowing filtering and execution.

### Layout
```
+---------------------------+
| [Search Input]            |
+---------------------------+
| Function List             |
| - function_name           |
| - function_name           |
| ...                       |
+---------------------------+
| Function Executor         |  <- Only visible when a function is selected
| (input fields, buttons)   |
+---------------------------+
```

### Search Input
- Placeholder: "Filter functions..."
- Search icon on left
- X clear button on right (when has value)

### Function List
- Each function displayed as a clickable row
- Shows function name and optional description
- Click to select and show executor panel

### Function Executor Panel
When a function is selected, shows:
- Function name as header
- Input field for each argument with:
  - Label (argument name)
  - Type hint
  - Value input
  - Optional lookup popover for FK arguments
- **Buttons:**
  - "Run in Editor" - Opens new query tab with formatted SELECT call, auto-executes
  - "Copy SQL" - Copies the function call SQL to clipboard
  - "View Definition" - Shows the full function source in a modal

### Loading/Empty States
- **Loading**: Spinner with "Loading functions..."
- **Error**: Red text showing error message
- **Empty**: Code icon with "No functions found" and hint "Run pnpm schema:generate"

---

## Query Tabs System Enhancements

### Drag and Drop
- When dragging a .sql file over the tabs area:
  - Show highlight ring (primary color)
  - Show overlay with folder icon: "Drop SQL file to open"
- On drop, parse file and show connection picker dialog

### Tab State Enhancements
Each tab should maintain:
- `connectionId`: Associated connection (currently missing)
- `tableContext`: Parsed table info for editing (schema, table, primaryKeys, foreignKeys)
- `autoExecute`: Flag to auto-run on creation

### Empty State Enhancements
Add to empty state:
- "or drag and drop a .sql file here" hint
- Quick tips section:
  - ⌘K AI SQL Assistant
  - ⌘O Open file
  - ⌘↵ Execute

---

## Query Editor Enhancements

### Editor Implementation (COMPLETED)
Using `gpui-component`'s `Input` with `InputState` configured as a code editor:
- Language: SQL (via `tree-sitter-sequel`)
- Line numbers: enabled
- Soft wrap: enabled
- Syntax highlighting: automatic via gpui-component's `HighlightTheme`
- Theme sync: Automatically matches app's dark/light mode

**Implementation files:**
- `src/postcommander/types.rs` - `QueryTab.editor: Entity<InputState>`
- `src/postcommander/page.rs` - `add_tab()` creates editor with `.code_editor("sql")`
- `src/postcommander/results.rs` - `render_query_editor()` renders `Input::new(&editor)`
- `src/theme/mod.rs` - `sync_to_gpui_component()` sets `highlight_theme`

### Future Enhancements (NOT YET IMPLEMENTED)

### SQL Keyword Auto-Capitalization
When user types a SQL keyword followed by space/enter/tab/semicolon, automatically convert to uppercase.

Keywords list includes: `SELECT`, `FROM`, `WHERE`, `AND`, `OR`, `JOIN`, `LEFT`, `RIGHT`, `INNER`, `OUTER`, `GROUP BY`, `ORDER BY`, `HAVING`, `LIMIT`, `OFFSET`, `INSERT`, `UPDATE`, `DELETE`, `CREATE`, `DROP`, `ALTER`, `TABLE`, `INDEX`, `VIEW`, etc.

### Theme Sync
Editor theme dynamically matches application dark/light mode:
- Reads CSS custom properties (`--background`, `--foreground`, `--muted`, etc.)
- Computes if dark mode based on background luminance
- Creates custom Monaco theme with matching colors

### Dangerous SQL Detection
Before executing queries containing:
- DROP
- ALTER
- DELETE (without WHERE)
- TRUNCATE

Show a warning toast: "Dangerous SQL detected - review before executing"
Display which operations are detected.

### Autocomplete
- Connection-aware: suggests tables and columns from current database
- Triggered on typing or explicitly
- Shows table names with schema prefix when not `public`

### Toolbar Enhancements
Add missing buttons:
- **Save button** (Save icon, ⌘S): Save query to .sql file
- **Structure Toggle** (PanelRightOpen/PanelRightClose icon): Toggle structure panel visibility

### Resizable Editor/Results Split (IMPLEMENTED)
- Horizontal drag handle between editor and results
- Editor height: 200px default, 100px minimum, 600px maximum
- Sidebar width: 240px default, 180px minimum, 500px maximum
- **Both values persist** to settings and restore on app relaunch

---

## Results Table Enhancements

### Toolbar Enhancements
- Add "Editable" indicator (green text) when table has primary keys
- Implement Export dropdown menu with options:
  - Export as CSV
  - Export as JSON (pretty printed)
  - Export as SQL INSERT
  - Export as Markdown
  - Copy as Markdown (max 100 rows)

### Column Headers Enhancements
- If column is a foreign key: show Link icon before name, light primary background
- Add checkbox column on left for row selection

### Cell Display Enhancements
- Null values: Show em-dash (—) in muted color (currently shows "NULL")
- Expandable values: Show expand button on hover (values > 50 chars or multiline)
- Foreign key columns: Special rendering with clickable values

### Foreign Key Cells
- Display value normally
- On hover, show external link icon
- On click, opens new query tab with SELECT for referenced table/row

### Row Selection
- Checkbox column on left
- Multi-select supported
- Selection state tracked as array of row indices

### Selection Footer
Appears when rows are selected:
- Shows count: "3 rows selected"
- **Clear button**: Deselects all
- **Bulk Edit button**: Opens bulk edit dialog (only if editable)
- **Delete button**: Opens delete confirmation (only if editable)
- **View Related button**: Opens batch preview dialog (only if has foreign keys)

### Inline Cell Editing

**Activation:**
- Double-click on any cell
- If not editable (no primary keys), just copy value to clipboard

**Edit Popover:**
```
+----------------------------------+
| column_name    ⌘+↵ save  esc cancel |  <- Header
+----------------------------------+
| [Textarea]                       |
|                                  |
+----------------------------------+
| [Expand]        [Cancel] [Save]  |  <- Footer
+----------------------------------+
```

- Textarea auto-resizes up to 200px height
- Expand button shown for large values (>300 chars or >3 newlines)
- Type "NULL" to set null value
- Empty input on null original = keep null

**Expanded Edit Dialog:**
Full modal with larger textarea (300px min-height, 60vh max)

### Table Context Parsing
- Parse query results to detect if they come from a single table
- Extract primary keys and foreign keys from database metadata
- Store in `tableContext` for enabling editing features

---

## Structure Panel

### Purpose
Shows the schema structure of tables referenced in the current query.

### Toggle
Button in query editor toolbar toggles visibility.

### Header
```
+----------------------------------+
| Structure        [List] [Diagram]|
+----------------------------------+
```
- Two view modes: List view and Diagram view

### List View
Collapsible sections for each table in query:
```
▼ schema.table_name (alias)
  │ 🔑 id           uuid
  │ 🔗 company_id   uuid
  │    name         varchar(255)
  │    created_at   timestamp
```

**Column Indicators:**
- Key icon (amber): Primary key column
- Link icon (blue): Foreign key column
- Hovering FK icon shows tooltip: `referenced_table.column`

### Diagram View
Visual relationship diagram showing tables and their connections:
- Boxes for each table
- Lines connecting FK relationships
- Interactive - click to focus or open new query

### Empty State
When no FROM/JOIN in query:
```
     [Table Icon]
  Table structure appears here
  Add FROM or JOIN to your query
```

### Query Parsing
- Parses SQL to extract tables from FROM and JOIN clauses
- Supports schema-qualified names: `schema.table`
- Supports aliases: `table AS t`
- Debounces parsing (400ms delay)

### Panel Sizing
- Width: 25% default (when visible), 15% minimum, 40% maximum
- Resizable with vertical drag handle between editor and structure panel

---

## AI SQL Assistant

### Activation
- Press ⌘K or click AI button in editor toolbar
- Opens as overlay on top of editor

### Layout
```
+-----------------------------------------------+
| ✨ AI Query Builder                      [X] |  <- Header
+-----------------------------------------------+
| [Add table] @table1 @table2 via:col Loading...|  <- Table selection
+-----------------------------------------------+
| [suggestion 1] [suggestion 2] [suggestion 3]  |  <- AI suggestions (pills)
+-----------------------------------------------+
| Type @table to select, then describe query... |  <- Prompt input
|                               [Generate ↵]    |
+-----------------------------------------------+
|                    Hide suggestions           |  <- Toggle link
+-----------------------------------------------+
```

### Table Selection

**Add Table Button:**
- Opens dropdown picker with search
- Shows table name and type (table/view)
- Already selected tables are disabled
- Max 20 tables shown

**@Mention Autocomplete:**
- Typing `@` triggers autocomplete
- Shows filtered table list
- Arrow keys to navigate, Tab/Enter to select
- Inserts `@schema.table` into prompt

**Selected Tables Display:**
- Pills showing table name
- If added via FK expansion, shows "via column_name"
- X button to remove

**FK Expansion:**
- When a table is added, automatically fetches related tables via foreign keys
- Shows "Loading related..." indicator
- Related tables added with `viaColumn` attribute

### AI Suggestions
- After tables are selected, AI generates prompt suggestions
- Displayed as clickable pills/chips
- Click to insert into prompt input
- Can be hidden via toggle link

### Prompt Input
- Single-line input field
- Placeholder: "Type @table to select, then describe your query..."
- Focus ring on focus
- Generate button enabled only when tables selected AND prompt has content

### Generation Flow
1. User clicks Generate or presses Enter
2. Close AI overlay
3. Stream generated SQL into editor
4. On complete:
   - If dangerous SQL detected, show warning toast
   - If safe, show success toast and auto-execute

### Cancel
- Escape key closes overlay
- X button closes overlay
- If generation in progress, abort it

---

## Status Bar Enhancements

### Right Side Enhancements
Add keyboard shortcuts as hints:
- `⌘K` AI
- `⌘O` open
- `⌘↵` run

---

## Dialogs and Modals

### Password Prompt Dialog
Modal asking for password to authenticate connection:
- Connection name in title
- Password input (type=password)
- Cancel and Submit buttons
- Error message display on failure

### Open File Dialog
When opening a .sql file, prompt for:
- Connection selection dropdown
- Database selection dropdown
- Open and Cancel buttons

### Expanded Cell Dialog
For viewing/editing large cell values:
- Column name in header
- Copy button
- Pre-formatted content display
- If editing: textarea and save/cancel buttons

### Delete Confirmation Dialog
```
Delete N row(s)?
This action cannot be undone. The selected row(s) will be
permanently deleted from schema.table.
[Cancel] [Delete]
```

### Bulk Edit Dialog
```
Bulk Edit N row(s)
Update a column value for all selected rows in schema.table.

Column: [Dropdown of non-PK columns]
New Value: [Input, placeholder: "Enter new value (type NULL for null)"]

[Cancel] [Update N row(s)]
```

### Batch Preview Dialog
Shows preview of related records across foreign keys for selected rows.

### Connection Dialog Enhancements
Add missing fields:
- Connection Name (required)
- SSL toggle

---

## Keyboard Shortcuts

### Global
| Shortcut | Action |
|----------|--------|
| ⌘O | Open SQL file |
| Escape | Close AI assistant / Cancel edit |

### Query Editor
| Shortcut | Action |
|----------|--------|
| ⌘↵ | Execute query |
| ⌘K | Open AI SQL Assistant |
| ⌘S | Save query to file |

### AI Assistant
| Shortcut | Action |
|----------|--------|
| @ | Trigger table mention |
| ↑/↓ | Navigate suggestions |
| Tab/Enter | Select suggestion |
| Enter | Generate (when ready) |
| Escape | Close |

### Results Table (Cell Editing)
| Shortcut | Action |
|----------|--------|
| Double-click | Enter edit mode |
| ⌘↵ | Save edit |
| Escape | Cancel edit |

---

## State Management Enhancements

### Connection Store Enhancements
Add support for:
- `connections`: Array of connection configs (currently single connection)
- `connectionStatuses`: Map of connectionId -> status
- `treeData`: Hierarchical tree node structure
- `refreshRequest`: Pending refresh for connection/database

### AI Assistant Store
Transient state:
- `isOpen`: Boolean
- `selectedTables`: Array of selected tables
- `isExpandingFk`: Boolean loading flag

### Functions Store
- `manifest`: Loaded function definitions
- `loading`: Boolean
- `error`: Error message
- `searchQuery`: Filter text
- `selectedFunction`: Currently selected function

### Sidebar Store
- `isCollapsed`: Boolean
- `adminSidebarWidth`: Number (for PostCommander route)

### What Persists vs. Clears on Reload
**Persists (settings file):**
- Connection definitions (under `postcommander.connection`)
- Tree expansion state (under `postcommander.expanded_nodes`)
- Sidebar width (under `postcommander.sidebar_width`)
- Editor/results divider height (under `postcommander.editor_height`)
- Tab definitions (id, name, connectionId, database, query) - NOT YET IMPLEMENTED

**Clears on Reload:**
- Query results
- Execution state
- Schema/table data (auto-fetched on reconnect if database node is expanded)
- AI assistant state
- Cell editing state

**Settings File Structure:**
```json
{
  "theme_name": "One Dark",
  "window_bounds": { "x": 0, "y": 0, "width": 1200, "height": 800 },
  "postcommander": {
    "connection": { "host": "localhost", "port": "5432", ... },
    "expanded_nodes": ["server", "database", "schema:public"],
    "sidebar_width": 280.0,
    "editor_height": 250.0
  }
}
```

**Implementation Note:** When the app reconnects with a previously expanded database node, schemas are automatically fetched. This is handled in `connect_to_database()` which checks if the "database" node is already in `expanded_nodes` after successful connection.

---

## Appendix: Data Types

### QueryTab (Enhanced)
```typescript
interface QueryTab {
  id: string
  name: string
  connectionId: string  // Currently missing
  database: string
  query: string
  results?: {
    columns: { name: string; dataType: string; nullable: boolean }[]
    rows: Record<string, unknown>[]
    rowCount: number
    executionTime: number
  }
  tableContext?: {  // Currently missing
    schema: string
    table: string
    primaryKeys: string[]
    foreignKeys?: {
      column: string
      references: { schema: string; table: string; column: string }
    }[]
  }
  isExecuting?: boolean
  error?: string
  autoExecute?: boolean  // Currently missing
}
```

### ConnectionConfig (Enhanced)
```typescript
interface ConnectionConfig {
  id: string  // Currently missing
  name: string  // Currently missing
  host: string
  port: number
  database: string
  user: string
  password: string
  ssl: boolean  // Currently missing
  isLocal?: boolean  // Currently missing
}
```

### SelectedTable (AI)
```typescript
interface SelectedTable {
  schema: string
  name: string
  connectionId: string
  database: string
  isPrimary?: boolean
  viaColumn?: string
}
```
