use crate::components::TextInput;
use crate::icons::icon_sm;
use crate::postcommander::page::PostCommanderPage;
use crate::theme::ActiveTheme;
use gpui::*;

impl PostCommanderPage {
    pub fn render_connection_dialog(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let surface = colors.surface;
        let border = colors.border;
        let text = colors.text;
        let text_muted = colors.text_muted;
        let element = colors.element;
        let element_hover = colors.element_hover;
        let accent = colors.accent;
        let accent_foreground = colors.accent_foreground;
        let border_variant = colors.border_variant;
        let background = colors.background;

        deferred(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .id("connection-backdrop")
                        .absolute()
                        .inset_0()
                        .bg(rgba(0x00000088))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.show_connection_dialog = false;
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .id("connection-dialog-panel")
                        .relative()
                        .w(px(420.))
                        .p_5()
                        .rounded_xl()
                        .bg(rgb(background))
                        .border_1()
                        .border_color(rgb(border))
                        .shadow_xl()
                        .occlude()
                        .child(self.render_dialog_header(cx, text, text_muted, element_hover))
                        .child(self.render_dialog_form(cx, text_muted, text, surface, border_variant, accent))
                        .child(self.render_dialog_buttons(
                            cx,
                            element,
                            element_hover,
                            text,
                            accent,
                            accent_foreground,
                        )),
                ),
        )
    }

    fn render_dialog_header(
        &self,
        cx: &mut Context<Self>,
        text: u32,
        text_muted: u32,
        element_hover: u32,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .mb_5()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(icon_sm("database", text_muted))
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(text))
                            .child("Connect to PostgreSQL"),
                    ),
            )
            .child(
                div()
                    .id("close-dialog")
                    .size(px(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .cursor_pointer()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.show_connection_dialog = false;
                        cx.notify();
                    }))
                    .child(icon_sm("x", text_muted)),
            )
    }

    fn render_dialog_form(
        &self,
        cx: &mut Context<Self>,
        text_muted: u32,
        text: u32,
        surface: u32,
        border_variant: u32,
        accent: u32,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(self.render_input_field(cx, "Host", self.input_host.clone(), text_muted, text, surface, border_variant, accent, true))
                    .child(self.render_input_field(cx, "Port", self.input_port.clone(), text_muted, text, surface, border_variant, accent, false).w(px(100.))),
            )
            .child(self.render_input_field(cx, "Database", self.input_database.clone(), text_muted, text, surface, border_variant, accent, false))
            .child(self.render_input_field(cx, "Username", self.input_username.clone(), text_muted, text, surface, border_variant, accent, false))
            .child(self.render_input_field(cx, "Password", self.input_password.clone(), text_muted, text, surface, border_variant, accent, false))
    }

    fn render_input_field(
        &self,
        cx: &mut Context<Self>,
        label: &str,
        input: Entity<TextInput>,
        text_muted: u32,
        text: u32,
        surface: u32,
        border_variant: u32,
        _accent: u32,
        flex_grow: bool,
    ) -> Div {
        input.update(cx, |input, _| {
            input.set_colors(text, text_muted);
        });

        let mut container = div();
        if flex_grow {
            container = container.flex_1();
        }

        container
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(text_muted))
                    .child(label.to_string()),
            )
            .child(
                div()
                    .id(SharedString::from(format!("input-container-{}", label.to_lowercase())))
                    .h(px(36.))
                    .px_3()
                    .flex()
                    .items_center()
                    .rounded_lg()
                    .bg(rgb(surface))
                    .border_1()
                    .border_color(rgb(border_variant))
                    .child(
                        div()
                            .w_full()
                            .text_sm()
                            .line_height(px(20.))
                            .child(input),
                    ),
            )
    }

    fn render_dialog_buttons(
        &self,
        cx: &mut Context<Self>,
        element: u32,
        element_hover: u32,
        text: u32,
        accent: u32,
        accent_foreground: u32,
    ) -> impl IntoElement {
        div()
            .flex()
            .justify_end()
            .gap_3()
            .mt_6()
            .child(
                div()
                    .id("cancel-btn")
                    .h(px(36.))
                    .px_5()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_lg()
                    .bg(rgb(element))
                    .cursor_pointer()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.show_connection_dialog = false;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text))
                            .child("Cancel"),
                    ),
            )
            .child(
                div()
                    .id("connect-btn")
                    .h(px(36.))
                    .px_5()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .rounded_lg()
                    .bg(rgb(accent))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.9))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.connect_to_database(cx);
                    }))
                    .child(icon_sm("plug", accent_foreground))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(accent_foreground))
                            .child("Connect"),
                    ),
            )
    }
}
