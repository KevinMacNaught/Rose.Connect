use gpui::prelude::FluentBuilder;
use gpui::*;
use std::sync::Arc;

use crate::postcommander::types::TableContext;
use crate::theme::ActiveTheme;

const ROW_HEIGHT: f32 = 32.;
const HEADER_HEIGHT: f32 = 48.;
const MIN_COL_WIDTH: f32 = 50.;
const RESIZE_HANDLE_WIDTH: f32 = 6.;
const END_PADDING: f32 = 16.;

#[derive(Clone)]
struct DraggedColumnResize {
    col_index: usize,
}

impl Render for DraggedColumnResize {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

struct ResizeDragState {
    col_index: usize,
    last_x: Option<Pixels>,
}

pub struct DataTableColumn {
    pub name: SharedString,
    pub type_name: Option<SharedString>,
    pub width: Pixels,
}

impl DataTableColumn {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            type_name: None,
            width: px(150.),
        }
    }

    pub fn type_name(mut self, type_name: impl Into<SharedString>) -> Self {
        self.type_name = Some(type_name.into());
        self
    }

    #[allow(dead_code)]
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }
}

#[derive(Clone)]
pub struct CellSaveRequested {
    pub row_index: usize,
    pub col_index: usize,
    pub new_value: String,
}

#[derive(Clone)]
pub struct CellDoubleClicked {
    pub row_index: usize,
    pub col_index: usize,
    pub column_name: SharedString,
    pub current_value: SharedString,
}

pub struct DataTableState {
    columns: Vec<DataTableColumn>,
    rows: Arc<Vec<Vec<SharedString>>>,
    table_context: Option<TableContext>,
    scroll_offset: Point<Pixels>,
    viewport_size: Size<Pixels>,
    resize_drag: Option<ResizeDragState>,
}

impl DataTableState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            columns: vec![],
            rows: Arc::new(vec![]),
            table_context: None,
            scroll_offset: Point::default(),
            viewport_size: Size {
                width: px(100.),
                height: px(100.),
            },
            resize_drag: None,
        }
    }

    pub fn set_columns(&mut self, columns: Vec<DataTableColumn>) {
        self.columns = columns;
    }

    pub fn set_rows(&mut self, rows: Arc<Vec<Vec<SharedString>>>) {
        self.rows = rows;
    }

    pub fn rows(&self) -> &Arc<Vec<Vec<SharedString>>> {
        &self.rows
    }

    pub fn set_table_context(&mut self, context: Option<TableContext>) {
        self.table_context = context;
    }

    pub fn clear(&mut self) {
        self.columns.clear();
        self.rows = Arc::new(vec![]);
        self.table_context = None;
        self.scroll_offset = Point::default();
    }

    pub fn set_edit_saving(&mut self, _saving: bool, _cx: &mut Context<Self>) {}

    pub fn set_edit_error(&mut self, _error: Option<String>, _cx: &mut Context<Self>) {}

    pub fn update_cell_value(
        &mut self,
        row_index: usize,
        col_index: usize,
        new_value: SharedString,
    ) {
        let rows = Arc::make_mut(&mut self.rows);
        if let Some(row) = rows.get_mut(row_index) {
            if let Some(cell) = row.get_mut(col_index) {
                *cell = new_value;
            }
        }
    }

    pub fn finish_editing(&mut self, _cx: &mut Context<Self>) {}

    pub fn start_resize_drag(&mut self, col_index: usize) {
        self.resize_drag = Some(ResizeDragState {
            col_index,
            last_x: None,
        });
    }

    pub fn update_resize_drag(&mut self, current_x: Pixels, cx: &mut Context<Self>) {
        if let Some(ref mut drag) = self.resize_drag {
            if let Some(last_x) = drag.last_x {
                let delta = current_x - last_x;
                let col_index = drag.col_index;
                if let Some(col) = self.columns.get_mut(col_index) {
                    col.width = (col.width + delta).max(px(MIN_COL_WIDTH));
                    cx.notify();
                }
            }
            drag.last_x = Some(current_x);
        }
    }

    pub fn end_resize_drag(&mut self) {
        self.resize_drag = None;
    }

    fn content_size(&self) -> Size<Pixels> {
        let row_height = px(32.);
        let header_height = px(48.);
        let total_width: Pixels = self.columns.iter().map(|c| c.width).sum();
        let total_height = header_height + (row_height * self.rows.len() as f32);
        Size {
            width: (total_width + px(END_PADDING)).max(px(100.)),
            height: total_height,
        }
    }

    fn on_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut Context<Self>) {
        let delta = event.delta.pixel_delta(px(20.));

        self.scroll_offset.x -= delta.x;
        self.scroll_offset.y -= delta.y;

        let content_size = self.content_size();
        let max_scroll_x = (content_size.width - self.viewport_size.width).max(px(0.));
        let max_scroll_y = (content_size.height - self.viewport_size.height).max(px(0.));

        self.scroll_offset.x = self.scroll_offset.x.clamp(px(0.), max_scroll_x);
        self.scroll_offset.y = self.scroll_offset.y.clamp(px(0.), max_scroll_y);

        cx.notify();
    }

    pub fn set_viewport_size(&mut self, size: Size<Pixels>, cx: &mut Context<Self>) {
        if self.viewport_size != size {
            self.viewport_size = size;
            // Re-clamp scroll offset with new viewport size
            let content_size = self.content_size();
            let max_scroll_x = (content_size.width - size.width).max(px(0.));
            let max_scroll_y = (content_size.height - size.height).max(px(0.));
            self.scroll_offset.x = self.scroll_offset.x.clamp(px(0.), max_scroll_x);
            self.scroll_offset.y = self.scroll_offset.y.clamp(px(0.), max_scroll_y);
            cx.notify();
        }
    }
}

impl EventEmitter<CellSaveRequested> for DataTableState {}
impl EventEmitter<CellDoubleClicked> for DataTableState {}

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

        // Virtualization: calculate visible row range
        let scroll_y_after_header = (scroll_offset.y - header_height).max(px(0.));
        let first_visible_row = (f32::from(scroll_y_after_header) / ROW_HEIGHT).floor() as usize;
        let first_visible_row = first_visible_row.saturating_sub(2); // Buffer above
        let visible_count = (f32::from(viewport_size.height) / ROW_HEIGHT).ceil() as usize + 4; // Buffer below
        let last_visible_row = (first_visible_row + visible_count).min(row_count);

        // Header - fixed at top, only scrolls horizontally
        let state_for_resize = self.state.clone();
        let header = div()
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
                let state_entity = state_for_resize.clone();

                div()
                    .relative()
                    .w(col_width)
                    .flex_shrink_0()
                    .h(header_height)
                    .child(
                        // Column header content
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
                    .child({
                        let state_for_drag_start = state_entity.clone();
                        let state_for_drag_move = state_entity.clone();
                        let state_for_drag_end = state_entity.clone();
                        // Resize handle on right edge
                        div()
                            .id(ElementId::NamedInteger("col-resize".into(), col_idx as u64))
                            .absolute()
                            .right(px(0.))
                            .top(px(0.))
                            .w(px(RESIZE_HANDLE_WIDTH))
                            .h_full()
                            .cursor_col_resize()
                            .hover(|s| s.bg(hsla(0.6, 0.8, 0.5, 0.5)))
                            .on_drag(
                                DraggedColumnResize { col_index: col_idx },
                                move |drag, _point, _window, cx| {
                                    state_for_drag_start.update(cx, |state, _cx| {
                                        state.start_resize_drag(drag.col_index);
                                    });
                                    cx.new(|_| drag.clone())
                                },
                            )
                            .on_drag_move::<DraggedColumnResize>(move |event, _window, cx| {
                                state_for_drag_move.update(cx, |state, cx| {
                                    state.update_resize_drag(event.event.position.x, cx);
                                });
                            })
                            .on_mouse_up(
                                MouseButton::Left,
                                move |_event, _window, cx| {
                                    state_for_drag_end.update(cx, |state, _cx| {
                                        state.end_resize_drag();
                                    });
                                },
                            )
                    })
            }));

        let row_hover_bg = colors.element_hover;
        let cell_hover_bg = colors.element_selected;

        let column_names: Vec<SharedString> = columns.iter().map(|c| c.name.clone()).collect();

        // Only render visible rows (virtualization)
        let visible_rows: Vec<_> = (first_visible_row..last_visible_row)
            .map(|row_ix| {
                let row = &rows[row_ix];
                let bg = if row_ix % 2 == 0 { background } else { panel_background };
                let row_y = header_height + row_height * row_ix as f32 - scroll_offset.y;
                let state_for_row = self.state.clone();
                let column_names_for_row = column_names.clone();

                let col_widths_for_row = col_widths.clone();

                div()
                    .id(ElementId::NamedInteger("row".into(), row_ix as u64))
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
                        let is_null = cell.as_ref() == "NULL";
                        let width = col_widths_for_row.get(col_ix).copied().unwrap_or(px(150.));
                        let cell_value = cell.clone();
                        let column_name = column_names_for_row
                            .get(col_ix)
                            .cloned()
                            .unwrap_or_else(|| "".into());
                        let state_for_cell = state_for_row.clone();

                        div()
                            .id(ElementId::NamedInteger(
                                format!("cell-{}", row_ix).into(),
                                col_ix as u64,
                            ))
                            .w(width)
                            .flex_shrink_0()
                            .h(row_height)
                            .px_3()
                            .flex()
                            .items_center()
                            .overflow_hidden()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(cell_hover_bg)))
                            .on_click(move |event, _window, cx| {
                                if event.click_count() == 2 {
                                    state_for_cell.update(cx, |_, cx| {
                                        cx.emit(CellDoubleClicked {
                                            row_index: row_ix,
                                            col_index: col_ix,
                                            column_name: column_name.clone(),
                                            current_value: cell_value.clone(),
                                        });
                                    });
                                }
                            })
                            .child(
                                div()
                                    .w_full()
                                    .text_sm()
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .text_ellipsis()
                                    .text_color(rgb(if is_null { text_muted } else { text }))
                                    .when(is_null, |el| el.italic())
                                    .child(if is_null { SharedString::from("—") } else { cell.clone() }),
                            )
                    }))
            })
            .collect();

        let state_for_scroll = self.state.clone();
        let state_for_measure = self.state.clone();

        // Use a stacked layout: canvas measures bounds, then content is overlaid
        div()
            .id("data-table-container")
            .size_full()
            .relative()
            // Canvas as first child to measure the container
            .child(
                canvas(
                    move |bounds, _window, cx| {
                        state_for_measure.update(cx, |state, cx| {
                            state.set_viewport_size(bounds.size, cx);
                        });
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .inset_0(),
            )
            // Content container
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
    }
}
