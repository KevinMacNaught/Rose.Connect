use crate::theme::ActiveTheme;
use gpui::App;

pub struct RenderColors {
    pub text: u32,
    pub text_muted: u32,
    pub text_placeholder: u32,
    pub background: u32,
    pub panel_background: u32,
    pub editor_background: u32,
    pub surface: u32,
    pub element: u32,
    pub element_hover: u32,
    pub element_active: u32,
    pub border: u32,
    pub border_variant: u32,
    pub accent: u32,
    pub accent_foreground: u32,
    pub status_success: u32,
    pub status_warning: u32,
    pub status_error: u32,
    pub status_error_background: u32,
    pub status_error_border: u32,
}

impl RenderColors {
    pub fn from_context(cx: &App) -> Self {
        let colors = cx.theme().colors();
        Self {
            text: colors.text,
            text_muted: colors.text_muted,
            text_placeholder: colors.text_placeholder,
            background: colors.background,
            panel_background: colors.panel_background,
            editor_background: colors.elevated_surface,
            surface: colors.surface,
            element: colors.element,
            element_hover: colors.element_hover,
            element_active: colors.element_active,
            border: colors.border,
            border_variant: colors.border_variant,
            accent: colors.accent,
            accent_foreground: colors.accent_foreground,
            status_success: colors.status_success,
            status_warning: colors.status_warning,
            status_error: colors.status_error,
            status_error_background: colors.status_error_background,
            status_error_border: colors.status_error_border,
        }
    }
}
