# Component Theming Rules

Rules for applying theme colors consistently across all components and pages.

## Accessing Theme Colors

Always use the `ActiveTheme` trait to access colors:

```rust
use crate::theme::ActiveTheme;

fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = cx.theme();
    let colors = theme.colors();

    div().bg(rgb(colors.background))
}
```

For helper functions that don't have context access, pass `&ThemeColors`:

```rust
fn render_button(label: &str, colors: &ThemeColors) -> impl IntoElement {
    div()
        .bg(rgb(colors.element))
        .text_color(rgb(colors.text))
        .child(label)
}
```

## Never Hardcode Colors

```rust
// WRONG - hardcoded colors
div().bg(rgb(0x3b82f6)).text_color(rgb(0xffffff))

// CORRECT - theme tokens
div().bg(rgb(colors.accent)).text_color(rgb(colors.accent_foreground))
```

## Background Layering

Use progressively elevated backgrounds for visual hierarchy:

| Layer | Token | Use Case |
|-------|-------|----------|
| Base | `background` | Main content area, window background |
| Panel | `panel_background` | Sidebar panels, secondary regions |
| Surface | `surface` | Cards, containers on panels |
| Elevated | `elevated_surface` | Cards, modals, popovers, dropdowns |
| Element | `element` | Buttons, inputs, interactive elements |

```rust
// Main layout
div().bg(rgb(colors.background))
    .child(
        // Sidebar
        div().bg(rgb(colors.panel_background))
    )
    .child(
        // Content with card
        div().child(
            div().bg(rgb(colors.elevated_surface)) // Card
        )
    )
```

## Element States

Interactive elements must use state tokens:

```rust
div()
    .bg(rgb(colors.element))
    .hover(|s| s.bg(rgb(colors.element_hover)))
    .active(|s| s.bg(rgb(colors.element_active)))
```

| State | Token | When |
|-------|-------|------|
| Default | `element` | Normal state |
| Hover | `element_hover` | Mouse over |
| Active | `element_active` | Being pressed |
| Selected | `element_selected` | Selected/checked |
| Disabled | `element_disabled` | Disabled state |

For ghost/transparent elements:

```rust
div()
    .bg(rgb(colors.ghost_element_background)) // Transparent
    .hover(|s| s.bg(rgb(colors.ghost_element_hover)))
```

## Text Colors

| Token | Use Case |
|-------|----------|
| `text` | Primary content, headings, labels |
| `text_muted` | Secondary text, descriptions, metadata |
| `text_accent` | Links, highlighted text |
| `text_disabled` | Disabled text |
| `text_placeholder` | Input placeholders |

```rust
// Primary label
div().text_color(rgb(colors.text)).child("Title")

// Secondary info
div().text_color(rgb(colors.text_muted)).child("Updated 2 hours ago")

// Link
div().text_color(rgb(colors.text_accent)).child("Learn more")
```

## Icon Colors

Match icon colors to their context:

| Token | Use Case |
|-------|----------|
| `icon` | Primary icons |
| `icon_muted` | Secondary/decorative icons |
| `icon_disabled` | Disabled icons |
| `icon_accent` | Active/selected toggle icons |

## Borders

| Token | Use Case |
|-------|----------|
| `border` | Standard borders, prominent separation |
| `border_variant` | Subtle dividers between sections |
| `border_focused` | Keyboard focus indicator |
| `border_selected` | Selected items |
| `border_disabled` | Disabled elements |

```rust
// Standard border
div().border_1().border_color(rgb(colors.border))

// Subtle divider
div().border_b_1().border_color(rgb(colors.border_variant))

// Focus ring
div().when(focused, |el| el.border_color(rgb(colors.border_focused)))
```

## Primary Action Buttons

Use `accent` for primary CTAs:

```rust
div()
    .bg(rgb(colors.accent))
    .text_color(rgb(colors.accent_foreground))
    .child("Submit")
```

## Status Indicators

Each status has foreground, background, and border variants:

```rust
// Error message
div()
    .bg(rgb(colors.status_error_background))
    .border_1()
    .border_color(rgb(colors.status_error_border))
    .text_color(rgb(colors.status_error))
    .child("Error: Something went wrong")

// Success badge
div()
    .bg(rgb(colors.status_success_background))
    .text_color(rgb(colors.status_success))
    .child("Complete")
```

| Status | Foreground | Background | Border |
|--------|------------|------------|--------|
| Success | `status_success` | `status_success_background` | `status_success_border` |
| Warning | `status_warning` | `status_warning_background` | `status_warning_border` |
| Error | `status_error` | `status_error_background` | `status_error_border` |
| Info | `status_info` | `status_info_background` | `status_info_border` |

## Data-Driven Colors (Contrast Handling)

**Important:** This pattern is ONLY for confirmed user-customizable colors. For most UI elements, use semantic theme tokens instead.

**Before using this pattern, ask:**
> "Should this color be user-customizable or theme-controlled?"

If theme-controlled (most cases), use tokens like `element`, `status_*`, etc.

If truly user-customizable (rare), calculate contrasting text:

```rust
fn contrasting_text_color(bg_color: u32) -> u32 {
    let r = ((bg_color >> 16) & 0xFF) as f32 / 255.0;
    let g = ((bg_color >> 8) & 0xFF) as f32 / 255.0;
    let b = (bg_color & 0xFF) as f32 / 255.0;

    let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    if luminance > 0.5 { 0x1a1a1a } else { 0xffffff }
}
```

**Valid use cases:**
- User picks a custom color for their workspace/project
- User-defined tag/label colors (like Trello labels)
- Brand colors from external configuration

## Drag and Drop Highlights

Derive from accent color with alpha:

```rust
let drag_highlight = (colors.accent << 8) | 0x20; // 0x20 = ~12% alpha

div()
    .drag_over::<DraggedItem>(move |style, _, _, _| {
        style.bg(rgba(drag_highlight))
    })
```

## Component Patterns

### Button (Secondary)

```rust
fn button(label: &str, colors: &ThemeColors) -> impl IntoElement {
    div()
        .px_3()
        .py_1()
        .rounded_md()
        .bg(rgb(colors.element))
        .border_1()
        .border_color(rgb(colors.border_variant))
        .text_color(rgb(colors.text))
        .hover(|s| s.bg(rgb(colors.element_hover)))
        .active(|s| s.bg(rgb(colors.element_active)))
        .child(label)
}
```

### Button (Primary)

```rust
fn primary_button(label: &str, colors: &ThemeColors) -> impl IntoElement {
    div()
        .px_4()
        .py_1()
        .rounded_md()
        .bg(rgb(colors.accent))
        .text_color(rgb(colors.accent_foreground))
        .hover(|s| s.bg(rgb(colors.accent))) // Can add accent_hover if needed
        .child(label)
}
```

### Card

```rust
fn card(colors: &ThemeColors) -> Div {
    div()
        .rounded_lg()
        .bg(rgb(colors.elevated_surface))
        .shadow_sm()
        .p_4()
}
```

### Input Field

```rust
fn input(placeholder: &str, colors: &ThemeColors) -> impl IntoElement {
    div()
        .px_3()
        .py_2()
        .rounded_md()
        .bg(rgb(colors.surface))
        .border_1()
        .border_color(rgb(colors.border))
        .text_color(rgb(colors.text))
        // Placeholder would use colors.text_placeholder
}
```

### Badge

```rust
fn badge(label: &str, colors: &ThemeColors) -> impl IntoElement {
    div()
        .px_2()
        .py_0p5()
        .rounded(px(4.))
        .bg(rgb(colors.element))
        .text_xs()
        .text_color(rgb(colors.text_muted))
        .child(label)
}
```

### Status Badge

```rust
fn status_badge(label: &str, status: Status, colors: &ThemeColors) -> impl IntoElement {
    let (bg, fg) = match status {
        Status::Success => (colors.status_success_background, colors.status_success),
        Status::Warning => (colors.status_warning_background, colors.status_warning),
        Status::Error => (colors.status_error_background, colors.status_error),
        Status::Info => (colors.status_info_background, colors.status_info),
    };

    div()
        .px_2()
        .py_0p5()
        .rounded(px(4.))
        .bg(rgb(bg))
        .text_xs()
        .text_color(rgb(fg))
        .child(label)
}
```

### Modal/Dialog

```rust
fn modal(colors: &ThemeColors) -> Div {
    div()
        .rounded_lg()
        .bg(rgb(colors.elevated_surface))
        .border_1()
        .border_color(rgb(colors.border))
        .shadow_xl()
        .p_6()
}
```

### Sidebar Navigation Item

```rust
fn nav_item(label: &str, selected: bool, colors: &ThemeColors) -> impl IntoElement {
    let bg = if selected { colors.element_selected } else { colors.ghost_element_background };

    div()
        .px_3()
        .py_2()
        .rounded_md()
        .bg(rgb(bg))
        .text_color(rgb(colors.text))
        .hover(|s| s.bg(rgb(colors.element_hover)))
        .child(label)
}
```

## When a Token Doesn't Exist

If you need a color that existing tokens don't cover:

1. **First, try existing tokens** - Most UI patterns can use existing semantic tokens
2. **Consider deriving** - Can you derive from an existing token? (e.g., add alpha to `accent`)
3. **Ask before adding** - If neither works, **stop and clarify with the user** whether a new token should be added

**Do NOT:**
- Hardcode a color value
- Guess what the token should be
- Add a new token without discussion

**Example conversation:**
> "I need a color for [description]. Existing tokens don't seem to fit because [reason]. Should I:
> A) Use `[closest_token]` even though it's not ideal
> B) Add a new semantic token like `[proposed_name]`
> C) Something else?"

This ensures the theme system grows intentionally rather than accumulating ad-hoc tokens.

## Checklist for New Components

Before submitting a new component, verify:

- [ ] Uses `ActiveTheme` trait or receives `&ThemeColors`
- [ ] No hardcoded color values (no `rgb(0x...)` literals)
- [ ] Backgrounds follow layering hierarchy
- [ ] Interactive elements have hover/active states
- [ ] Text uses appropriate `text`/`text_muted`/`text_accent` tokens
- [ ] Borders use `border` or `border_variant` appropriately
- [ ] Status indicators use status color variants
- [ ] Primary actions use `accent`/`accent_foreground`
- [ ] Disabled states use `*_disabled` tokens where applicable
- [ ] If a needed token doesn't exist, clarified with user before proceeding

## Quick Reference

```
Backgrounds:  background < panel_background < surface < elevated_surface < element
Elements:     element → element_hover → element_active | element_selected | element_disabled
Text:         text | text_muted | text_accent | text_disabled | text_placeholder
Icons:        icon | icon_muted | icon_accent | icon_disabled
Borders:      border | border_variant | border_focused | border_selected | border_disabled
Status:       status_{type} | status_{type}_background | status_{type}_border
Accent:       accent + accent_foreground
```
