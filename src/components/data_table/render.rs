use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::checkbox::Checkbox;
use std::sync::Arc;

use crate::icons::icon_sm;
use crate::theme::ActiveTheme;

use super::fk_card::render_fk_card;
use super::resize::render_resize_handle;
use super::types::{
    CellContextMenu, CellDoubleClicked, CellSaveRequested, DataTableColumn, DataTableState,
    HEADER_HEIGHT, ROW_HEIGHT,
};

#[derive(IntoElement)]
pub struct DataTable {
    state: Entity<DataTableState>,
}

impl DataTable {
    pub fn new(state: Entity<DataTableState>) -> Self {
        Self { state }
    }
}

impl RenderOnce for DataTable {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);
        let theme = cx.theme();
        let colors = theme.colors();
        let background = colors.background;
        let panel_background = colors.panel_background;
        let border_variant = colors.border_variant;
        let text = colors.text;
        let text_muted = colors.text_muted;
        let accent = colors.accent;

        let row_height = px(ROW_HEIGHT);
        let header_height = px(HEADER_HEIGHT);

        let columns = &state.columns;
        let rows = state.rows.clone();
        let row_count = rows.len();

        let col_widths: Vec<Pixels> = columns.iter().map(|c| c.width).collect();
        let columns_width: Pixels = col_widths.iter().copied().sum();

        let scroll_offset = state.scroll_offset;
        let viewport_size = state.viewport_size;

        let scroll_y_after_header = (scroll_offset.y - header_height).max(px(0.));
        let first_visible_row = (f32::from(scroll_y_after_header) / ROW_HEIGHT).floor() as usize;
        let first_visible_row = first_visible_row.saturating_sub(2);
        let visible_count = (f32::from(viewport_size.height) / ROW_HEIGHT).ceil() as usize + 4;
        let last_visible_row = (first_visible_row + visible_count).min(row_count);

        let header = render_header(
            columns,
            &state,
            self.state.clone(),
            columns_width,
            header_height,
            scroll_offset,
            text,
            text_muted,
            accent,
            panel_background,
            border_variant,
        );

        let row_hover_bg = colors.element_hover;
        let cell_hover_bg = colors.element_selected;

        let column_names: Vec<SharedString> = columns.iter().map(|c| c.name.clone()).collect();
        let column_types: Vec<Option<SharedString>> =
            columns.iter().map(|c| c.type_name.clone()).collect();

        let foreign_keys = state
            .table_context
            .as_ref()
            .map(|ctx| ctx.foreign_keys.clone())
            .unwrap_or_else(|| Arc::new(std::collections::HashMap::new()));

        let visible_rows = render_visible_rows(
            first_visible_row,
            last_visible_row,
            &rows,
            &col_widths,
            columns_width,
            row_height,
            header_height,
            scroll_offset,
            background,
            panel_background,
            border_variant,
            text,
            text_muted,
            accent,
            row_hover_bg,
            cell_hover_bg,
            column_names,
            column_types,
            foreign_keys,
            self.state.clone(),
        );

        let state_for_scroll = self.state.clone();
        let state_for_measure = self.state.clone();
        let state_for_fk_card = self.state.clone();
        let active_fk_card = state.active_fk_card.clone();

        let fk_card_position = active_fk_card.as_ref().map(|card| {
            state.calculate_cell_position(card.row_index, card.col_index)
        });

        div()
            .id("data-table-container")
            .size_full()
            .relative()
            .child(
                canvas(
                    move |bounds, _window, cx| {
                        state_for_measure.update(cx, |state, _cx| {
                            state.set_container_origin(bounds.origin);
                            state.viewport_size = bounds.size;
                            let content_size = state.content_size();
                            let max_scroll_x = (content_size.width - bounds.size.width).max(px(0.));
                            let max_scroll_y = (content_size.height - bounds.size.height).max(px(0.));
                            state.scroll_offset.x = state.scroll_offset.x.clamp(px(0.), max_scroll_x);
                            state.scroll_offset.y = state.scroll_offset.y.clamp(px(0.), max_scroll_y);
                        });
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .inset_0(),
            )
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .overflow_hidden()
                    .on_scroll_wheel(move |event, _window, cx| {
                        state_for_scroll.update(cx, |state, cx| {
                            state.on_scroll(event, cx);
                        });
                    })
                    .child(
                        div()
                            .relative()
                            .size_full()
                            .children(visible_rows)
                            .child(header),
                    ),
            )
            .when_some(active_fk_card.zip(fk_card_position), |el, (card, pos)| {
                el.child(render_fk_card(&card, pos, state_for_fk_card, cx))
            })
    }
}

#[allow(clippy::too_many_arguments)]
fn render_header(
    columns: &[DataTableColumn],
    state: &DataTableState,
    state_entity: Entity<DataTableState>,
    columns_width: Pixels,
    header_height: Pixels,
    scroll_offset: Point<Pixels>,
    text: u32,
    text_muted: u32,
    accent: u32,
    panel_background: u32,
    border_variant: u32,
) -> Div {
    div()
        .absolute()
        .left(-scroll_offset.x)
        .top(px(0.))
        .w(columns_width)
        .flex()
        .flex_shrink_0()
        .h(header_height)
        .bg(rgb(panel_background))
        .border_b_1()
        .border_color(rgb(border_variant))
        .children(columns.iter().enumerate().map(|(col_idx, col)| {
            let is_pk = state
                .table_context
                .as_ref()
                .map(|c| c.primary_keys.iter().any(|pk| pk == col.name.as_ref()))
                .unwrap_or(false);

            let col_width = col.width;

            div()
                .relative()
                .w(col_width)
                .flex_shrink_0()
                .h(header_height)
                .child(
                    div()
                        .size_full()
                        .px_3()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .overflow_hidden()
                        .child(
                            div()
                                .flex()
                                .gap_1()
                                .items_center()
                                .overflow_hidden()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(text))
                                        .overflow_hidden()
                                        .whitespace_nowrap()
                                        .child(col.name.clone()),
                                )
                                .when(is_pk, |el| {
                                    el.child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(accent))
                                            .child("PK"),
                                    )
                                }),
                        )
                        .when_some(col.type_name.clone(), |el, type_name| {
                            el.child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(text_muted))
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .child(type_name),
                            )
                        }),
                )
                .child(render_resize_handle(col_idx, state_entity.clone()))
        }))
}

#[allow(clippy::too_many_arguments)]
fn render_visible_rows(
    first_visible_row: usize,
    last_visible_row: usize,
    rows: &[Vec<SharedString>],
    col_widths: &[Pixels],
    columns_width: Pixels,
    row_height: Pixels,
    header_height: Pixels,
    scroll_offset: Point<Pixels>,
    background: u32,
    panel_background: u32,
    border_variant: u32,
    text: u32,
    text_muted: u32,
    accent: u32,
    row_hover_bg: u32,
    cell_hover_bg: u32,
    column_names: Vec<SharedString>,
    column_types: Vec<Option<SharedString>>,
    foreign_keys: Arc<std::collections::HashMap<String, crate::postcommander::types::ForeignKeyInfo>>,
    state: Entity<DataTableState>,
) -> Vec<Stateful<Div>> {
    (first_visible_row..last_visible_row)
        .map(|row_ix| {
            let row = &rows[row_ix];
            let bg = if row_ix % 2 == 0 { background } else { panel_background };
            let row_y = header_height + row_height * row_ix as f32 - scroll_offset.y;
            let state_for_row = state.clone();
            let column_names_for_row = column_names.clone();
            let column_types_for_row = column_types.clone();
            let foreign_keys_for_row = foreign_keys.clone();
            let col_widths_for_row = col_widths.to_vec();

            div()
                .id(ElementId::Integer(row_ix as u64))
                .group("table-row")
                .absolute()
                .left(-scroll_offset.x)
                .top(row_y)
                .w(columns_width)
                .flex()
                .h(row_height)
                .bg(rgb(bg))
                .hover(|s| s.bg(rgb(row_hover_bg)))
                .border_b_1()
                .border_color(rgb(border_variant))
                .children(row.iter().enumerate().map(move |(col_ix, cell)| {
                    let column_type = column_types_for_row
                        .get(col_ix)
                        .and_then(|t| t.clone());
                    render_cell(
                        row_ix,
                        col_ix,
                        cell,
                        row,
                        &col_widths_for_row,
                        row_height,
                        text,
                        text_muted,
                        accent,
                        cell_hover_bg,
                        &column_names_for_row,
                        column_type,
                        &foreign_keys_for_row,
                        &state_for_row,
                    )
                }))
        })
        .collect()
}

fn is_boolean_type(type_name: &Option<SharedString>) -> bool {
    type_name
        .as_ref()
        .map(|t| {
            let t = t.to_lowercase();
            t == "bool" || t == "boolean"
        })
        .unwrap_or(false)
}

#[allow(clippy::too_many_arguments)]
fn render_cell(
    row_ix: usize,
    col_ix: usize,
    cell: &SharedString,
    row: &[SharedString],
    col_widths: &[Pixels],
    row_height: Pixels,
    text: u32,
    text_muted: u32,
    accent: u32,
    cell_hover_bg: u32,
    column_names: &[SharedString],
    column_type: Option<SharedString>,
    foreign_keys: &Arc<std::collections::HashMap<String, crate::postcommander::types::ForeignKeyInfo>>,
    state: &Entity<DataTableState>,
) -> impl IntoElement {
    let is_null = cell.as_ref() == "NULL";
    let width = col_widths.get(col_ix).copied().unwrap_or(px(150.));
    let cell_value = cell.clone();
    let column_name = column_names.get(col_ix).cloned().unwrap_or_else(|| "".into());
    let state_for_cell = state.clone();
    let state_for_context = state.clone();
    let row_data = row.to_vec();
    let column_names_vec: Vec<String> = column_names.iter().map(|s| s.to_string()).collect();

    let fk_info: Option<crate::postcommander::types::ForeignKeyInfo> =
        foreign_keys.get(column_name.as_ref()).cloned();
    let is_fk = fk_info.is_some();
    let is_bool = is_boolean_type(&column_type);

    let cell_content: AnyElement = if is_bool && !is_null {
        let is_checked = cell.to_lowercase() == "true" || cell.as_ref() == "t";
        let state_for_checkbox = state.clone();
        Checkbox::new(ElementId::Integer((row_ix as u64) << 32 | (col_ix as u64) | 0x8000_0000))
            .checked(is_checked)
            .on_click(move |new_checked: &bool, _window, cx| {
                let new_value = if *new_checked { "true" } else { "false" };
                state_for_checkbox.update(cx, |_, cx| {
                    cx.emit(CellSaveRequested {
                        row_index: row_ix,
                        col_index: col_ix,
                        new_value: new_value.to_string(),
                    });
                });
            })
            .into_any_element()
    } else {
        div()
            .flex()
            .items_center()
            .gap_1()
            .overflow_hidden()
            .child(
                div()
                    .text_sm()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .text_color(rgb(if is_null {
                        text_muted
                    } else if is_fk {
                        accent
                    } else {
                        text
                    }))
                    .when(is_null, |el| el.italic())
                    .child(if is_null {
                        SharedString::from("—")
                    } else {
                        cell.clone()
                    }),
            )
            .when(is_fk && !is_null, |el| {
                el.child(icon_sm("external-link", accent))
            })
            .into_any_element()
    };

    div()
        .id(ElementId::Integer((row_ix as u64) << 32 | (col_ix as u64)))
        .w(width)
        .flex_shrink_0()
        .h(row_height)
        .px_3()
        .flex()
        .items_center()
        .overflow_hidden()
        .cursor_pointer()
        .hover(|s| s.bg(rgb(cell_hover_bg)))
        .when(!is_bool, |el| {
            let state_for_click = state_for_cell.clone();
            let cell_value_click = cell_value.clone();
            let column_name_click = column_name.clone();
            let fk_info_click = fk_info.clone();
            el.on_click(move |event, _window, cx| {
                if event.click_count() == 2 {
                    state_for_click.update(cx, |_, cx| {
                        cx.emit(CellDoubleClicked {
                            row_index: row_ix,
                            col_index: col_ix,
                            column_name: column_name_click.clone(),
                            current_value: cell_value_click.clone(),
                        });
                    });
                } else if event.click_count() == 1 {
                    if let Some(ref fk) = fk_info_click {
                        if !is_null {
                            state_for_click.update(cx, |state, cx| {
                                state.show_fk_card(
                                    row_ix,
                                    col_ix,
                                    cell_value_click.clone(),
                                    fk.clone(),
                                    cx,
                                );
                            });
                        }
                    }
                }
            })
        })
        .on_mouse_down(MouseButton::Right, move |event, _window, cx| {
            let position = event.position;
            state_for_context.update(cx, |_, cx| {
                cx.emit(CellContextMenu {
                    row_index: row_ix,
                    col_index: col_ix,
                    column_names: column_names_vec.clone(),
                    row_data: row_data.clone(),
                    position,
                });
            });
        })
        .child(cell_content)
}
