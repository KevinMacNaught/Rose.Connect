# Performance Patterns

## Quick Index
- **Purpose**: Proven optimization patterns for GPUI applications
- **Key files**: `src/postcommander/sql/completion.rs`, `src/components/data_table/render.rs`, `src/postcommander/theme_colors.rs`
- **When to read**: Before optimizing hot paths, reducing allocations, or improving render performance

## String Allocation Reduction

### Pattern: Cache Lowercase Conversions

**Problem**: Calling `.to_lowercase()` repeatedly in loops allocates new strings each iteration.

**Before:**
```rust
fn matches(label: &str, filter: &str) -> bool {
    let label_lower = label.to_lowercase();
    let filter_lower = filter.to_lowercase();  // Allocates every call!
    label_lower.starts_with(&filter_lower)
}

for item in items.iter() {
    if matches(&item.name, &user_filter) {  // to_lowercase() per iteration
        // ...
    }
}
```

**After:**
```rust
let filter_lower = ctx_info.filter.to_lowercase();  // Allocate once

let matches = |label: &str| -> bool {
    if filter_lower.is_empty() {
        return true;
    }
    let label_lower = label.to_lowercase();
    label_lower.starts_with(&filter_lower)  // Reuse filter_lower
};

for item in items.iter() {
    if matches(&item.name) {  // No repeated allocations
        // ...
    }
}
```

**Location**: `/Users/kevinmacnaught/Repos/Rose.Connect/src/postcommander/sql/completion.rs:187`

**Impact**: Reduced allocations from O(n) to O(1) where n = number of completion items (potentially hundreds per keystroke).

## ElementId Optimization

### Pattern: Integer Encoding vs String Formatting

**Problem**: Creating ElementIds from formatted strings allocates and hashes strings for every cell in a table.

**Before:**
```rust
// Per-cell: allocates String, hashes string
div()
    .id(format!("cell-{}-{}", row_ix, col_ix))
    .child(cell_content)
```

**After:**
```rust
// Per-cell: no allocation, integer hash
div()
    .id(ElementId::Integer((row_ix as u64) << 32 | (col_ix as u64)))
    .child(cell_content)
```

**Encoding scheme:**
- Row IDs: `ElementId::Integer(row_ix as u64)`
- Cell IDs: `ElementId::Integer((row_ix as u64) << 32 | (col_ix as u64))`
  - Upper 32 bits = row index
  - Lower 32 bits = column index

**Location**: `/Users/kevinmacnaught/Repos/Rose.Connect/src/components/data_table/render.rs:278,335`

**Impact**: For a 100x100 table, eliminates 10,000 string allocations + format operations per frame.

## Arc Cloning Strategy

### Pattern: Clone Arc Per-Row, Not Per-Cell

**Problem**: Cloning `Arc<T>` has a small cost (atomic increment). Doing it per-cell when per-row suffices wastes cycles.

**Before:**
```rust
// rows: Arc<Vec<Vec<SharedString>>>
for row_ix in visible_rows {
    for col_ix in 0..columns.len() {
        let rows_clone = rows.clone();  // Arc clone per cell
        div().on_click(move |_, cx| {
            let value = &rows_clone[row_ix][col_ix];
            // ...
        })
    }
}
```

**After:**
```rust
for row_ix in visible_rows {
    let row_data = rows[row_ix].clone();  // Clone Vec<SharedString> once per row

    for col_ix in 0..columns.len() {
        let cell_value = row_data[col_ix].clone();  // Clone SharedString (cheap)
        div().on_click(move |_, cx| {
            // Use cell_value directly
        })
    }
}
```

**Alternative (when row data needed):**
```rust
for row_ix in visible_rows {
    let rows_for_row = rows.clone();  // Arc clone once per row

    for col_ix in 0..columns.len() {
        let rows_for_cell = rows_for_row.clone();  // Still Arc clone, but...
        // Only do this if the closure needs access to ALL rows (rare)
    }
}
```

**Location**: `/Users/kevinmacnaught/Repos/Rose.Connect/src/components/data_table/render.rs`

**Impact**: For a 100-column table, reduces Arc clones from 100 per row to 1 per row.

## Theme Color Extraction

### Pattern: Extract Colors Once Per Render

**Problem**: Calling `cx.theme().colors()` repeatedly accesses a theme registry and clones color values.

**Before:**
```rust
impl Render for MyComponent {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgb(cx.theme().colors().background))  // Theme lookup
            .text_color(rgb(cx.theme().colors().text))  // Theme lookup
            .child(
                div().border_color(rgb(cx.theme().colors().border))  // Theme lookup
            )
    }
}
```

**After (Option 1: Local extraction):**
```rust
impl Render for MyComponent {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();  // Extract once
        div()
            .bg(rgb(colors.background))
            .text_color(rgb(colors.text))
            .child(
                div().border_color(rgb(colors.border))
            )
    }
}
```

**After (Option 2: Dedicated struct for complex components):**
```rust
pub struct RenderColors {
    pub text: u32,
    pub background: u32,
    pub border: u32,
    // ... all colors needed
}

impl RenderColors {
    pub fn from_context(cx: &App) -> Self {
        let colors = cx.theme().colors();
        Self {
            text: colors.text,
            background: colors.background,
            border: colors.border,
            // ...
        }
    }
}

impl Render for MyComponent {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = RenderColors::from_context(cx);
        // Use c.text, c.background, etc.
    }
}
```

**Location**: `/Users/kevinmacnaught/Repos/Rose.Connect/src/postcommander/theme_colors.rs`

**When to use Option 2**: Components with >10 color usages or deeply nested render functions.

**Impact**: Reduces theme registry access from O(color_usages) to O(1).

## State Grouping for Cache Locality

### Pattern: Group Related Fields Into Substates

**Problem**: Large structs with scattered fields reduce cache efficiency and code clarity.

**Before:**
```rust
pub struct PostCommanderPage {
    // Resize-related (9 fields scattered)
    sidebar_width: f32,
    is_resizing_sidebar: bool,
    resize_sidebar_start_x: f32,

    // Dialog-related (6 fields)
    dialog_visible: bool,
    input_host: Entity<TextInput>,

    // Overlay-related (3 fields)
    context_menu: Option<...>,

    // 40+ more fields
}
```

**After:**
```rust
pub struct PostCommanderPage {
    resize: ResizeState,           // 11 fields grouped
    connection_dialog: ConnectionDialogState,  // 6 fields
    overlays: ActiveOverlays,      // 2 fields
    // ... 20 total fields (down from 52)
}

pub(crate) struct ResizeState {
    pub sidebar_width: f32,
    pub is_resizing_sidebar: bool,
    pub resize_sidebar_start_x: f32,
    pub resize_sidebar_start_width: f32,
    // ... all resize state together
}
```

**Benefits:**
1. **Cache locality**: Related fields accessed together are stored together
2. **Cognitive clarity**: `self.resize.sidebar_width` vs `self.sidebar_width`
3. **Default initialization**: Substates can implement `Default` independently
4. **Easier refactoring**: Move entire state group to separate file if needed

**Location**: `/Users/kevinmacnaught/Repos/Rose.Connect/src/postcommander/state.rs`

**When to use**: When you have 3+ fields that are always accessed together (e.g., all resize state, all dialog state).

## Module Organization for Compile Times

### Pattern: Feature Module Extraction

**Problem**: Large single-file modules increase compile times. Changing one function recompiles everything.

**Before:**
```
postcommander/
  page.rs  (1034 lines: SQL parsing, safety, formatting, rendering, state)
```

**After:**
```
postcommander/
  sql/
    mod.rs
    completion.rs    (458 lines: autocomplete logic)
    format.rs        (148 lines: SQL formatting)
    safety.rs        (105 lines: danger detection)
  page.rs            (644 lines: just component logic)
```

**Benefits:**
1. **Incremental compilation**: Changing formatting doesn't recompile completion
2. **Parallel compilation**: rustc can compile sql/ modules in parallel
3. **Clear dependencies**: `mod.rs` shows public API surface

**Pattern in `sql/mod.rs`:**
```rust
mod completion;
mod format;
mod safety;

pub use completion::SqlCompletionProvider;
pub use format::{format_sql, maybe_capitalize_last_word};
pub use safety::{analyze_sql, SqlDangerLevel};
```

**Location**: `/Users/kevinmacnaught/Repos/Rose.Connect/src/postcommander/sql/`

## Checklist: Before Optimizing

- [ ] **Profile first**: Use `cargo flamegraph` or instruments to identify actual bottlenecks
- [ ] **Measure impact**: Add timing logs before/after optimization
- [ ] **Check debug vs release**: Some "slow" code is fast with optimizations enabled
- [ ] **Consider readability cost**: Only optimize hot paths; keep cold paths simple
- [ ] **Verify correctness**: Run tests after performance changes

## Common Anti-Patterns

### Premature Arc Wrapping
```rust
// BAD: Arc overhead when data is small and rarely cloned
struct Settings {
    data: Arc<SettingsData>,  // SettingsData is 3 fields
}

// GOOD: Just clone the small struct
struct Settings {
    data: SettingsData,  // Cheap to clone
}
```

**Rule**: Only use Arc if data is >1KB or cloned >10 times per frame.

### Over-Caching
```rust
// BAD: Storing computed values that are cheap to recompute
struct State {
    items: Vec<Item>,
    items_count: usize,  // Just use items.len()
}
```

**Rule**: Only cache if computation is >1ms or done >100 times per frame.

### String Over-Engineering
```rust
// BAD: Optimizing strings that allocate once at startup
const LABEL: &str = "Save";  // Was String
```

**Rule**: Only optimize strings in loops or per-frame code.

## See Also

- `docs/refactoring-patterns.md` - File splitting patterns
- `src/components/data_table/render.rs` - Real-world example of ElementId + Arc optimization
- `src/postcommander/sql/completion.rs` - String allocation optimization
