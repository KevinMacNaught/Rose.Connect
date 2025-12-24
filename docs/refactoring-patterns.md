# Refactoring Patterns

## Quick Index
- **Purpose**: Patterns for splitting large GPUI components and modules
- **Key examples**: PostCommander (1034→644 lines), DataTable (865→400 lines)
- **When to read**: Before splitting files >500 lines, or when adding features to existing split modules

## File Size Triggers

**Act on these thresholds:**
- **~300 lines**: Start thinking about structure
- **~500 lines**: Actively look for ways to split
- **800+ lines**: Stop and refactor immediately

UI-heavy render code (lots of `.child()` chains) should lean toward 300. Business logic can stretch to 500.

## Pattern 1: Component Method Extraction (PostCommanderPage)

When a single component file grows large, extract cohesive groups of methods into separate files using Rust's `impl` block splitting.

### Structure

```
postcommander/
  mod.rs                    # Module declarations, pub use exports
  types.rs                  # Shared types, enums, structs
  page.rs                   # Main struct, constructor, Render impl
  query_execution.rs        # impl PostCommanderPage { fn execute_query... }
  resize_handlers.rs        # impl PostCommanderPage { fn start_resize... }
  dialogs.rs                # impl PostCommanderPage { fn render_dialog... }
  sidebar.rs                # impl PostCommanderPage { fn render_sidebar... }
  ui_helpers.rs             # Pure functions (no &self access)
```

### Key Rules

1. **Types first**: Extract shared types to `types.rs` before splitting impl blocks
2. **Field visibility**: Use `pub(crate)` on struct fields that other impl files need
3. **One impl per file**: Group related methods by feature (resize, dialogs, rendering)
4. **Pure functions separate**: Static helpers go in `ui_helpers.rs`

### Example: PostCommanderPage Split

**Before (1034 lines in `page.rs`):**
```rust
pub struct PostCommanderPage {
    sidebar_width: f32,
    tabs: Vec<QueryTab>,
    // ... 50 more fields
}

impl PostCommanderPage {
    pub fn new(...) { ... }
    fn execute_query(&mut self, ...) { ... }      // 150 lines
    fn render_safety_dialog(...) { ... }           // 100 lines
    fn start_resize(&mut self, ...) { ... }        // 120 lines
    fn render(&mut self, ...) { ... }              // 500 lines
}
```

**After:**

`page.rs` (644 lines):
```rust
pub struct PostCommanderPage {
    pub(crate) sidebar_width: f32,  // pub(crate) for access from impl files
    pub(crate) tabs: Vec<QueryTab>,
    // ...
}

impl PostCommanderPage {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Constructor only
    }
}

impl Render for PostCommanderPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Main render logic
    }
}
```

`query_execution.rs` (201 lines):
```rust
use super::*;

impl PostCommanderPage {
    pub(crate) fn execute_query(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Query execution logic with safety checks
    }

    pub(crate) fn handle_query_result(&mut self, ...) {
        // ...
    }
}
```

`resize_handlers.rs` (122 lines):
```rust
use super::*;

impl PostCommanderPage {
    pub(crate) fn start_resize(&mut self, x: Pixels, cx: &mut Context<Self>) {
        // ...
    }

    pub(crate) fn update_resize(&mut self, x: Pixels, cx: &mut Context<Self>) {
        // ...
    }
}
```

`dialogs.rs` (103 lines):
```rust
use super::*;

impl PostCommanderPage {
    pub(crate) fn render_safety_dialog(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        // Modal dialog rendering
    }
}
```

### mod.rs Pattern

```rust
// Declare all submodules
mod cell_edit;
mod connection_dialog;
pub mod database;       // pub if used outside the module
mod dialogs;
mod page;
mod query_execution;    // Private, only used by page.rs
mod resize_handlers;
mod types;
pub mod ui_helpers;

// Re-export the main struct
pub use page::PostCommanderPage;
```

## Pattern 2: Directory Module with Render Struct (DataTable)

When a component has distinct responsibilities (rendering, event handling, state), convert it to a directory module with focused files.

### Structure

```
components/
  data_table/
    mod.rs              # Module exports
    types.rs            # DataTableColumn, DataTableState, events
    render.rs           # DataTable struct, Render impl
    fk_card.rs          # Popup card rendering logic
    resize.rs           # Column resize drag handlers
```

### Key Rules

1. **Separate state and render**: State entity (`DataTableState`) in `types.rs`, render struct (`DataTable`) in `render.rs`
2. **Feature isolation**: Each file handles one feature (FK cards, resizing)
3. **Minimal exports**: Only export what consumers need via `mod.rs`
4. **Arc for shared data**: Use `Arc<Vec<T>>` for data shared between closures

### Example: DataTable Split

**Before (865 lines in `data_table.rs`):**
```rust
// All in one file
pub struct DataTableColumn { ... }
pub struct DataTableState { ... }
pub struct DataTable { ... }

impl DataTableState {
    fn on_scroll(...) { ... }
    fn resize_column(...) { ... }
    fn render_fk_card(...) { ... }
}

impl Render for DataTable {
    fn render(...) -> impl IntoElement {
        // 400 lines of render logic
    }
}
```

**After:**

`types.rs` (284 lines):
```rust
use std::sync::Arc;

#[derive(Clone)]
pub struct DataTableColumn {
    pub name: SharedString,
    pub width: Pixels,
    pub is_pk: bool,
}

pub struct DataTableState {
    pub columns: Vec<DataTableColumn>,
    pub rows: Arc<Vec<Vec<SharedString>>>,  // Arc for cheap cloning
    pub scroll_offset: Point<Pixels>,
    pub fk_card: Option<FkCardData>,
    // ... other state
}

// Event types
#[derive(Debug, Clone)]
pub struct CellSaveRequested {
    pub row_index: usize,
    pub col_index: usize,
    pub new_value: String,
}

impl EventEmitter<CellSaveRequested> for DataTableState {}
```

`render.rs` (400 lines):
```rust
use super::types::*;
use super::resize::render_resize_handle;
use super::fk_card::render_fk_card;

pub struct DataTable {
    state: Entity<DataTableState>,
}

impl DataTable {
    pub fn new(state: Entity<DataTableState>) -> Self {
        Self { state }
    }

    fn render_header(&self, ...) -> impl IntoElement { ... }
    fn render_rows(&self, ...) -> impl IntoElement { ... }
}

impl Render for DataTable {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        // Main rendering orchestration
    }
}
```

`fk_card.rs` (228 lines):
```rust
use super::types::*;

pub fn render_fk_card(
    card: &FkCardData,
    state: Entity<DataTableState>,
    cx: &Context<DataTable>,
) -> impl IntoElement {
    // Isolated FK card rendering
}
```

`resize.rs` (73 lines):
```rust
use super::types::*;

pub fn render_resize_handle(
    col_index: usize,
    state: Entity<DataTableState>,
) -> impl IntoElement {
    // Column resize drag handlers
}
```

`mod.rs`:
```rust
mod fk_card;
mod render;
mod resize;
mod types;

// Only export what consumers need
pub use render::DataTable;
pub use types::{
    CellSaveRequested,
    DataTableColumn,
    DataTableState,
    FkDataRequest,
};
```

## Pattern 3: Feature Module Extraction

Extract cohesive features into standalone modules (not just impl blocks).

### Examples from PostCommander

**SQL Safety Detection** (`sql_safety.rs`, 105 lines):
```rust
pub enum SqlDangerLevel {
    Safe,
    ModifyData,
    DestructiveSchema,
}

pub struct SqlDangerInfo {
    pub level: SqlDangerLevel,
    pub message: String,
    pub affected_tables: Vec<String>,
}

pub fn analyze_sql_safety(sql: &str, table_names: &[String]) -> SqlDangerInfo {
    // Pure function, no dependencies on PostCommanderPage
}
```

**SQL Autocomplete** (`sql_completion.rs`, 458 lines):
```rust
use gpui_component::input::CompletionProvider;

pub struct SqlCompletionProvider {
    schemas: Rc<RefCell<SchemaMap>>,
    structures: Rc<RefCell<Vec<TableStructureInfo>>>,
}

impl CompletionProvider for SqlCompletionProvider {
    fn completions(...) -> Task<Result<CompletionResponse>> {
        // Complex autocomplete logic isolated
    }
}
```

**Database Operations** (`database.rs`, 269 lines):
```rust
pub struct DatabaseManager {
    pool: Option<Pool>,
}

impl DatabaseManager {
    pub async fn connect(&mut self, config: &ConnectionConfig) -> Result<()> { ... }
    pub async fn execute_query(&self, sql: &str) -> Result<QueryResult> { ... }
    pub async fn fetch_schemas(&self) -> Result<SchemaMap> { ... }
}
```

### When to Extract Features

Extract when:
- Feature has >100 lines of logic
- Feature could be tested independently
- Feature has its own types/enums
- Feature doesn't need mutable access to main component state

Keep in main component when:
- Logic is <50 lines
- Tightly coupled to UI state
- Just a render helper

## Performance Considerations

### Before Refactoring
PostCommanderPage had a 100μs polling loop causing 10,000 redraws/sec. During refactoring, this was removed.

### After Refactoring Optimizations
1. **Connection info caching**: Getters return `&str` instead of cloning strings
2. **Arc wrapping**: `Arc<HashMap<...>>` for foreign_keys avoids cloning per row
3. **Arc<SchemaMap>**: Sidebar renders without cloning schema data
4. **TabId newtype**: `Copy` type instead of String allocations

### Pattern: Arc for Render Data

When data is used in render closures (like `uniform_list`), wrap in Arc:

```rust
// Before: Clones entire Vec on every frame
pub struct QueryResult {
    pub rows: Vec<Vec<String>>,
}

// After: Arc clone = pointer increment
pub struct QueryResult {
    pub rows: Arc<Vec<Vec<SharedString>>>,
}
```

## Type Safety Improvements

### Newtype Pattern: TabId

**Before (panic-prone):**
```rust
pub struct PostCommanderPage {
    tabs: Vec<QueryTab>,
    active_tab: Option<String>,  // Could be any string
}

fn get_active_tab(&self) -> &QueryTab {
    let id = self.active_tab.as_ref().expect("no active tab");  // PANIC!
    self.tabs.iter().find(|t| &t.id == id).expect("tab not found")
}
```

**After (type-safe):**
```rust
use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TabId(NonZeroU64);

impl TabId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(NonZeroU64::new(COUNTER.fetch_add(1, Ordering::Relaxed)).unwrap())
    }
}

pub struct PostCommanderPage {
    tabs: Vec<QueryTab>,
    active_tab_id: Option<TabId>,
}

fn get_active_tab(&self) -> Option<&QueryTab> {
    let id = self.active_tab_id?;
    self.tabs.iter().find(|t| t.id == id)
}
```

Benefits:
- Copy instead of Clone (no allocations)
- Type-safe (can't pass random strings)
- Forces Option handling (no expect() panics)

## Checklist for Splitting Files

When refactoring a large component:

- [ ] Extract shared types to `types.rs` first
- [ ] Make struct fields `pub(crate)` if accessed by impl files
- [ ] Group related methods into cohesive impl blocks
- [ ] Move pure functions to `ui_helpers.rs`
- [ ] Create `mod.rs` with minimal pub exports
- [ ] Update imports in files using the component
- [ ] Run `cargo check` after each file split
- [ ] Verify no performance regressions
- [ ] Consider Arc wrapping for large data structures
- [ ] Replace String IDs with Copy newtypes where appropriate

## See Also

- `/Users/kevinmacnaught/Repos/Rose.Connect/src/postcommander/` - Complete example of Pattern 1
- `/Users/kevinmacnaught/Repos/Rose.Connect/src/components/data_table/` - Complete example of Pattern 2
- `/Users/kevinmacnaught/Repos/Rose.Connect/CLAUDE.md` § "File Size Guidelines" - When to refactor
