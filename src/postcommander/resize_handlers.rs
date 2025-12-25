use crate::postcommander::PostCommanderPage;
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;

impl PostCommanderPage {
    pub(crate) fn render_resize_handle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let border_variant = colors.border_variant;
        let accent = colors.accent;
        let is_resizing = self.resize.is_resizing_sidebar;

        div()
            .id("sidebar-resize-handle")
            .w(px(4.))
            .h_full()
            .cursor_col_resize()
            .bg(transparent_black())
            .when(is_resizing, |el| el.bg(rgb(accent)))
            .hover(move |s| s.bg(rgb(border_variant)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.resize.is_resizing_sidebar = true;
                    this.resize.resize_sidebar_start_x = f32::from(event.position.x);
                    this.resize.resize_sidebar_start_width = this.resize.sidebar_width;
                    cx.notify();
                }),
            )
    }

    pub(crate) fn render_resize_overlay(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("resize-overlay")
            .absolute()
            .inset_0()
            .cursor_col_resize()
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if this.resize.is_resizing_sidebar {
                    let delta = f32::from(event.position.x) - this.resize.resize_sidebar_start_x;
                    let new_width = (this.resize.resize_sidebar_start_width + delta).clamp(180.0, 500.0);
                    this.resize.sidebar_width = new_width;
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.resize.is_resizing_sidebar = false;
                    this.save_sidebar_width(cx);
                    cx.notify();
                }),
            )
    }

    pub(crate) fn render_editor_resize_handle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let border_variant = colors.border_variant;
        let accent = colors.accent;
        let is_resizing = self.resize.is_resizing_editor;

        div()
            .id("editor-resize-handle")
            .w_full()
            .h(px(4.))
            .cursor_row_resize()
            .bg(transparent_black())
            .when(is_resizing, |el| el.bg(rgb(accent)))
            .hover(move |s| s.bg(rgb(border_variant)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.resize.is_resizing_editor = true;
                    this.resize.resize_editor_start_y = f32::from(event.position.y);
                    this.resize.resize_editor_start_height = this.resize.editor_height;
                    cx.notify();
                }),
            )
    }

    pub(crate) fn render_editor_resize_overlay(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("editor-resize-overlay")
            .absolute()
            .inset_0()
            .cursor_row_resize()
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if this.resize.is_resizing_editor {
                    let delta = f32::from(event.position.y) - this.resize.resize_editor_start_y;
                    let new_height = (this.resize.resize_editor_start_height + delta).clamp(100.0, 600.0);
                    this.resize.editor_height = new_height;
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.resize.is_resizing_editor = false;
                    this.save_editor_height(cx);
                    cx.notify();
                }),
            )
    }

    pub(crate) fn save_sidebar_width(&self, cx: &mut Context<Self>) {
        use crate::settings::AppSettings;
        AppSettings::update_global(cx, |settings| {
            settings.postcommander_mut().sidebar_width = Some(self.resize.sidebar_width);
        });
        AppSettings::get_global(cx).save();
    }

    pub(crate) fn save_editor_height(&self, cx: &mut Context<Self>) {
        use crate::settings::AppSettings;
        AppSettings::update_global(cx, |settings| {
            settings.postcommander_mut().editor_height = Some(self.resize.editor_height);
        });
        AppSettings::get_global(cx).save();
    }
}
