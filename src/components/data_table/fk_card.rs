use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::icons::icon_sm;
use crate::theme::ActiveTheme;

use super::types::{DataTableState, DraggedFkCard, FkHoverCardData};

pub(crate) fn render_fk_card(
    card: &FkHoverCardData,
    position: Point<Pixels>,
    state: Entity<DataTableState>,
    cx: &App,
) -> impl IntoElement {
    let theme = cx.theme();
    let colors = theme.colors();
    let surface = colors.surface;
    let border = colors.border;
    let text = colors.text;
    let text_muted = colors.text_muted;
    let accent = colors.accent;
    let element_hover = colors.element_hover;

    let table_name = card.fk_info.referenced_table.clone();
    let fk_condition = format!(
        "{} = {}",
        card.fk_info.referenced_column, card.cell_value
    );
    let is_loading = card.is_loading;
    let referenced_row = card.referenced_row.clone();
    let error = card.error.clone();
    let state_for_dismiss = state.clone();
    let state_for_drag = state.clone();
    let state_for_drag_end = state.clone();

    let final_position = position + card.drag_offset;

    deferred(
        anchored()
            .position(final_position)
            .child(
                div()
                    .id("fk-card")
                    .occlude()
                    .w(px(320.))
                    .max_h(px(400.))
                    .bg(rgb(surface))
                    .border_1()
                    .border_color(rgb(border))
                    .rounded_lg()
                    .shadow_xl()
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .child(
                        div()
                            .id("fk-card-header")
                            .px_3()
                            .py_2()
                            .border_b_1()
                            .border_color(rgb(border))
                            .flex()
                            .justify_between()
                            .items_center()
                            .on_drag(DraggedFkCard, move |_, _, _, cx| {
                                cx.new(|_| DraggedFkCard)
                            })
                            .on_drag_move::<DraggedFkCard>(move |event, _window, cx| {
                                state_for_drag.update(cx, |state, cx| {
                                    if state.fk_card_drag_start.is_none() {
                                        state.start_fk_card_drag(event.event.position);
                                    } else {
                                        state.update_fk_card_drag(event.event.position, cx);
                                    }
                                });
                            })
                            .on_mouse_up(MouseButton::Left, move |_, _window, cx| {
                                state_for_drag_end.update(cx, |state, _cx| {
                                    state.end_fk_card_drag();
                                });
                            })
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(text))
                                            .child(table_name),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child(fk_condition),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .id("fk-hover-close")
                                            .p_1()
                                            .rounded_md()
                                            .cursor_pointer()
                                            .hover(|s| s.bg(rgb(element_hover)))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                move |_, _window, cx| {
                                                    state_for_dismiss.update(cx, |state, cx| {
                                                        state.hide_fk_card(cx);
                                                    });
                                                },
                                            )
                                            .child(icon_sm("x", text_muted)),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .id("fk-hover-card-content")
                            .p_2()
                            .max_h(px(300.))
                            .overflow_y_scroll()
                            .child(if is_loading {
                                div()
                                    .py_4()
                                    .flex()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(text_muted))
                                            .child("Loading..."),
                                    )
                                    .into_any_element()
                            } else if let Some(err) = error {
                                div()
                                    .py_4()
                                    .px_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(text_muted))
                                            .child(format!("Error: {}", err)),
                                    )
                                    .into_any_element()
                            } else if let Some(rows) = referenced_row {
                                if rows.is_empty() {
                                    div()
                                        .py_4()
                                        .flex()
                                        .justify_center()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(text_muted))
                                                .italic()
                                                .child("Record not found"),
                                        )
                                        .into_any_element()
                                } else {
                                    div()
                                        .flex()
                                        .flex_col()
                                        .children(rows.into_iter().map(|(col_name, value)| {
                                            let is_null = value == "NULL";
                                            div()
                                                .flex()
                                                .py_1()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .w(px(100.))
                                                        .flex_shrink_0()
                                                        .text_xs()
                                                        .text_color(rgb(text_muted))
                                                        .overflow_hidden()
                                                        .whitespace_nowrap()
                                                        .text_ellipsis()
                                                        .child(col_name),
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .text_xs()
                                                        .font_family("monospace")
                                                        .text_color(rgb(if is_null {
                                                            text_muted
                                                        } else {
                                                            accent
                                                        }))
                                                        .when(is_null, |el| el.italic())
                                                        .overflow_hidden()
                                                        .whitespace_nowrap()
                                                        .text_ellipsis()
                                                        .child(if is_null {
                                                            "NULL".to_string()
                                                        } else {
                                                            value
                                                        }),
                                                )
                                        }))
                                        .into_any_element()
                                }
                            } else {
                                div()
                                    .py_4()
                                    .flex()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(text_muted))
                                            .italic()
                                            .child("No data"),
                                    )
                                    .into_any_element()
                            }),
                    ),
            ),
    )
    .with_priority(2)
}
