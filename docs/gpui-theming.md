# GPUI Theming System

This document explains Zed's theming architecture and how to implement it in a GPUI application.

## Overview

Zed's theming system provides:
- **150+ semantic color tokens** organized by purpose
- **Type-safe access** via Rust structs
- **Reactive updates** - windows refresh automatically on theme change
- **Light/dark variants** with OS appearance detection

## Theme File Structure

Themes are JSON files with this structure:

```json
{
  "$schema": "https://zed.dev/schema/themes/v0.2.0.json",
  "name": "One",
  "author": "Zed Industries",
  "themes": [
    {
      "name": "One Dark",
      "appearance": "dark",
      "style": {
        "background": "#282c33ff",
        "surface.background": "#2f343eff",
        "element.background": "#3b414dff",
        "border": "#464b57ff",
        "border.variant": "#363c46ff",
        "text": "#dce0e5ff",
        "text.muted": "#a9afbcff"
      }
    }
  ]
}
```

**Key points:**
- Colors use **dot notation** keys: `element.hover`, `editor.background`
- Colors are **hex RGBA**: `#RRGGBBAA` (alpha is last two digits)
- A theme family can contain multiple variants (light/dark)

## Core Types

### ThemeStyles

The main container for all theme data:

```rust
pub struct ThemeStyles {
    pub colors: ThemeColors,      // 150+ UI colors
    pub status: StatusColors,     // Error, warning, success, etc.
    pub accents: AccentColors,    // Bracket colors, indent guides
    pub player: PlayerColors,     // Collaboration cursors
    pub syntax: Arc<SyntaxTheme>, // Syntax highlighting
}
```

### Theme

The runtime theme object:

```rust
pub struct Theme {
    pub id: String,
    pub name: SharedString,
    pub appearance: Appearance,  // Light or Dark
    pub styles: ThemeStyles,
}
```

## Semantic Color Categories

### Backgrounds (Layering)

Backgrounds follow a layering system. In dark themes, higher layers are lighter:

| Token | Purpose | Example Values (Dark) |
|-------|---------|----------------------|
| `background` | Base app/window background | `#282c33` |
| `surface_background` | Panels, sidebars, headers | `#2f343e` |
| `elevated_surface_background` | Modals, popovers, dropdowns | `#363c46` |
| `element_background` | Buttons, inputs, cards | `#3b414d` |
| `ghost_element_background` | Elements that blend with surface | transparent |

### Element States

Interactive elements have state variants:

| Token | Purpose |
|-------|---------|
| `element_background` | Default state |
| `element_hover` | Mouse hover |
| `element_active` | Being pressed |
| `element_selected` | Selected/checked |
| `element_disabled` | Disabled state |

### Borders

| Token | Purpose |
|-------|---------|
| `border` | Standard borders (high contrast) |
| `border_variant` | Subtle dividers between sections |
| `border_focused` | Keyboard focus indicator |
| `border_selected` | Selected items |
| `border_disabled` | Disabled elements |

### Text

| Token | Purpose |
|-------|---------|
| `text` | Primary text |
| `text_muted` | Secondary/deemphasized text |
| `text_placeholder` | Input placeholders |
| `text_disabled` | Disabled text |
| `text_accent` | Links, highlights |

### Icons

| Token | Purpose |
|-------|---------|
| `icon` | Default icon color |
| `icon_muted` | Deemphasized icons |
| `icon_disabled` | Disabled icons |
| `icon_accent` | Active/selected toggles |

### UI Component Backgrounds

Specific components have dedicated tokens:

```
title_bar_background
title_bar_inactive_background
status_bar_background
toolbar_background
tab_bar_background
tab_active_background
tab_inactive_background
panel_background
editor_background
editor_gutter_background
```

### Status Colors

Each status type has three variants: foreground, background, and border.

| Status | Use Case |
|--------|----------|
| `error` | Errors, failures |
| `warning` | Warnings, caution |
| `success` | Success, completion |
| `info` | Informational |
| `hint` | Suggestions |
| `created` | New files/additions |
| `modified` | Changed files |
| `deleted` | Deletions |
| `conflict` | Merge conflicts |
| `ignored` | Git-ignored files |

Access pattern:
```rust
let status = theme.status();
div().bg(status.error_background).child(
    text("Error!").color(status.error)
)
```

## Accessing Themes in Components

### The ActiveTheme Trait

GPUI provides the `ActiveTheme` trait for easy theme access:

```rust
use gpui::ActiveTheme;

fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = cx.theme();
    let colors = theme.colors();

    div()
        .bg(colors.background)
        .border_1()
        .border_color(colors.border_variant)
        .child(
            div()
                .text_color(colors.text)
                .child("Hello, World!")
        )
}
```

### Available Accessors

```rust
let theme = cx.theme();

theme.colors()   // ThemeColors - main UI colors
theme.status()   // StatusColors - error, warning, success, etc.
theme.syntax()   // SyntaxTheme - code highlighting
theme.accents()  // AccentColors - cycling colors for brackets
theme.players()  // PlayerColors - collaboration cursors
```

## Setting Up Theming

### 1. Define the GlobalTheme

```rust
use gpui::Global;
use std::sync::Arc;

pub struct GlobalTheme {
    pub theme: Arc<Theme>,
}

impl Global for GlobalTheme {}

impl GlobalTheme {
    pub fn init(cx: &mut App) {
        let theme = load_default_theme(); // Your loading logic
        cx.set_global(Self { theme: Arc::new(theme) });
    }

    pub fn get(cx: &App) -> &Arc<Theme> {
        &cx.global::<Self>().theme
    }
}
```

### 2. Implement ActiveTheme

```rust
pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        GlobalTheme::get(self)
    }
}
```

### 3. Initialize at Startup

```rust
fn main() {
    Application::new().run(|cx: &mut App| {
        // Initialize theme system
        ThemeRegistry::set_global(cx);
        GlobalTheme::init(cx);

        // Open window...
    });
}
```

## Theme Switching

### Update and Refresh

```rust
pub fn set_theme(theme_name: &str, cx: &mut App) {
    let registry = ThemeRegistry::global(cx);
    if let Some(theme) = registry.get(theme_name) {
        cx.global_mut::<GlobalTheme>().theme = theme;
        cx.refresh_windows(); // Triggers re-render of all windows
    }
}
```

### Observing Theme Changes

Components automatically re-render when `cx.refresh_windows()` is called. For custom logic:

```rust
cx.observe_global::<GlobalTheme>(|cx| {
    // React to theme changes
}).detach();
```

## Color Format Conversion

Theme colors are typically stored as `Hsla`. Convert to GPUI's `rgb()`:

```rust
use gpui::{rgb, Hsla};

// If your theme uses hex values
let color = rgb(0x282c33);

// If working with Hsla
let hsla: Hsla = theme.colors().background;
div().bg(hsla) // GPUI accepts Hsla directly
```

## Best Practices

### 1. Use Semantic Tokens

```rust
// Good - semantic meaning
div().bg(colors.surface_background)

// Avoid - hardcoded colors
div().bg(rgb(0x2f343e))
```

### 2. Respect the Layering System

```rust
// Main content area
div().bg(colors.background)
    // Sidebar panel
    .child(div().bg(colors.surface_background))
    // Modal overlay
    .child(div().bg(colors.elevated_surface_background))
```

### 3. Use Appropriate Border Variants

```rust
// Prominent separation
div().border_color(colors.border)

// Subtle divider
div().border_color(colors.border_variant)
```

### 4. Handle States Consistently

```rust
div()
    .bg(colors.element_background)
    .hover(|s| s.bg(colors.element_hover))
    .active(|s| s.bg(colors.element_active))
```

### 5. Match Text to Background

```rust
// Primary content
div().bg(colors.background).text_color(colors.text)

// Secondary/muted content
div().text_color(colors.text_muted)

// Disabled state
div().text_color(colors.text_disabled)
```

## Example: Themed Button

```rust
fn button(label: &str, cx: &App) -> impl IntoElement {
    let colors = cx.theme().colors();

    div()
        .px_3()
        .py_1()
        .rounded_md()
        .bg(colors.element_background)
        .border_1()
        .border_color(colors.border_variant)
        .text_color(colors.text)
        .hover(|s| s.bg(colors.element_hover))
        .active(|s| s.bg(colors.element_active))
        .child(label)
}
```

## Example: Status Message

```rust
fn status_message(message: &str, level: StatusLevel, cx: &App) -> impl IntoElement {
    let status = cx.theme().status();

    let (bg, fg, border) = match level {
        StatusLevel::Error => (status.error_background, status.error, status.error_border),
        StatusLevel::Warning => (status.warning_background, status.warning, status.warning_border),
        StatusLevel::Success => (status.success_background, status.success, status.success_border),
        StatusLevel::Info => (status.info_background, status.info, status.info_border),
    };

    div()
        .px_3()
        .py_2()
        .rounded_md()
        .bg(bg)
        .border_1()
        .border_color(border)
        .text_color(fg)
        .child(message)
}
```

## gpui_component Code Editor Background

When using `gpui_component::input::Input` as a code editor (via `.code_editor("sql")`), the editor background and line number gutter are controlled by the `HighlightTheme`, **not** by styles applied to the Input component.

**Problem:** Setting `.bg()` on the Input only affects the outer container. The line number gutter is painted internally using `cx.theme().editor_background()`, which comes from `HighlightTheme.style.editor_background`.

**Solution:** Set the editor background on the HighlightTheme when initializing themes:

```rust
use gpui_component::highlighter::HighlightTheme;
use std::sync::Arc;

let base_theme = if theme.appearance.is_light() {
    HighlightTheme::default_light()
} else {
    HighlightTheme::default_dark()
};
let mut highlight_theme = (*base_theme).clone();
highlight_theme.style.editor_background = Some(hex_to_hsla(colors.elevated_surface));
gpui_theme.highlight_theme = Arc::new(highlight_theme);
```

**Why this matters:** The default dark `HighlightTheme` uses black (`#000000`) for `editor_background`, which looks harsh in many dark themes. Setting it to `elevated_surface` or similar provides better visual consistency.

## Reference: One Dark Colors

A reference implementation based on Zed's One Dark theme:

```rust
struct OneDarkColors {
    // Backgrounds (darkest to lightest)
    background: 0x282c33,
    surface: 0x2f343e,
    elevated_surface: 0x363c46,
    element: 0x3b414d,
    element_hover: 0x464b57,

    // Borders
    border: 0x464b57,
    border_variant: 0x363c46,

    // Text
    text: 0xdce0e5,
    text_muted: 0xa9afbc,
    text_accent: 0x74ade8,

    // Status
    error: 0xd07277,
    warning: 0xdec184,
    success: 0xa1c181,
    info: 0x74ade8,
}
```
