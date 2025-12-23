mod builtin;
mod registry;

pub use registry::*;

use gpui::{App, BorrowAppContext, Global, Hsla, SharedString};
use std::sync::Arc;

fn hex_to_hsla(hex: u32) -> Hsla {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if max == min {
        return Hsla { h: 0.0, s: 0.0, l, a: 1.0 };
    }

    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

    let h = if max == r {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if max == g {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    Hsla { h, s, l, a: 1.0 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Appearance {
    Light,
    Dark,
}

impl Appearance {
    pub fn is_light(&self) -> bool {
        matches!(self, Self::Light)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ThemeColors {
    // Backgrounds (layered from darkest to lightest in dark themes)
    pub background: u32,
    pub panel_background: u32,
    pub editor_background: u32,
    pub surface: u32,
    pub elevated_surface: u32,

    // Element states
    pub element: u32,
    pub element_hover: u32,
    pub element_selected: u32,
    pub element_active: u32,
    #[allow(dead_code)]
    pub element_disabled: u32,
    #[allow(dead_code)]
    pub ghost_element_background: u32,
    #[allow(dead_code)]
    pub ghost_element_hover: u32,

    // Borders
    pub border: u32,
    pub border_variant: u32,
    #[allow(dead_code)]
    pub border_focused: u32,
    #[allow(dead_code)]
    pub border_selected: u32,
    #[allow(dead_code)]
    pub border_disabled: u32,

    // Text
    pub text: u32,
    pub text_muted: u32,
    pub text_accent: u32,
    #[allow(dead_code)]
    pub text_disabled: u32,
    pub text_placeholder: u32,

    // Icons
    #[allow(dead_code)]
    pub icon: u32,
    #[allow(dead_code)]
    pub icon_muted: u32,
    #[allow(dead_code)]
    pub icon_disabled: u32,
    #[allow(dead_code)]
    pub icon_accent: u32,

    // Status colors (foreground)
    pub status_success: u32,
    pub status_warning: u32,
    pub status_error: u32,
    pub status_info: u32,

    // Status backgrounds (subtle, for badges/alerts)
    pub status_success_background: u32,
    #[allow(dead_code)]
    pub status_warning_background: u32,
    pub status_error_background: u32,
    #[allow(dead_code)]
    pub status_info_background: u32,

    // Status borders
    #[allow(dead_code)]
    pub status_success_border: u32,
    #[allow(dead_code)]
    pub status_warning_border: u32,
    #[allow(dead_code)]
    pub status_error_border: u32,
    #[allow(dead_code)]
    pub status_info_border: u32,

    // Accent for primary actions (buttons, links)
    pub accent: u32,
    pub accent_foreground: u32,
}

#[derive(Clone, Debug)]
pub struct Theme {
    #[allow(dead_code)]
    pub id: String,
    pub name: SharedString,
    pub appearance: Appearance,
    pub colors: ThemeColors,
}

impl Theme {
    pub fn colors(&self) -> &ThemeColors {
        &self.colors
    }
}

pub struct GlobalTheme {
    theme: Arc<Theme>,
}

impl Global for GlobalTheme {}

impl GlobalTheme {
    pub fn init(cx: &mut App) {
        let registry = ThemeRegistry::global(cx);
        let settings = crate::settings::AppSettings::get_global(cx);
        let theme = registry
            .get(&settings.theme_name)
            .unwrap_or_else(|| registry.get("One Dark").unwrap());
        cx.set_global(GlobalTheme { theme: theme.clone() });
        Self::sync_to_gpui_component(&theme, cx);
    }

    pub fn theme(cx: &App) -> &Arc<Theme> {
        &cx.global::<Self>().theme
    }

    pub fn reload_theme(cx: &mut App) {
        let registry = ThemeRegistry::global(cx);
        let settings = crate::settings::AppSettings::get_global(cx);
        let theme = registry
            .get(&settings.theme_name)
            .unwrap_or_else(|| registry.get("One Dark").unwrap());
        cx.update_global::<Self, _>(|this, _| this.theme = theme.clone());
        Self::sync_to_gpui_component(&theme, cx);
        cx.refresh_windows();
    }

    fn sync_to_gpui_component(theme: &Theme, cx: &mut App) {
        use gpui_component::highlighter::HighlightTheme;
        use gpui_component::theme::{Theme as GpuiTheme, ThemeMode};

        let colors = &theme.colors;
        let mode = if theme.appearance.is_light() {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };

        let gpui_theme = GpuiTheme::global_mut(cx);
        gpui_theme.mode = mode;

        gpui_theme.background = hex_to_hsla(colors.element);
        gpui_theme.foreground = hex_to_hsla(colors.text);
        gpui_theme.muted = hex_to_hsla(colors.element);
        gpui_theme.muted_foreground = hex_to_hsla(colors.text_muted);
        gpui_theme.border = hex_to_hsla(colors.border);
        gpui_theme.input = hex_to_hsla(colors.border);
        gpui_theme.ring = hex_to_hsla(colors.accent);
        gpui_theme.primary = hex_to_hsla(colors.accent);
        gpui_theme.primary_hover = hex_to_hsla(colors.accent);
        gpui_theme.primary_active = hex_to_hsla(colors.accent);
        gpui_theme.primary_foreground = hex_to_hsla(colors.accent_foreground);
        gpui_theme.secondary = hex_to_hsla(colors.element);
        gpui_theme.secondary_hover = hex_to_hsla(colors.element_hover);
        gpui_theme.secondary_active = hex_to_hsla(colors.element_active);
        gpui_theme.secondary_foreground = hex_to_hsla(colors.text);
        gpui_theme.accent = hex_to_hsla(colors.element_hover);
        gpui_theme.accent_foreground = hex_to_hsla(colors.text_muted);
        gpui_theme.danger = hex_to_hsla(colors.status_error);
        gpui_theme.danger_foreground = hex_to_hsla(colors.accent_foreground);
        gpui_theme.success = hex_to_hsla(colors.status_success);
        gpui_theme.success_foreground = hex_to_hsla(colors.accent_foreground);
        gpui_theme.warning = hex_to_hsla(colors.status_warning);
        gpui_theme.warning_foreground = hex_to_hsla(colors.accent_foreground);
        gpui_theme.popover = hex_to_hsla(colors.elevated_surface);
        gpui_theme.popover_foreground = hex_to_hsla(colors.text);
        gpui_theme.list = hex_to_hsla(colors.surface);
        gpui_theme.list_hover = hex_to_hsla(colors.element_hover);
        gpui_theme.list_active = hex_to_hsla(colors.element_active);
        gpui_theme.table = hex_to_hsla(colors.background);
        gpui_theme.table_head = hex_to_hsla(colors.element);
        gpui_theme.table_head_foreground = hex_to_hsla(colors.text);
        gpui_theme.table_hover = hex_to_hsla(colors.element_hover);
        gpui_theme.scrollbar_thumb = hex_to_hsla(colors.border);
        gpui_theme.scrollbar_thumb_hover = hex_to_hsla(colors.text_muted);
        gpui_theme.caret = hex_to_hsla(colors.text);

        gpui_theme.chart_1 = hex_to_hsla(colors.accent);
        gpui_theme.chart_2 = hex_to_hsla(colors.status_success);
        gpui_theme.chart_3 = hex_to_hsla(colors.status_warning);
        gpui_theme.chart_4 = hex_to_hsla(colors.status_error);
        gpui_theme.chart_5 = hex_to_hsla(colors.status_info);

        let base_theme = if theme.appearance.is_light() {
            HighlightTheme::default_light()
        } else {
            HighlightTheme::default_dark()
        };
        let mut highlight_theme = (*base_theme).clone();
        highlight_theme.style.editor_background = Some(hex_to_hsla(colors.elevated_surface));
        gpui_theme.highlight_theme = Arc::new(highlight_theme);
    }
}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        GlobalTheme::theme(self)
    }
}

impl<T> ActiveTheme for gpui::Context<'_, T> {
    fn theme(&self) -> &Arc<Theme> {
        GlobalTheme::theme(self)
    }
}
