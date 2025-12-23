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

- Leave ~78px left padding in your custom header for traffic lights
- The entire window content becomes your canvas when `appears_transparent: true`

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
