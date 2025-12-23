# GPUI Development Guide

This guide covers GPUI patterns, common gotchas, and UI implementation details.

## Basic Patterns

- Components implement the `Render` trait with a `render` method returning `impl IntoElement`
- Use `div()` and builder-pattern methods for layout (`.flex()`, `.flex_col()`, `.size_full()`, etc.)
- Colors use `rgb(0xHHHHHH)` format
- Text content uses `SharedString` for efficient sharing
- App lifecycle: `Application::new().run()` with a closure receiving `&mut App`
- Windows created via `cx.open_window(WindowOptions, |window, cx| { ... })`
- State managed through `cx.new(|cx| YourComponent { ... })`

## Common Gotchas

### Context Borrowing
- `update_global()` requires `use gpui::BorrowAppContext` trait import
- `cx.refresh_windows()` refreshes all windows - use after changing global state like themes
- Can't borrow context mutably and immutably at same time in `cx.new()` closures - extract data before the closure:
  ```rust
  // WRONG - borrows cx twice
  let picker = cx.new(|_| ThemePicker::new(cx));

  // RIGHT - extract data first
  let themes = ThemeRegistry::global(cx).list();
  let current = AppSettings::get_global(cx).theme_name.clone();
  let picker = cx.new(|_| ThemePicker::new(themes, &current));
  ```

### Text Truncation in Flex Containers
When text may overflow in a constrained flex container (like sidebar items), apply these styles to truncate with ellipsis:

```rust
div()
    .flex()
    .items_center()
    .gap_2()
    .child(icon_sm("table", text_muted))  // Icon won't shrink (has flex_shrink_0)
    .child(
        div()
            .min_w_0()           // Allow shrinking below content size
            .overflow_hidden()   // Hide overflow
            .whitespace_nowrap() // Keep on one line
            .text_ellipsis()     // Show "..." when truncated
            .text_sm()
            .child(long_table_name)
    )
```

Key points:
- `min_w_0()` on the text container lets it shrink below its natural content width
- Without it, flex items default to `min-width: auto` and won't shrink
- Combine with `flex_shrink_0()` on icons to ensure they never shrink while text truncates

### Scrollable Containers
Require THREE things to work in a flex layout:
```rust
// Parent containers in the flex chain also need min_h_0()!
div()
    .flex_1()
    .min_h_0()  // Parent must allow shrinking too
    .flex()
    .child(
        div()
            .id("my-scroll-container")  // 1. REQUIRED - makes div stateful
            .flex_1()
            .min_h_0()                  // 2. REQUIRED - allows shrinking below content size
            .overflow_y_scroll()        // 3. Enable vertical scrolling
            .children(...)
    )
```
- Without `.id()`, scrolling won't work (div must be stateful)
- Without `.min_h_0()` on EVERY flex container in the chain, scrolling won't activate
- Flexbox default is `min-height: auto` which prevents shrinking below content size

### Drag and Drop
- Requires elements to have an `.id()` to be stateful
- `on_drag(value, constructor)` - the value must be `Clone + 'static`, constructor receives `(&T, Point<Pixels>, &mut Window, &mut App)` and returns an `Entity<W>` for the drag preview
- `on_drop()` must use `cx.listener(|this, dropped: &T, window, cx| {...})` to access component state for mutation
- `drag_over::<T>(|style, data, window, app| style)` - the type parameter specifies which drag type to respond to; closure needs `move` keyword to capture variables
- After mutating state in `on_drop`, call `cx.notify()` to trigger a re-render

### Element Types
- Adding `.id()` to a `Div` transforms it to `Stateful<Div>` - return `impl IntoElement` instead of `Div` from helper functions
- `rgba(hex)` takes only ONE argument (hex includes alpha as 0xRRGGBBAA), NOT `rgba(color, alpha)`
- `linear_gradient(angle, from, to)` requires `LinearColorStop` types, not raw `Rgba` - use `linear_color_stop(color, percentage)`
- `when_some()` and other fluent builder methods require `use gpui::prelude::FluentBuilder` trait import
- Structs used in closures (like `on_drag`) need `#[derive(Clone, Copy)]` to avoid move errors in `Fn` closures
- `Pixels` inner field is private - use `f32::from(pixels)` to convert, not `pixels.0`

### Window and Lifecycle
- `observe_window_bounds` must be called on `Context<T>` (inside a component), not on `App`:
  ```rust
  cx.observe_window_bounds(window, |_this, window, cx| {
      let bounds = window.bounds();
      // Save bounds to settings...
  }).detach();
  ```
- `on_window_closed` is on `App`, fires when any window closes

### Async Operations
`cx.spawn()` uses a special async closure syntax - the `async move` goes BEFORE the closure parameters:
```rust
// CORRECT
cx.spawn(async move |this, cx| {
    let result = some_future.await;
    let _ = this.update(cx, |this, cx| {
        this.state = result;
        cx.notify();
    });
}).detach();

// WRONG - lifetime/type errors
cx.spawn(|this, cx| async move { ... })
```
The `this` parameter is a `WeakEntity<T>` and `cx` is `&mut AsyncApp`. Use `this.update(cx, |this, cx| { ... })` to modify component state.

### Theme Color Extraction
When using `cx.theme()` in render methods, extract ALL color values to local variables BEFORE calling methods that need `&mut Context<Self>` (like `cx.listener()`):
```rust
// CORRECT - extract colors first
fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = cx.theme();
    let colors = theme.colors();
    let text = colors.text;
    let element_hover = colors.element_hover;

    div()
        .text_color(rgb(text))
        .hover(move |s| s.bg(rgb(element_hover)))
        .on_click(cx.listener(|this, _, _, cx| { ... }))
}

// WRONG - colors borrows cx while cx.listener needs mutable
div()
    .hover(move |s| s.bg(rgb(colors.element_hover)))
    .on_click(cx.listener(...))  // ERROR
```

## Lucide Icons

This project includes Lucide icons in `assets/icons/`. Always use these instead of text characters.

```rust
use crate::icons::icon_sm;

// icon_sm(name, color) - 16px
// icon_md(name, color) - 20px
// icon_lg(name, color) - 24px

div().child(icon_sm("search", colors.text_muted))
```

**Gotcha: Icons Shrinking in Flex Containers**

Icons may shrink when placed in flex containers with constrained width. The icon helpers already include `flex_shrink_0()` to prevent this, but if you're using raw `svg()` calls, add it manually:

```rust
svg()
    .path("assets/icons/search.svg")
    .size(px(16.))
    .flex_shrink_0()  // Prevents icon from shrinking
    .text_color(rgb(color))
```

| Instead of | Use icon |
|------------|----------|
| "🔍" | `search` |
| "▸" or ">" | `chevron-right` |
| "▾" or "▼" | `chevron-down` |
| "✓" | `check` |
| "✕" | `x` |
| "+" | `plus` |
| "⚙" | `settings` |
| "🗑" | `trash-2` |
| "✏" | `pencil` |

See `AVAILABLE_ICONS` in `src/icons.rs` or check `assets/icons/` directory.

## Modal Dialogs

Use `deferred()` to render modals on top of other content:
```rust
.when(show_modal, |el| {
    el.child(deferred(
        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .id("backdrop")
                    .absolute()
                    .inset_0()
                    .bg(rgba(0x000000aa))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.show_modal = false;
                        cx.notify();
                    }))
            )
            .child(
                div()
                    .relative()
                    .w(px(600.))
                    // ... modal content
            )
    ))
})
```

- `deferred()` delays painting so element renders on top
- Backdrop needs `.id()` to be clickable
- `when()` requires `use gpui::prelude::FluentBuilder`

## Resizable Panels

Use an overlay during drag to capture mouse events globally:
```rust
pub struct MyComponent {
    sidebar_width: f32,
    is_resizing: bool,
    resize_start_x: f32,
    resize_start_width: f32,
}

fn render_resize_handle(&self, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = cx.theme();
    let colors = theme.colors();
    let border_variant = colors.border_variant;
    let accent = colors.accent;
    let is_resizing = self.is_resizing;

    div()
        .id("resize-handle")
        .w(px(4.))
        .h_full()
        .cursor_col_resize()
        .bg(transparent_black())
        .when(is_resizing, |el| el.bg(rgb(accent)))
        .hover(move |s| s.bg(rgb(border_variant)))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, event: &MouseDownEvent, _, cx| {
                this.is_resizing = true;
                this.resize_start_x = f32::from(event.position.x);
                this.resize_start_width = this.sidebar_width;
                cx.notify();
            }),
        )
}

fn render_resize_overlay(&self, cx: &mut Context<Self>) -> impl IntoElement {
    div()
        .id("resize-overlay")
        .absolute()
        .inset_0()
        .cursor_col_resize()
        .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
            if this.is_resizing {
                let delta = f32::from(event.position.x) - this.resize_start_x;
                let new_width = (this.resize_start_width + delta).clamp(180.0, 500.0);
                this.sidebar_width = new_width;
                cx.notify();
            }
        }))
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|this, _, _, cx| {
                this.is_resizing = false;
                this.save_sidebar_width(cx);  // Persist on resize end
                cx.notify();
            }),
        )
}

// Persist width to settings
fn save_sidebar_width(&self, cx: &mut Context<Self>) {
    AppSettings::update_global(cx, |settings| {
        settings.postcommander_mut().sidebar_width = Some(self.sidebar_width);
    });
    AppSettings::get_global(cx).save();
}

// In render():
div()
    .child(sidebar.w(px(self.sidebar_width)))
    .child(self.render_resize_handle(cx))
    .child(main_content)
    .when(self.is_resizing, |el| el.child(self.render_resize_overlay(cx)))
```

- Overlay essential because mouse events only go to elements under cursor
- Use `f32::from(event.position.x)` to convert `Pixels`
- Show visual feedback on handle during resize

## Text Input Component

GPUI doesn't have a built-in text input widget. This project includes a custom `TextInput` component in `src/components/text_input.rs` based on GPUI's input example.

### Usage

```rust
use crate::components::TextInput;

// Create in component initialization
let input = cx.new(|cx| {
    let mut input = TextInput::new(cx, "placeholder text");
    input.set_content("initial value");
    input.set_masked(true);  // For password fields
    input.set_colors(text_color, placeholder_color);  // Theme colors
    input
});

// Render - just include the entity, it renders itself with all handlers
div()
    .child(input.clone())

// Read value
let value = input.read(cx).content().to_string();
```

### Key Bindings Required

TextInput requires key bindings to be registered in `main.rs`:

```rust
cx.bind_keys([
    KeyBinding::new("backspace", components::Backspace, Some("TextInput")),
    KeyBinding::new("delete", components::Delete, Some("TextInput")),
    KeyBinding::new("left", components::Left, Some("TextInput")),
    KeyBinding::new("right", components::Right, Some("TextInput")),
    KeyBinding::new("shift-left", components::SelectLeft, Some("TextInput")),
    KeyBinding::new("shift-right", components::SelectRight, Some("TextInput")),
    KeyBinding::new("cmd-a", components::SelectAll, Some("TextInput")),
    KeyBinding::new("cmd-v", components::Paste, Some("TextInput")),
    KeyBinding::new("cmd-c", components::Copy, Some("TextInput")),
    KeyBinding::new("cmd-x", components::Cut, Some("TextInput")),
    KeyBinding::new("home", components::Home, Some("TextInput")),
    KeyBinding::new("end", components::End, Some("TextInput")),
]);
```

### Features

- Click to position cursor
- Click and drag to select text
- Shift+arrows to extend selection
- Cmd+A to select all
- Delete/Backspace to delete selected text
- Cmd+C/V/X for copy/paste/cut
- Password masking with `set_masked(true)`
- IME support (marked text)

### Architecture

The component has two parts:
1. `TextInput` - The entity that holds state and implements `Render` with all event handlers
2. `TextInputElement` - The low-level `Element` impl that handles text layout and painting

When you render a `TextInput` entity, its `Render` impl automatically includes the `TextInputElement` and all necessary handlers. Don't use `TextInputElement` directly unless you need custom rendering.

### Common Gotcha: Colors Must Be Set Before Render

Colors are stored on the `TextInput` struct. If using theme colors, update them before/during render:

```rust
fn render_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = cx.theme();
    let text = theme.colors().text;
    let muted = theme.colors().text_muted;

    // Update colors on the input entity
    self.my_input.update(cx, |input, _| {
        input.set_colors(text, muted);
    });

    // Then render
    div().child(self.my_input.clone())
}
```

### Gotcha: Masked Mode and Byte Index Conversion

When `set_masked(true)` is used (for password fields), the display text uses bullet characters ("•") which are 3 bytes each in UTF-8. However, cursor and selection indices are byte offsets into the actual content.

The TextInput component handles this internally with `content_index_to_display_index()` and `display_index_to_content_index()` helper methods. If you're extending TextInput or building a similar component, you must convert indices when:
- Rendering cursor/selection positions (content → display)
- Processing mouse click positions (display → content)
- IME bounds calculation (content → display)
- Marked text runs for underlines (content → display)

Example: If content is "ab" (2 bytes) and display is "••" (6 bytes), cursor at byte 1 in content should be at byte 3 in display.

## Code Editor (gpui-component Input)

For multi-line code editing with syntax highlighting, use `gpui-component`'s `Input` and `InputState`:

```rust
use gpui_component::input::{Input, InputState};

// Create the editor state (in component init or add_tab())
let editor = cx.new(|cx| {
    InputState::new(window, cx)
        .code_editor("sql".to_string())  // Language: sql, rust, javascript, etc.
        .line_number(true)               // Show line numbers
        .soft_wrap(true)                  // Wrap long lines
        .default_value("SELECT * FROM ") // Initial content
        .placeholder("Enter SQL query...")
});

// Render the editor
Input::new(&editor)
    .bordered(false)
    .p(px(8.))
    .h_full()
    .font_family("monospace")

// Read content
let sql = editor.read(cx).value().to_string();
```

### Supported Languages

Enable `tree-sitter-languages` feature in Cargo.toml:
```toml
gpui-component = { git = "...", features = ["tree-sitter-languages"] }
```

Languages: `sql`, `rust`, `javascript`, `typescript`, `python`, `go`, `html`, `css`, `json`, `yaml`, `markdown`, and more.

### Theme Sync

The code editor uses gpui-component's `HighlightTheme`. Sync it with your app's theme in `sync_to_gpui_component()`:

```rust
use gpui_component::highlighter::HighlightTheme;

// Syntax highlighting theme
let highlight_theme = if theme.appearance.is_light() {
    HighlightTheme::default_light()
} else {
    HighlightTheme::default_dark()
};
gpui_theme.highlight_theme = highlight_theme;

// IMPORTANT: Set caret color or cursor will be invisible!
gpui_theme.caret = hex_to_hsla(colors.text);
```

**Gotcha: Invisible Cursor** - If the cursor doesn't appear in the editor, you forgot to set `gpui_theme.caret`. The cursor color defaults to something that may be invisible against your background.

### Gotcha: InputState::new() Requires Window

`InputState::new(window, cx)` requires a `&mut Window` parameter. In GPUI, the `Window` is separate from `Context<T>`. You may need to update function signatures to pass `window`:

```rust
// Before
fn add_tab(&mut self, cx: &mut Context<Self>) { ... }

// After - pass window through
fn add_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
    let editor = cx.new(|cx| InputState::new(window, cx)...);
}

// Update call sites in on_click handlers:
.on_click(cx.listener(|this, _, window, cx| {  // window is 3rd param
    this.add_tab(window, cx);
}))
```

## Modal Dialog Event Propagation

When creating modal dialogs, use `.occlude()` on the dialog panel to prevent clicks from propagating to the backdrop:

```rust
deferred(
    div()
        .absolute()
        .inset_0()
        .child(
            // Backdrop - clicking closes modal
            div()
                .id("backdrop")
                .absolute()
                .inset_0()
                .bg(rgba(0x00000088))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.show_modal = false;
                    cx.notify();
                }))
        )
        .child(
            // Dialog panel - clicks don't propagate to backdrop
            div()
                .id("dialog")
                .occlude()  // <-- This stops event propagation
                .w(px(400.))
                .p_4()
                .rounded_xl()
                .bg(rgb(background))
                // ... dialog content
        )
)
```

Without `.occlude()`, clicking anywhere on the dialog would close it because the click would propagate to the backdrop underneath.

## Virtualized Lists (uniform_list)

For scrollable lists with many items (100+), use GPUI's `uniform_list` to only render visible rows:

```rust
use gpui::uniform_list;

uniform_list(
    "my-list-id",           // Element ID
    items.len(),            // Total item count
    |range, _window, _cx| { // Render only items in visible range
        range
            .map(|ix| render_row(&items[ix]))
            .collect::<Vec<_>>()
    },
)
.flex_1()
.min_h_0()  // Required for scrolling in flex container
```

### Critical: Avoid Cloning Data Every Frame

The `uniform_list` callback must be `'static`, so you can't borrow from `self`. **Do NOT clone your data into the closure**:

```rust
// WRONG - clones ALL rows on every frame (slow!)
.when_some(result.clone(), |el, res| {
    let rows = res.rows.clone();  // Cloning Vec<Vec<String>> = megabytes!
    el.child(uniform_list("rows", rows.len(), move |range, _, _| {
        range.map(|i| render_row(&rows[i])).collect()
    }))
})
```

### Solution: Arc + SharedString

Store list data in `Arc<Vec<...>>` with pre-computed `SharedString` values:

```rust
use std::sync::Arc;
use gpui::SharedString;

// In your data struct
pub struct QueryResult {
    pub rows: Arc<Vec<Vec<SharedString>>>,  // Arc clone = pointer increment
}

// When creating data, convert to SharedString ONCE
let rows: Vec<Vec<SharedString>> = raw_rows
    .iter()
    .map(|row| row.iter().map(|cell| SharedString::from(cell.display())).collect())
    .collect();
let result = QueryResult { rows: Arc::new(rows) };

// In render - Arc clone is cheap!
.when_some(result.clone(), |el, res| {
    let rows = res.rows.clone();  // Just increments ref count
    el.child(uniform_list("rows", rows.len(), move |range, _, _| {
        range.map(|i| render_row(&rows[i])).collect()
    }))
})
```

Why this matters:
- `Arc::clone()` = increment counter (nanoseconds)
- `SharedString::clone()` = increment counter (nanoseconds)
- `Vec<Vec<String>>::clone()` = allocate + copy all bytes (milliseconds with large data)

### Simplified Row Rendering

For maximum performance, keep row rendering minimal:

```rust
fn render_row(row: &[SharedString], bg: u32, text: u32) -> impl IntoElement {
    div()
        .flex()
        .h(px(32.))  // Fixed height required for uniform_list
        .bg(rgb(bg))
        .children(row.iter().map(|cell| {
            div()
                .w(px(150.))
                .h(px(32.))
                .px_3()
                .text_sm()
                .text_color(rgb(text))
                .child(cell.clone())  // SharedString clone is cheap
        }))
}
```

Avoid in hot paths:
- `.hover()` effects (tracking state per row)
- Zebra striping calculations
- Deep nesting of divs
- Theme lookups inside the callback (extract colors before)

## Context Menus (Right-Click Menus)

**Important**: Do NOT use `gpui-component`'s `ContextMenuExt` trait - it has visual artifacts (black shadows). Instead, implement context menus the Zed way:

### State Setup

Store the menu, position, and dismiss subscription in your component:
```rust
use gpui_component::menu::PopupMenu;

pub struct MyComponent {
    context_menu: Option<(Entity<PopupMenu>, Point<Pixels>, Subscription)>,
    // ...
}
```

### Deploy on Right-Click

```rust
pub fn deploy_context_menu(
    &mut self,
    position: Point<Pixels>,
    window: &mut Window,
    cx: &mut Context<Self>,
) {
    use gpui_component::menu::PopupMenuItem;

    let entity = cx.entity().downgrade();

    let menu = PopupMenu::build(window, cx, move |menu, _window, _cx| {
        let entity = entity.clone();
        menu.item(
            PopupMenuItem::new("Do Something")
                .on_click(move |_, window, cx| {
                    if let Some(this) = entity.upgrade() {
                        this.update(cx, |this, cx| {
                            this.do_something(window, cx);
                        });
                    }
                }),
        )
    });

    let subscription = cx.subscribe(&menu, |this, _, _: &DismissEvent, cx| {
        this.context_menu = None;
        cx.notify();
    });

    self.context_menu = Some((menu, position, subscription));
    cx.notify();
}
```

### Trigger from Element

```rust
div()
    .id("my-element")
    .on_mouse_down(
        MouseButton::Right,
        cx.listener(|this, event: &MouseDownEvent, window, cx| {
            this.deploy_context_menu(event.position, window, cx);
        }),
    )
```

### Render the Menu

In your `Render` impl, render the menu using `deferred` and `anchored` with a **full-window occlude overlay**:
```rust
fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let context_menu = self.context_menu.as_ref().map(|(menu, pos, _)| (menu.clone(), *pos));

    div()
        // ... your content ...
        .when_some(context_menu, |el, (menu, position)| {
            let window_size = window.bounds().size;
            el.child(
                deferred(
                    anchored().child(
                        div()
                            .w(window_size.width)
                            .h(window_size.height)
                            .occlude()
                            .child(
                                anchored()
                                    .position(position)
                                    .anchor(Corner::TopLeft)
                                    .child(menu),
                            ),
                    ),
                )
                .with_priority(1),
            )
        })
}
```

**Why the full-window occlude overlay?** Without it, hover effects on elements behind the menu will still trigger as you move the mouse. The pattern is:
1. Outer `anchored()` at origin (0,0)
2. Full-window div with `.occlude()` blocks ALL mouse events (hover, click) to elements behind
3. Inner `anchored()` positions the actual menu at the click location

This pattern comes from `gpui-component`'s own `context_menu.rs` implementation.

Key points:
- `deferred()` ensures proper z-ordering (renders on top)
- Full-window `.occlude()` div blocks hover events on elements behind
- Subscribe to `DismissEvent` to clean up when menu closes
- `.with_priority(1)` ensures it renders above other deferred elements

### Highlighting the Right-Clicked Item

For good UX, the item you right-clicked should stay highlighted while the menu is open. Store the target item's identifier alongside the menu:

```rust
// Include the item name in the tuple
context_menu: Option<(Entity<PopupMenu>, Point<Pixels>, String, Subscription)>,

// When deploying, store the item name
self.context_menu = Some((menu, position, item_name, subscription));

// In rendering, check if item matches
let context_menu_item = self.context_menu.as_ref().map(|(_, _, name, _)| name.clone());

// Apply highlight when rendering items
let is_context_target = context_menu_item.as_ref() == Some(&item.name);
div()
    .when(is_context_target, |el| el.bg(rgb(element_hover)))
    .when(!is_context_target, |el| el.hover(move |s| s.bg(rgb(element_hover))))
```

## gpui-component Library

The project uses `gpui-component` from Longbridge for additional UI components like Button, Switch, Checkbox, Slider, Progress, Badge, Avatar, Spinner, Calendar, Select, and Charts.

### Available Components

```rust
use gpui_component::{
    avatar::Avatar,
    badge::Badge,
    button::{Button, ButtonVariants},
    calendar::{Calendar, CalendarState},
    chart::LineChart,
    checkbox::Checkbox,
    progress::Progress,
    select::{Select, SelectState, SearchableVec},
    slider::{Slider, SliderState},
    spinner::Spinner,
    switch::Switch,
    Disableable, Sizable,  // Traits for .disabled() and .with_size()
};
```

### Gotcha: Icon Asset Path Mismatch

gpui-component looks for icons at `icons/foo.svg` but this project stores them at `assets/icons/foo.svg`. The `Assets` implementation in `src/assets.rs` handles this by falling back to `assets/icons/` when the direct path doesn't exist:

```rust
// In Assets::load()
if path.starts_with("icons/") {
    let assets_path = self.base.join("assets").join(path);
    if assets_path.exists() {
        return fs::read(assets_path)...
    }
}
```

Without this fix, Spinner and other icon-based components will be invisible.

### Component State Patterns

Some components require Entity state:

```rust
// Calendar
let calendar_state = cx.new(|cx| CalendarState::new(window, cx));
Calendar::new(&calendar_state)

// Select/Dropdown
let items = SearchableVec::new(vec!["Apple", "Banana", "Orange"]);
let select_state = cx.new(|cx| SelectState::new(items, None, window, cx).searchable(true));
Select::new(&select_state).placeholder("Choose...").cleanable(true)

// Slider
let slider_state = cx.new(|_cx| SliderState::new().min(0.0).max(100.0).default_value(50.0));
Slider::new(&slider_state)
```

### Interactive Components with Handlers

```rust
// Switch with state tracking
Switch::new("my-switch")
    .checked(self.is_enabled)
    .label("Enable feature")
    .on_click(cx.listener(|this, checked: &bool, _window, cx| {
        this.is_enabled = *checked;
        cx.notify();
    }))

// Checkbox
Checkbox::new("my-checkbox")
    .checked(self.agreed)
    .label("I agree")
    .on_click(cx.listener(|this, checked: &bool, _window, cx| {
        this.agreed = *checked;
        cx.notify();
    }))

// Button variants
Button::new("btn").label("Primary").primary()
Button::new("btn").label("Danger").danger()
Button::new("btn").label("Ghost").ghost()
Button::new("btn").label("Outline").primary().outline()
Button::new("btn").label("Loading").loading(true)
Button::new("btn").label("Disabled").disabled(true)  // Requires `use gpui_component::Disableable`
```

### Sizing

Use `Sizable` trait for component sizes:

```rust
use gpui_component::Sizable;

Spinner::new().with_size(gpui_component::Size::Small)
Avatar::new().name("John Doe").with_size(gpui_component::Size::Large)
Button::new("btn").label("Small").with_size(gpui_component::Size::Small)
```

### Theming Note

gpui-component uses its own theme system via `gpui_component::theme`. Components automatically respect the global gpui_component theme set during `gpui_component::init(cx)`. They handle their own hover/active/disabled states internally.

## Custom Dropdown/Select Components

When the built-in `Select` component isn't sufficient (e.g., you need multiple columns, custom layouts, or complex filtering), build a custom dropdown using this pattern:

### Structure

```rust
pub struct MyComponent {
    dropdown_open: bool,
    selected_item: Option<MyItem>,
    search_input: Entity<TextInput>,
    items: Vec<MyItem>,
}
```

### Positioning Pattern

Use **relative/absolute positioning** with `deferred()` for z-ordering:

```rust
fn render_dropdown_trigger(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    let dropdown_open = self.dropdown_open;

    div()
        .relative()  // Container for absolute positioning
        .child(
            // The trigger button
            div()
                .id("dropdown-trigger")
                .px_3().py_2()
                .border_1().rounded_md()
                .cursor_pointer()
                .child("Select...")
                .child(icon_sm("chevron-down", text_muted))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                    this.dropdown_open = !this.dropdown_open;
                    cx.notify();
                }))
        )
        .when(dropdown_open, |el| {
            el.child(
                deferred(
                    div()
                        .absolute()
                        .top(px(42.))  // Position below trigger
                        .left_0()
                        .child(self.render_dropdown_panel(cx))
                )
                .with_priority(1)  // Render above other content
            )
        })
}
```

### Dropdown Panel with Columns

```rust
fn render_dropdown_panel(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    div()
        .id("dropdown-panel")
        .occlude()  // Prevent clicks from propagating to backdrop
        .w(px(500.))
        .bg(rgb(surface))
        .border_1().rounded_lg().shadow_lg()
        .child(
            v_flex()
                // Column headers
                .child(
                    h_flex()
                        .px_3().py_2()
                        .bg(rgb(element_bg))
                        .child(div().w(px(100.)).child("Column 1"))
                        .child(div().w(px(150.)).child("Column 2"))
                        .child(div().flex_1().child("Column 3"))
                )
                // Search input
                .child(
                    div().p_2().child(self.search_input.clone())
                )
                // Scrollable rows
                .child(
                    div()
                        .id("dropdown-scroll")
                        .max_h(px(250.))
                        .overflow_y_scroll()
                        .children(filtered_items.iter().map(|item| {
                            div()
                                .id(item.id)
                                .px_3().py_2()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(element_hover)))
                                .flex()
                                .child(div().w(px(100.)).child(&item.col1))
                                .child(div().w(px(150.)).child(&item.col2))
                                .child(div().flex_1().child(&item.col3))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                    this.selected_item = Some(item.clone());
                                    this.dropdown_open = false;
                                    cx.notify();
                                }))
                        }))
                )
        )
}
```

### Click-Outside-to-Close Backdrop

Add a transparent backdrop in the main render to close when clicking outside:

```rust
fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let dropdown_open = self.dropdown_open;

    div()
        .size_full()
        // ... your content including the dropdown trigger ...
        .when(dropdown_open, |el| {
            el.child(
                div()
                    .id("dropdown-backdrop")
                    .absolute()
                    .inset_0()
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                        this.dropdown_open = false;
                        cx.notify();
                    }))
            )
        })
}
```

### Key Points

- **`.relative()` on container** - Required for absolute positioning of dropdown panel
- **`.absolute().top(px(42.)).left_0()`** - Position panel below trigger (adjust `top` based on trigger height)
- **`deferred().with_priority(1)`** - Ensures dropdown renders above other content
- **`.occlude()`** - On dropdown panel to prevent clicks propagating to backdrop
- **Backdrop in main render** - Catches clicks outside to close dropdown
- **Search filtering** - Filter items based on `search_input.read(cx).content()`

See `src/components_test.rs` for a complete working example (`render_custom_select_section` and `render_lot_dropdown`).

## Event Emitter Pattern (Entity Communication)

When child components need to notify parents of events (like a DataTable notifying the page that a cell was saved), use the `EventEmitter` trait with subscriptions:

### Define the Event

```rust
#[derive(Debug, Clone)]
pub struct CellSaveRequested {
    pub row_index: usize,
    pub col_index: usize,
    pub new_value: String,
}

// Make the state struct an emitter of this event
impl EventEmitter<CellSaveRequested> for DataTableState {}
```

### Emit Events from Child

```rust
// In a click handler, emit the event
state_entity.update(cx, |_state, cx| {
    cx.emit(CellSaveRequested {
        row_index: row,
        col_index: col,
        new_value: value,
    });
});
```

### Subscribe in Parent

```rust
// Store subscriptions to keep them alive
pub struct MyPage {
    _subscriptions: Vec<Subscription>,
    // ...
}

// When creating the child entity, subscribe to its events
let table_state = cx.new(DataTableState::new);

let subscription = cx.subscribe(&table_state, |this, table_state, event: &CellSaveRequested, cx| {
    this.handle_cell_save(table_state, event, cx);
});
self._subscriptions.push(subscription);
```

**Critical**: Store the `Subscription` somewhere (like a Vec) or it will be dropped immediately and no events will fire!

## Anchored Popup Pattern

For popups that need to appear at a specific position (like edit dialogs at a cell location), use `deferred()` with `anchored()`:

```rust
let edit_popup = if let Some(editing) = &self.editing_state {
    let popup_x = editing.bounds.origin.x;
    let popup_y = editing.bounds.origin.y + editing.bounds.size.height;

    Some(
        deferred(
            anchored()
                .position(Point::new(popup_x, popup_y))
                .anchor(Corner::TopLeft)  // Which corner of popup goes at position
                .child(
                    div()
                        .id("edit-popup")
                        .occlude()  // Stop click propagation
                        .w(px(320.))
                        .p_3()
                        .bg(rgb(surface))
                        .border_1()
                        .rounded_lg()
                        .shadow_xl()
                        .child(/* popup content */)
                ),
        )
        .with_priority(2)  // Higher priority = renders on top
    )
} else {
    None
};

// In the parent container
div()
    .relative()
    .children(main_content)
    .children(edit_popup)  // Renders on top due to deferred()
```

Key points:
- `anchored()` positions the popup at exact pixel coordinates
- `Corner::TopLeft` means the top-left of the popup sits at the position
- `.occlude()` prevents clicks from going through to elements behind
- `.with_priority(n)` controls z-order among deferred elements

## Updating Data in Arc

When you have data in `Arc<Vec<T>>` and need to update it (e.g., after a cell edit), you can't mutate directly. Use `Arc::get_mut()` with a fallback to cloning:

```rust
pub fn update_cell_value(&mut self, row: usize, col: usize, value: SharedString) {
    // Try to get mutable access (only works if we have sole ownership)
    if let Some(rows) = Arc::get_mut(&mut self.rows) {
        if let Some(row_data) = rows.get_mut(row) {
            if let Some(cell) = row_data.get_mut(col) {
                *cell = value;
            }
        }
    } else {
        // Arc is shared elsewhere, clone and update
        let mut new_rows = (*self.rows).clone();
        if let Some(row_data) = new_rows.get_mut(row) {
            if let Some(cell) = row_data.get_mut(col) {
                *cell = value;
            }
        }
        self.rows = Arc::new(new_rows);
    }
}
```

This pattern avoids unnecessary cloning when possible but handles the shared case gracefully.

## Double-Click Detection

GPUI provides click count in mouse events for detecting double-clicks:

```rust
.on_mouse_down(MouseButton::Left, move |event, window, cx| {
    if event.click_count == 2 {
        // Handle double-click
        start_editing(window, cx);
    }
})
```

## Horizontal Scrolling (IMPORTANT GOTCHA)

**GPUI's `overflow_scroll()` does NOT properly support horizontal scrolling.** The layout system calculates content size based on flex constraints, which clamps horizontal scroll to 0 even when content is wider than the container.

### The Problem

Even with `.overflow_scroll()` set and content wider than the viewport:
- Vertical scrolling works
- Horizontal scroll events are received (delta.x has values)
- But the scroll offset is immediately clamped to 0 because GPUI's layout reports content width = container width

This is how GPUI's scroll listener handles it:
```rust
// In GPUI's div.rs - scroll_max is calculated from content_size - bounds
let scroll_max = (content_size - bounds.size).max(&Default::default());
scroll_offset.x = scroll_offset.x.clamp(-scroll_max.width, px(0.));
```

If the layout engine thinks content fits in the container (due to flex sizing), `scroll_max.width` is 0 and horizontal scroll is disabled.

### The Solution: Manual Scroll Handling

Implement your own scroll handling like Zed's editor does:

```rust
pub struct MyScrollableComponent {
    scroll_offset: Point<Pixels>,
    // ... other fields
}

impl MyScrollableComponent {
    fn on_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut Context<Self>) {
        let delta = event.delta.pixel_delta(px(20.)); // 20px line height for Lines delta

        // Update scroll offset (negative because scroll down = content moves up)
        self.scroll_offset.x -= delta.x;
        self.scroll_offset.y -= delta.y;

        // Calculate content dimensions
        let content_width = px(3000.);  // Your actual content width
        let content_height = px(2000.); // Your actual content height
        let viewport_width = px(800.);  // Estimate or track actual viewport
        let viewport_height = px(400.);

        // Clamp to valid range
        let max_scroll_x = (content_width - viewport_width).max(px(0.));
        let max_scroll_y = (content_height - viewport_height).max(px(0.));

        self.scroll_offset.x = self.scroll_offset.x.clamp(px(0.), max_scroll_x);
        self.scroll_offset.y = self.scroll_offset.y.clamp(px(0.), max_scroll_y);

        cx.notify();
    }
}

impl Render for MyScrollableComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let offset_x = -self.scroll_offset.x;
        let offset_y = -self.scroll_offset.y;

        let content = div()
            .w(px(3000.))  // Explicit content width
            .h(px(2000.))
            .flex()
            .flex_col()
            .children(/* your content */);

        div()
            .id("scroll-container")
            .size_full()
            .overflow_hidden()  // Clip content, don't use overflow_scroll!
            .on_scroll_wheel(cx.listener(Self::on_scroll))
            .child(
                div()
                    .relative()
                    .size_full()
                    .child(
                        div()
                            .absolute()
                            .left(offset_x)  // Translate content
                            .top(offset_y)
                            .child(content),
                    ),
            )
    }
}
```

### Key Points

1. **Don't use `.overflow_scroll()`** for horizontal scrolling - use `.overflow_hidden()` instead
2. **Track scroll offset in component state** - not GPUI's built-in scroll
3. **Handle `on_scroll_wheel` manually** - apply delta to your offset
4. **Use absolute positioning** - translate content with `.left(offset)` and `.top(offset)`
5. **Calculate your own content dimensions** - GPUI's layout won't give correct values
6. **Clamp scroll offset** - prevent scrolling past content bounds

### Why This Works

- `overflow_hidden` clips content without involving GPUI's scroll system
- Absolute positioning lets you place content at any offset
- Manual scroll handling gives you full control over horizontal scrolling
- This is the same approach Zed uses for their editor's horizontal scrolling

### Dynamic Viewport Measurement with `canvas()`

When you need to measure the actual bounds of a container (for scroll calculations, virtualization, etc.), use GPUI's `canvas()` element:

```rust
use gpui::canvas;

pub struct MyComponent {
    viewport_size: Size<Pixels>,
}

impl MyComponent {
    fn set_viewport_size(&mut self, size: Size<Pixels>, cx: &mut Context<Self>) {
        if self.viewport_size != size {
            self.viewport_size = size;
            cx.notify();
        }
    }
}

impl Render for MyComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        div()
            .size_full()
            .relative()
            // Canvas measures the container bounds
            .child(
                canvas(
                    move |bounds, _window, cx| {
                        entity.update(cx, |this, cx| {
                            this.set_viewport_size(bounds.size, cx);
                        });
                    },
                    |_, _, _, _| {}, // paint callback (unused)
                )
                .absolute()
                .inset_0(), // Fill the container
            )
            // Actual content
            .child(content)
    }
}
```

The canvas's first callback runs during the prepaint phase and receives the actual bounds.

### Row Virtualization for Performance

When rendering large tables (100+ rows), only render visible rows to maintain 60fps:

```rust
// Calculate visible row range
let scroll_y_after_header = (scroll_offset.y - header_height).max(px(0.));
let first_visible_row = (f32::from(scroll_y_after_header) / ROW_HEIGHT).floor() as usize;
let first_visible_row = first_visible_row.saturating_sub(2); // Buffer above
let visible_count = (f32::from(viewport_height) / ROW_HEIGHT).ceil() as usize + 4; // Buffer below
let last_visible_row = (first_visible_row + visible_count).min(row_count);

// Only render visible rows with absolute positioning
let visible_rows: Vec<_> = (first_visible_row..last_visible_row)
    .map(|row_ix| {
        let row_y = header_height + row_height * row_ix as f32 - scroll_offset.y;

        div()
            .absolute()
            .left(-scroll_offset.x)
            .top(row_y)
            .w(content_width)
            // ... row content
    })
    .collect();
```

**Important**: Always use `cargo run --release` when testing scroll performance. Debug builds are 10-100x slower and will appear choppy even with virtualization.

### See Also

- `src/scroll_test.rs` - Isolated test page demonstrating this pattern
- `src/components/data_table.rs` - DataTable component using manual scroll

## Column Resizing with Drag (IMPORTANT GOTCHA)

When implementing column resize handles, there's a critical coordinate system issue between `on_drag` and `on_drag_move`.

### The Problem

`on_drag` and `on_drag_move` may report positions in different coordinate systems. If you store the start position from `on_drag` and compare it to positions from `on_drag_move`, you'll get huge incorrect deltas:

```rust
// WRONG - coordinates may be in different systems
.on_drag(
    DragState { start_x: px(0.) },
    |drag, point, _window, cx| {
        let mut drag = drag.clone();
        drag.start_x = point.x;  // This position...
        cx.new(|_| drag)
    },
)
.on_drag_move::<DragState>(move |event, _window, cx| {
    let drag = event.drag(cx);
    let delta = event.event.position.x - drag.start_x;  // ...may not match this one!
    // delta could be hundreds of pixels on the first move event
})
```

### The Solution: Track Delta Between Move Events

Only use coordinates from `on_drag_move` events, which are guaranteed to be in the same coordinate system:

```rust
struct ResizeDragState {
    col_index: usize,
    last_x: Option<Pixels>,  // None until first move event
}

impl MyComponent {
    fn start_resize_drag(&mut self, col_index: usize) {
        self.resize_drag = Some(ResizeDragState {
            col_index,
            last_x: None,  // Don't set from on_drag position!
        });
    }

    fn update_resize_drag(&mut self, current_x: Pixels, cx: &mut Context<Self>) {
        if let Some(ref mut drag) = self.resize_drag {
            if let Some(last_x) = drag.last_x {
                // Delta between consecutive move events (same coordinate system)
                let delta = current_x - last_x;
                if let Some(col) = self.columns.get_mut(drag.col_index) {
                    col.width = (col.width + delta).max(px(50.));
                    cx.notify();
                }
            }
            // Always update for next event
            drag.last_x = Some(current_x);
        }
    }
}

// In render:
div()
    .id(ElementId::NamedInteger("col-resize".into(), col_idx as u64))
    .on_drag(
        DraggedColumnResize { col_index: col_idx },
        move |drag, _point, _window, cx| {  // Ignore the point!
            state_entity.update(cx, |state, _cx| {
                state.start_resize_drag(drag.col_index);
            });
            cx.new(|_| drag.clone())
        },
    )
    .on_drag_move::<DraggedColumnResize>(move |event, _window, cx| {
        state_entity.update(cx, |state, cx| {
            state.update_resize_drag(event.event.position.x, cx);
        });
    })
```

### Key Points

1. **First `on_drag_move` initializes position** - don't resize, just record `last_x`
2. **Subsequent events calculate delta** - from previous `on_drag_move` position
3. **Ignore `point` from `on_drag`** - it may be in a different coordinate system
4. **Drag payload must implement `Render`** - GPUI requires it even if you return `Empty`

```rust
#[derive(Clone)]
struct DraggedColumnResize {
    col_index: usize,
}

impl Render for DraggedColumnResize {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty  // No visual drag preview needed
    }
}
```

### See Also

- `src/components/data_table.rs` - Working column resize implementation

## Draggable Popup Cards

For popup cards that users can drag to reposition (like FK detail cards), combine `anchored()` positioning with drag tracking:

### State Structure

```rust
#[derive(Clone)]
pub struct PopupCardData {
    pub row_index: usize,
    pub col_index: usize,
    pub drag_offset: Point<Pixels>,  // User's drag adjustment
    // ... other data
}

pub struct MyComponent {
    active_card: Option<PopupCardData>,
    card_drag_start: Option<Point<Pixels>>,  // Track drag start position
}
```

### Drag Marker (Required)

GPUI's drag system requires the dragged value to implement `Render`:

```rust
#[derive(Clone)]
struct DraggedCard;

impl Render for DraggedCard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty  // No visual preview needed
    }
}
```

### Drag Methods

```rust
pub fn start_card_drag(&mut self, position: Point<Pixels>) {
    self.card_drag_start = Some(position);
}

pub fn update_card_drag(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
    if let (Some(start), Some(ref mut card)) = (self.card_drag_start, &mut self.active_card) {
        let delta = position - start;
        card.drag_offset = card.drag_offset + delta;
        self.card_drag_start = Some(position);  // Update for next delta
        cx.notify();
    }
}

pub fn end_card_drag(&mut self) {
    self.card_drag_start = None;
}
```

### Render with Drag Handlers

```rust
fn render_card(card: &PopupCardData, base_position: Point<Pixels>, state: Entity<MyComponent>) -> impl IntoElement {
    let final_position = base_position + card.drag_offset;
    let state_for_drag = state.clone();
    let state_for_drag_end = state.clone();

    deferred(
        anchored()
            .position(final_position)
            // NOTE: Don't use .snap_to_window() with dragging - see gotcha below
            .child(
                div()
                    .id("popup-card")
                    .occlude()
                    .child(
                        // Draggable header
                        div()
                            .id("card-header")
                            .on_drag(DraggedCard, move |_, _, _, cx| {
                                cx.new(|_| DraggedCard)
                            })
                            .on_drag_move::<DraggedCard>(move |event, _window, cx| {
                                state_for_drag.update(cx, |state, cx| {
                                    if state.card_drag_start.is_none() {
                                        state.start_card_drag(event.event.position);
                                    } else {
                                        state.update_card_drag(event.event.position, cx);
                                    }
                                });
                            })
                            .on_mouse_up(MouseButton::Left, move |_, _window, cx| {
                                state_for_drag_end.update(cx, |state, _cx| {
                                    state.end_card_drag();
                                });
                            })
                            .child(/* header content */)
                    )
                    .child(/* card body */)
            )
    )
    .with_priority(2)
}
```

### Gotcha: snap_to_window() Breaks Drag Tracking

**Do NOT use `.snap_to_window()` with draggable elements.**

`snap_to_window()` automatically repositions elements to stay within the window bounds. However, drag tracking uses raw mouse coordinates. If a popup is snapped to a different position than calculated, the drag offset will be wrong:

1. Card opens near bottom of window
2. `snap_to_window()` moves it up to stay visible
3. User starts dragging from the header
4. Drag tracking records mouse position, but doesn't know about the snap
5. Card jumps to wrong position

**Solution**: Remove `snap_to_window()` and let users drag cards into view if they open off-screen.

### Calculating Cell-Based Positions

When a popup should appear relative to a table cell, calculate the position from row/col indices:

```rust
fn calculate_cell_position(&self, row_index: usize, col_index: usize) -> Point<Pixels> {
    // Sum column widths up to target column
    let col_x: Pixels = self.columns.iter().take(col_index).map(|c| c.width).sum();

    // Cell position relative to scroll
    let cell_x = col_x - self.scroll_offset.x;
    let cell_y = px(HEADER_HEIGHT) + px(ROW_HEIGHT) * row_index as f32
                 - self.scroll_offset.y + px(ROW_HEIGHT);  // Below the cell

    // Add container's window position
    point(
        self.container_origin.x + cell_x,
        self.container_origin.y + cell_y,
    )
}
```

Track `container_origin` using a canvas callback:

```rust
canvas(
    move |bounds, _window, cx| {
        state.update(cx, |state, _cx| {
            state.container_origin = bounds.origin;
        });
    },
    |_, _, _, _| {},
)
.absolute()
.inset_0()
```

### See Also

- `src/components/data_table.rs` - FK card implementation with dragging
