use super::card::render_card;
use super::column::{ColumnStatus, KanbanColumn};
use super::{DraggedCard, KanbanCard};
use gpui::prelude::FluentBuilder;
use gpui::*;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Deserialize)]
struct KanbanData {
    columns: Vec<ColumnData>,
}

#[derive(Clone, Deserialize)]
struct ColumnData {
    #[allow(dead_code)]
    id: String,
    title: String,
    #[allow(dead_code)]
    color: String,
    cards: Vec<CardData>,
}

#[derive(Clone, Deserialize)]
struct CardData {
    id: String,
    title: String,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    category_color: Option<String>,
    #[serde(default)]
    date: Option<String>,
    #[serde(default)]
    estimate: Option<String>,
    #[serde(default)]
    log_time: Option<String>,
    #[serde(default)]
    comments: Option<u32>,
}

fn parse_hex_color(s: &str) -> u32 {
    let s = s.trim_start_matches("0x").trim_start_matches('#');
    u32::from_str_radix(s, 16).unwrap_or(0xe5e7eb)
}

pub struct KanbanBoard {
    columns: Vec<KanbanColumn>,
}

impl KanbanBoard {
    pub fn load(_cx: &mut Context<Self>) -> Self {
        let json_path = "data/kanban.json";
        let json_str = fs::read_to_string(json_path).expect("Failed to read kanban.json");
        let data: KanbanData = serde_json::from_str(&json_str).expect("Failed to parse JSON");

        Self {
            columns: data
                .columns
                .into_iter()
                .enumerate()
                .map(|(idx, col)| KanbanColumn {
                    title: col.title.into(),
                    status: ColumnStatus::from_index(idx),
                    cards: col
                        .cards
                        .into_iter()
                        .map(|card| KanbanCard {
                            id: card.id,
                            title: card.title.into(),
                            category: card.category.unwrap_or_else(|| "Task".to_string()).into(),
                            category_color: card
                                .category_color
                                .map(|c| parse_hex_color(&c))
                                .unwrap_or(0xe5e7eb),
                            date: card.date.unwrap_or_else(|| "".to_string()).into(),
                            estimate: card.estimate.map(|e| e.into()),
                            log_time: card.log_time.map(|l| l.into()),
                            comments: card.comments.unwrap_or(0),
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    fn move_card(&mut self, card_id: &str, from_column: usize, to_column: usize) {
        if from_column == to_column {
            return;
        }
        if let Some(card_idx) = self.columns[from_column]
            .cards
            .iter()
            .position(|c| c.id == card_id)
        {
            let card = self.columns[from_column].cards.remove(card_idx);
            self.columns[to_column].cards.push(card);
        }
    }
}

use crate::theme::ThemeColors;

fn render_header(colors: &ThemeColors) -> impl IntoElement {
    let text_dark = colors.text;
    let text_muted = colors.text_muted;
    let tab_bg = colors.element;
    let tab_selected_bg = colors.elevated_surface;
    let tab_hover_bg = colors.element_hover;
    let button_bg = colors.surface;
    let button_hover_bg = colors.element_hover;
    let border_color = colors.border_variant;
    let accent = colors.accent;
    let accent_foreground = colors.accent_foreground;

    let tabs = vec![
        ("▦", "Board", true),
        ("☰", "List", false),
        ("⊞", "Gantt", false),
        ("📅", "Calendar", false),
        ("▤", "Table", false),
    ];

    div()
        .pt(px(44.))
        .px_6()
        .pb_4()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_2xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(text_dark))
                        .child("Workspace"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(text_muted))
                        .child("⋮"),
                ),
        )
        .child(
            div()
                .mt_4()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_1()
                        .py_1()
                        .rounded_lg()
                        .bg(rgb(tab_bg))
                        .children(tabs.into_iter().map(move |(icon, label, selected)| {
                            let bg = if selected { tab_selected_bg } else { tab_bg };
                            div()
                                .px_3()
                                .py(px(6.))
                                .rounded_md()
                                .bg(rgb(bg))
                                .when(selected, |el| el.shadow_sm())
                                .cursor_pointer()
                                .hover(move |s| if !selected { s.bg(rgb(tab_hover_bg)) } else { s })
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(text_muted))
                                                .child(icon),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(text_dark))
                                                .child(label),
                                        ),
                                )
                        })),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .px_3()
                                .py(px(6.))
                                .rounded_lg()
                                .bg(rgb(button_bg))
                                .border_1()
                                .border_color(rgb(border_color))
                                .cursor_pointer()
                                .hover(move |s| s.bg(rgb(button_hover_bg)))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(text_muted))
                                                .child("↗"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(text_dark))
                                                .child("Share"),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .px_3()
                                .py(px(6.))
                                .rounded_lg()
                                .bg(rgb(button_bg))
                                .border_1()
                                .border_color(rgb(border_color))
                                .cursor_pointer()
                                .hover(move |s| s.bg(rgb(button_hover_bg)))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(text_muted))
                                                .child("≡"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(text_dark))
                                                .child("Filters"),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .px_3()
                                .py(px(6.))
                                .rounded_lg()
                                .bg(rgb(button_bg))
                                .border_1()
                                .border_color(rgb(border_color))
                                .cursor_pointer()
                                .hover(move |s| s.bg(rgb(button_hover_bg)))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(text_muted))
                                                .child("☰"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(text_dark))
                                                .child("Group by: Status"),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .px_4()
                                .py(px(6.))
                                .rounded_lg()
                                .bg(rgb(accent))
                                .cursor_pointer()
                                .hover(move |s| s.bg(rgb(accent)))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(accent_foreground))
                                        .child("Add Task"),
                                ),
                        ),
                ),
        )
}

fn render_empty_column(
    title: SharedString,
    card_count: usize,
    col_idx: usize,
    colors: &ThemeColors,
    cx: &mut Context<KanbanBoard>,
) -> impl IntoElement {
    let text_dark = colors.text;
    let text_muted = colors.text_muted;
    let badge_bg = colors.element;
    let border_color = colors.border_variant;
    let accent = colors.accent;
    let accent_bg = colors.element_hover;
    let drag_highlight = (colors.accent << 8) | 0x20;

    div()
        .id(SharedString::from(format!("column-{}", col_idx)))
        .flex()
        .flex_col()
        .w(px(280.))
        .h_full()
        .drag_over::<DraggedCard>(move |style, _, _, _| {
            style.bg(rgba(drag_highlight)).rounded_lg()
        })
        .on_drop(cx.listener(move |this, dragged: &DraggedCard, _window, cx| {
            this.move_card(&dragged.card_id, dragged.from_column, col_idx);
            cx.notify();
        }))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .px_2()
                .pb_3()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(text_dark))
                                .child(title),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.))
                                .bg(rgb(badge_bg))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(text_muted))
                                        .child(format!("{}", card_count)),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(text_muted))
                                .cursor_pointer()
                                .child("+"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(text_muted))
                                .cursor_pointer()
                                .child("⋮"),
                        ),
                ),
        )
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .size(px(48.))
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(border_color))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_xl()
                                .text_color(rgb(text_muted))
                                .child("📄"),
                        ),
                )
                .child(
                    div()
                        .mt_3()
                        .text_sm()
                        .text_color(rgb(text_muted))
                        .child("No tasks currently. Board is empty"),
                )
                .child(
                    div()
                        .mt_3()
                        .px_4()
                        .py_2()
                        .rounded_md()
                        .bg(rgb(accent_bg))
                        .cursor_pointer()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(accent))
                                .child("Create Task"),
                        ),
                ),
        )
}

impl Render for KanbanBoard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use crate::theme::ActiveTheme;
        let theme = cx.theme();
        let colors = *theme.colors();

        let text_dark = colors.text;
        let text_muted = colors.text_muted;
        let drag_highlight = (colors.accent << 8) | 0x20;

        let columns: Vec<_> = self
            .columns
            .iter()
            .enumerate()
            .map(|(col_idx, column)| {
                let card_count = column.cards.len();
                let column_title = column.title.clone();
                let is_empty = card_count == 0;
                let status = column.status;

                if is_empty {
                    return render_empty_column(column_title, card_count, col_idx, &colors, cx).into_any_element();
                }

                let cards: Vec<_> = column
                    .cards
                    .iter()
                    .map(|card| render_card(card, col_idx, &colors))
                    .collect();

                let count_bg = status.accent_color(&colors);

                div()
                    .id(SharedString::from(format!("column-{}", col_idx)))
                    .flex()
                    .flex_col()
                    .w(px(280.))
                    .h_full()
                    .rounded_lg()
                    .drag_over::<DraggedCard>(move |style, _, _, _| {
                        style.bg(rgba(drag_highlight))
                    })
                    .on_drop(cx.listener(move |this, dragged: &DraggedCard, _window, cx| {
                        this.move_card(&dragged.card_id, dragged.from_column, col_idx);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_2()
                            .pb_3()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(text_dark))
                                            .child(column_title),
                                    )
                                    .child(
                                        div()
                                            .px_2()
                                            .py_0p5()
                                            .rounded(px(4.))
                                            .bg(rgb(count_bg))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(white())
                                                    .child(format!("{}", card_count)),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(text_muted))
                                            .cursor_pointer()
                                            .child("+"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(text_muted))
                                            .cursor_pointer()
                                            .child("⋮"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .children(cards),
                    )
                    .into_any_element()
            })
            .collect();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(colors.background))
            .child(render_header(&colors))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .gap_6()
                    .px_6()
                    .pb_4()
                    .children(columns),
            )
    }
}
