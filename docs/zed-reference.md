# Zed Repository Reference

The Zed repo at `~/.cargo/git/checkouts/zed-*/` contains excellent examples for GPUI development.

## Key Files

| Topic | Path |
|-------|------|
| Native menus | `/crates/gpui/examples/set_menus.rs` |
| Menu/MenuItem definitions | `/crates/gpui/src/platform/app_menu.rs` |
| Real-world menu structure | `/crates/zed/src/zed/app_menus.rs` |
| Custom dropdown menus | `/crates/ui/src/components/context_menu.rs` |
| Deferred rendering (overlays) | `/crates/gpui/src/elements/deferred.rs` |
| Positioned popovers | `/crates/gpui/src/elements/anchored.rs` |
| GlobalTheme pattern | `/crates/theme/src/theme.rs` |
| ThemeRegistry pattern | `/crates/theme/src/registry.rs` |
| Theme picker UI | `/crates/theme_selector/src/theme_selector.rs` |
| Settings window layout | `/crates/settings_ui/src/settings_ui.rs` |
| Theme definitions | `/assets/themes/gruvbox/gruvbox.json` |
| **Text input example** | `/crates/gpui/examples/input.rs` |

## Text Input Implementation

GPUI doesn't have a built-in text input widget. The `/crates/gpui/examples/input.rs` file shows how to build one:

- `EntityInputHandler` trait for IME and text replacement
- `FocusHandle` for keyboard focus management
- `Element` impl for custom text rendering with cursor/selection
- Mouse handlers for click-to-position and drag-to-select
- Action handlers for keyboard navigation and editing

This project has a reusable `TextInput` component in `src/components/text_input.rs` based on this example.
