use crate::postcommander::sql::SqlDangerLevel;
use crate::postcommander::PostCommanderPage;
use crate::theme::ActiveTheme;
use gpui::*;

impl PostCommanderPage {
    pub(crate) fn render_safety_warning_dialog(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        let Some((danger_level, message)) = &self.safety_warning else {
            return div().into_any_element();
        };

        let is_dangerous = matches!(danger_level, SqlDangerLevel::Dangerous(_));
        let title = if is_dangerous { "Dangerous Query" } else { "Warning" };
        let icon_color = if is_dangerous { colors.status_error } else { colors.status_warning };

        div()
            .id("safety-warning-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x00000080))
            .on_mouse_down(MouseButton::Left, |_, _, _| {})
            .child(
                div()
                    .w(px(420.0))
                    .bg(rgb(colors.surface))
                    .border_1()
                    .border_color(rgb(colors.border))
                    .rounded_lg()
                    .shadow_lg()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                crate::icons::icon("alert-triangle", px(24.0), icon_color)
                            )
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(rgb(colors.text))
                                    .child(title)
                            )
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(colors.text_muted))
                            .child(message.clone())
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                div()
                                    .id("cancel-btn")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_md()
                                    .bg(rgb(colors.element))
                                    .text_sm()
                                    .text_color(rgb(colors.text))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(colors.element_hover)))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.cancel_dangerous_query(cx);
                                    }))
                                    .child("Cancel")
                            )
                            .child(
                                div()
                                    .id("proceed-btn")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_md()
                                    .bg(rgb(icon_color))
                                    .text_sm()
                                    .text_color(rgb(colors.accent_foreground))
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.9))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.execute_query_force(cx);
                                    }))
                                    .child("Execute Anyway")
                            )
                    )
            )
            .into_any_element()
    }

    pub(crate) fn render_temporary_message(&self, message: &str, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let message_text = message.to_string();

        div()
            .id("temporary-message-overlay")
            .absolute()
            .top(px(16.0))
            .right(px(16.0))
            .child(
                div()
                    .px_4()
                    .py_3()
                    .bg(rgb(colors.surface))
                    .border_1()
                    .border_color(rgb(colors.border))
                    .rounded_lg()
                    .shadow_lg()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        crate::icons::icon_sm("info", colors.accent)
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(colors.text))
                            .child(message_text)
                    )
            )
            .into_any_element()
    }
}
