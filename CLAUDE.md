# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Important: Knowledge Cutoff

It is December 2025. Libraries, APIs, and best practices may have changed since your knowledge cutoff. If you are unsure about current syntax, features, or whether something exists, use WebSearch or WebFetch to verify before making assumptions.

## Build Commands

```bash
cargo build          # Build the project
cargo run            # Run the application
cargo check          # Type check without building
cargo test           # Run tests
cargo test test_name # Run a specific test
cargo watch -x run -i data/ -i docs/  # Auto-rebuild on changes
```

**Cargo Watch Gotcha:** If your app writes files to the project directory (like `data/settings.json`), cargo watch will detect the change and trigger a rebuild/relaunch. Use `-i data/` to ignore the data directory.

## Project Overview

This is a Rust desktop application using GPUI, the GPU-accelerated UI framework from Zed Industries. GPUI is pulled directly from the Zed repository as a git dependency.

## File Size Guidelines

**Keep files under 300-500 lines** depending on complexity:
- **~300 lines**: Start thinking about structure
- **~500 lines**: Actively look for ways to split
- **800+ lines**: Stop and refactor immediately

UI-heavy render code (lots of `.child()` chains) should lean toward 300. Business logic can stretch to 500.

**How to split GPUI component files:**

Rust allows `impl` blocks to be split across multiple files in the same module:
```
component/
  mod.rs              # Module declarations
  types.rs            # Shared types (structs, enums)
  page.rs             # Main struct, core logic, Render impl
  sidebar.rs          # impl Component { fn render_sidebar... }
  dialog.rs           # impl Component { fn render_dialog... }
  ui_helpers.rs       # Standalone helper functions
```

Key patterns:
1. **Types first**: Extract shared types to `types.rs`
2. **Static helpers**: Move pure functions that don't need `&self` to `ui_helpers.rs`
3. **Render methods**: Group related render methods into separate files with their own `impl` blocks
4. **Use `pub(crate)`**: Fields accessed by other files in the module need `pub(crate)` visibility

---

## Documentation Index

Read the relevant docs before starting work in each area:

| Working on... | Read these docs |
|---------------|-----------------|
| **UI components, layouts, elements** | `docs/gpui-guide.md` |
| **Theme system, colors, styling** | `docs/gpui-theming.md`, `docs/component-theming-rules.md` |
| **macOS features (menus, titlebar)** | `docs/macos-app-guide.md` |
| **Looking for GPUI examples** | `docs/zed-reference.md` |
| **PostCommander database UI** | `docs/postcommander-ui-specification.md` |

### Quick Reference

**Theme colors** - Always use semantic tokens from the theme:
```rust
let theme = cx.theme();
let colors = theme.colors();
div().bg(rgb(colors.background)).text_color(rgb(colors.text))
```

**Icons** - Use Lucide icons from `assets/icons/`:
```rust
use crate::icons::icon_sm;
div().child(icon_sm("search", colors.text_muted))
```

**Scrollable containers** - Need `.id()`, `.min_h_0()`, and `.overflow_y_scroll()`. See `docs/gpui-guide.md`.

**Modal dialogs** - Use `deferred()` and `.occlude()` for z-ordering and event handling. See `docs/gpui-guide.md`.

**Text inputs** - Use `crate::components::TextInput`. See `docs/gpui-guide.md` for usage.

**Code editors** - Use `gpui_component::input::{Input, InputState}` with `.code_editor("sql")`. See `docs/gpui-guide.md`.

**gpui-component widgets** - Button, Switch, Checkbox, Slider, Progress, Badge, Avatar, Spinner, Calendar, Select, Charts. See `docs/gpui-guide.md` § "gpui-component Library".

**Custom dropdowns** - For multi-column selects or complex dropdowns, use relative/absolute positioning with `deferred()`. See `docs/gpui-guide.md` § "Custom Dropdown/Select Components" and `src/components_test.rs`.

**DataTable component** - Reusable data table with scrolling data grids. See `src/components/data_table.rs`. Supports:
- Manual horizontal/vertical scrolling (GPUI's overflow_scroll doesn't work for horizontal)
- Row virtualization for performance (only renders visible rows)
- Column resizing via drag handles
- Text truncation with ellipsis
- PK indicators in column headers
- See `docs/gpui-guide.md` § "Horizontal Scrolling" and "Column Resizing with Drag"
