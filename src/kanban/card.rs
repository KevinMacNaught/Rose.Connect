use crate::theme::ThemeColors;
use gpui::prelude::FluentBuilder;
use gpui::*;

#[derive(Clone)]
pub struct DraggedCard {
    pub card_id: String,
    pub card: KanbanCard,
    pub from_column: usize,
}

#[derive(Clone)]
pub struct KanbanCard {
    pub id: String,
    pub title: SharedString,
    pub category: SharedString,
    #[allow(dead_code)]
    pub category_color: u32,
    pub date: SharedString,
    pub estimate: Option<SharedString>,
    pub log_time: Option<SharedString>,
    pub comments: u32,
}

pub struct DragPreview {
    card: KanbanCard,
}

impl DragPreview {
    pub fn new(card: KanbanCard) -> Self {
        Self { card }
    }
}

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use crate::theme::ActiveTheme;
        let theme = cx.theme();
        let colors = theme.colors();

        let card = &self.card;
        let category = card.category.clone();
        let title = card.title.clone();
        let date = card.date.clone();
        let estimate = card.estimate.clone();
        let log_time = card.log_time.clone();
        let comments = card.comments;

        let card_bg = colors.elevated_surface;
        let element_bg = colors.element;
        let text_primary = colors.text;
        let text_muted = colors.text_muted;
        let status_success = colors.status_success;
        let status_success_bg = colors.status_success_background;

        div()
            .w(px(260.))
            .rounded_lg()
            .bg(rgb(card_bg))
            .shadow_lg()
            .opacity(0.9)
            .child(
                div()
                    .p_3()
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_start()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(4.))
                                    .bg(rgb(element_bg))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child(category),
                                    ),
                            )
                            .child(
                                div()
                                    .size(px(28.))
                                    .rounded_full()
                                    .bg(rgb(element_bg))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child("👤"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .mt_2()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text_primary))
                            .child(title),
                    )
                    .child(
                        div()
                            .mt_3()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child("📅"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child(date),
                                    ),
                            )
                            .when_some(estimate, |el, est| {
                                el.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(text_muted))
                                                .child("⏱"),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(text_muted))
                                                .child(est),
                                        ),
                                )
                            }),
                    )
                    .when_some(log_time, |el, log| {
                        el.child(
                            div()
                                .mt_2()
                                .flex()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .px_2()
                                        .py_1()
                                        .rounded(px(4.))
                                        .bg(rgb(status_success_bg))
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(status_success))
                                                .child("⏱"),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(rgb(status_success))
                                                .child(format!("Log: {}", log)),
                                        ),
                                )
                                .when(comments > 0, |el| {
                                    el.child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(text_muted))
                                                    .child("💬"),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(text_muted))
                                                    .child(format!("{}", comments)),
                                            ),
                                    )
                                }),
                        )
                    }),
            )
    }
}

pub fn render_card(card: &KanbanCard, col_idx: usize, colors: &ThemeColors) -> impl IntoElement {
    let dragged = DraggedCard {
        card_id: card.id.clone(),
        card: card.clone(),
        from_column: col_idx,
    };

    let category = card.category.clone();
    let title = card.title.clone();
    let date = card.date.clone();
    let estimate = card.estimate.clone();
    let log_time = card.log_time.clone();
    let comments = card.comments;

    let card_bg = colors.elevated_surface;
    let element_bg = colors.element;
    let text_primary = colors.text;
    let text_muted = colors.text_muted;
    let status_success = colors.status_success;
    let status_success_bg = colors.status_success_background;

    div()
        .id(SharedString::from(card.id.clone()))
        .mb_3()
        .rounded_lg()
        .bg(rgb(card_bg))
        .shadow_sm()
        .cursor_grab()
        .hover(|style| style.shadow_md())
        .on_drag(dragged, move |drag, _point, _window, cx| {
            cx.new(|_| DragPreview::new(drag.card.clone()))
        })
        .child(
            div()
                .p_3()
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .items_start()
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded(px(4.))
                                .bg(rgb(element_bg))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(text_muted))
                                        .child(category),
                                ),
                        )
                        .child(
                            div()
                                .size(px(28.))
                                .rounded_full()
                                .bg(rgb(element_bg))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(text_muted))
                                        .child("👤"),
                                ),
                        ),
                )
                .child(
                    div()
                        .mt_2()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(text_primary))
                        .child(title),
                )
                .child(
                    div()
                        .mt_3()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(text_muted))
                                        .child("📅"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(text_muted))
                                        .child(date),
                                ),
                        )
                        .when_some(estimate, |el, est| {
                            el.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child("⏱"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child(est),
                                    ),
                            )
                        }),
                )
                .when_some(log_time.clone(), |el, log| {
                    el.child(
                        div()
                            .mt_2()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(4.))
                                    .bg(rgb(status_success_bg))
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(status_success))
                                            .child("⏱"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(status_success))
                                            .child(format!("Log: {}", log)),
                                    ),
                            )
                            .when(comments > 0, |el| {
                                el.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(text_muted))
                                                .child("💬"),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(text_muted))
                                                .child(format!("{}", comments)),
                                        ),
                                )
                            }),
                    )
                })
                .when(log_time.is_none() && comments > 0, |el| {
                    el.child(
                        div()
                            .mt_2()
                            .flex()
                            .justify_end()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child("💬"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child(format!("{}", comments)),
                                    ),
                            ),
                    )
                }),
        )
}
