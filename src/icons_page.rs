use crate::icons::{icon_lg, AVAILABLE_ICONS};
use crate::theme::ActiveTheme;
use gpui::*;

pub struct IconsPage;

impl IconsPage {
    pub fn new() -> Self {
        Self
    }
}

impl Render for IconsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(colors.background))
            .child(
                div()
                    .pt_4()
                    .px_6()
                    .pb_4()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(colors.text))
                            .child("Icon Library"),
                    )
                    .child(
                        div()
                            .mt_1()
                            .text_sm()
                            .text_color(rgb(colors.text_muted))
                            .child(format!("{} Lucide icons available", AVAILABLE_ICONS.len())),
                    ),
            )
            .child(
                div()
                    .id("icons-scroll")
                    .flex_1()
                    .px_6()
                    .pb_6()
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_4()
                            .children(AVAILABLE_ICONS.iter().map(|name| {
                                let name_string: SharedString = (*name).into();
                                div()
                                    .w(px(120.))
                                    .p_4()
                                    .rounded_lg()
                                    .bg(rgb(colors.element))
                                    .border_1()
                                    .border_color(rgb(colors.border_variant))
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_3()
                                    .child(icon_lg(name, colors.text))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(colors.text_muted))
                                            .overflow_hidden()
                                            .text_ellipsis()
                                            .child(name_string),
                                    )
                            })),
                    ),
            )
    }
}
