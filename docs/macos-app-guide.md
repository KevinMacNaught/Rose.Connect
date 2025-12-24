# macOS App Development Guide

This guide covers macOS-specific features: native menu bars, window titlebars, and app lifecycle.

## Custom Window Titlebar

To create a Zed-style custom titlebar:
```rust
let options = WindowOptions {
    titlebar: Some(TitlebarOptions {
        title: Some("App Name".into()),
        appears_transparent: true,  // Hides OS titlebar
        traffic_light_position: Some(point(px(9.), px(9.))),
    }),
    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
        None,
        size(px(1200.), px(800.)),
        cx,
    ))),
    ..Default::default()
};
```

- Leave ~70px left padding in your custom header for traffic lights
- The entire window content becomes your canvas when `appears_transparent: true`

## App Shell Pattern

For multi-view applications, use a persistent shell that provides consistent chrome across all views:

```rust
const TITLEBAR_HEIGHT: f32 = 36.0;
const FOOTER_HEIGHT: f32 = 24.0;

// Shell structure:
// +----+-------------------------------------+
// |    | HEADER (app title, actions)         |  <- shell_bg (element)
// |icon+-------------------------------------+
// |nav |                                     |
// |    |     CONTENT AREA (apps render here) |  <- background
// |    |                                     |
// |    +-------------------------------------+
// |    | FOOTER (status)                     |  <- shell_bg (element)
// +----+-------------------------------------+

div()
    .size_full()
    .flex()
    .child(
        // Icon sidebar - full height including title bar region
        div()
            .w(px(52.))
            .h_full()
            .bg(rgb(shell_bg))  // colors.element
            .pt(px(TITLEBAR_HEIGHT))  // Push content below traffic lights
            .children(nav_icons)
    )
    .child(
        div()
            .flex_1()
            .flex()
            .flex_col()
            .child(
                // Header - needs left padding for traffic lights
                div()
                    .h(px(TITLEBAR_HEIGHT))
                    .bg(rgb(shell_bg))
                    .pl(px(70.))  // Clear traffic lights
                    .child(app_title)
            )
            .child(
                // Content area - apps render here
                div()
                    .flex_1()
                    .min_h_0()
                    .bg(rgb(background))
                    .child(current_app)
            )
            .child(
                // Footer
                div()
                    .h(px(FOOTER_HEIGHT))
                    .bg(rgb(shell_bg))
                    .child(status_text)
            )
    )
```

**Key points:**
- Icon sidebar extends full height (including title bar region) with its own background
- Header needs `pl(px(70.))` to clear traffic lights (they're positioned at 9,9)
- Apps rendered in content area should NOT add their own title bar padding
- Use `colors.element` for shell chrome, `colors.background` for content area

## Native Menu Bar

GPUI provides native macOS menu bar support through `Menu` and `MenuItem`:

```rust
use gpui::{Menu, MenuItem, SystemMenuType, actions};

actions!(app, [Quit, OpenSettings]);

fn app_menus() -> Vec<Menu> {
    vec![
        Menu {
            name: "App Name".into(),  // Appears in dropdown, NOT menu bar title
            items: vec![
                MenuItem::action("Settings...", OpenSettings),
                MenuItem::separator(),
                MenuItem::os_submenu("Services", SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action("Quit App Name", Quit),
            ],
        },
    ]
}

// In main():
cx.activate(true);  // Bring menu bar to foreground
cx.on_action(quit);
cx.on_action(open_settings);
cx.set_menus(app_menus());
```

**Note:** The bolded app name in the macOS menu bar comes from the binary/process name (Cargo.toml `name` field), NOT from the menu's `name` field. For a proper app name, create a macOS app bundle with Info.plist.

## Actions and Global State

Menu actions are dispatched globally. Route them to a specific window/component using Global state:

```rust
struct AppState {
    board: Option<Entity<KanbanBoard>>,
}

impl Global for AppState {}

fn open_settings(_: &OpenSettings, cx: &mut App) {
    // Clone handle first to avoid borrow checker issues
    let board = cx.global::<AppState>().board.clone();
    if let Some(board) = board {
        board.update(cx, |board, cx| {
            board.show_settings = true;
            cx.notify();
        });
    }
}

// In main():
cx.set_global(AppState { board: None });
cx.on_action(open_settings);

cx.open_window(options, |_window, cx| {
    let board = cx.new(|_cx| KanbanBoard::load());
    cx.global_mut::<AppState>().board = Some(board.clone());
    board
});
```

**Gotcha:** When accessing global state and then calling `entity.update(cx, ...)`, clone the entity handle first. Otherwise you get a borrow error because `cx.global()` borrows immutably while `update()` needs it mutably.

## Settings Window Pattern

Settings should be a separate window (not a modal overlay) so it stays open during theme changes when `cx.refresh_windows()` is called.

**Layout structure (Zed-style):**
```rust
div()
    .flex()
    .size_full()
    .bg(rgb(sidebar_bg))  // element - darker/more saturated
    .child(
        div()
            .w(px(220.))
            .pt(px(36.))  // Space for traffic lights
            .child(search_bar)
            .child(nav_items)
    )
    .child(
        div()
            .flex_1()
            .bg(rgb(content_bg))  // background - lighter
            .child(content)
    )
```

**Key design decisions:**
- Sidebar uses `element` (more saturated) for visual distinction
- Content uses `background` (lighter) - matches Zed's `editor.background`
- Search bar in sidebar uses `surface` to stand out
- Custom titlebar with `appears_transparent: true`
- Traffic lights at `point(px(9.), px(9.))`

**Why separate window vs modal:**
- Modals close when `cx.refresh_windows()` is called (theme changes trigger this)
- Separate windows persist and update their theme automatically

## Singleton Window Pattern

To prevent multiple instances of a window (e.g., settings), track the window handle globally:

```rust
struct SettingsWindowHandle(Option<WindowHandle<SettingsWindow>>);

impl Global for SettingsWindowHandle {}

pub fn open_settings_window(cx: &mut App) {
    // Check if window already exists and is still open
    if let Some(handle) = cx.try_global::<SettingsWindowHandle>() {
        if let Some(window_handle) = handle.0 {
            // update() returns Err if window was closed
            if window_handle
                .update(cx, |_, window, _| {
                    window.activate_window();  // Bring to front
                })
                .is_ok()
            {
                return;  // Window exists, focused it
            }
        }
    }

    // Create new window
    let window_handle = cx
        .open_window(options, |_window, cx| {
            cx.new(|_| SettingsWindow::new())
        })
        .unwrap();

    // Store handle for future checks
    cx.set_global(SettingsWindowHandle(Some(window_handle)));
}
```

**How it works:**
- `WindowHandle::update()` returns `Err` if the window has been closed
- Use `try_global()` since the global may not exist on first open
- `window.activate_window()` brings an existing window to the front
