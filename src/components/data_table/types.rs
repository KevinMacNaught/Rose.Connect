use gpui::*;
use std::sync::Arc;

use crate::postcommander::types::{ForeignKeyInfo, TableContext};

pub(crate) const ROW_HEIGHT: f32 = 32.;
pub(crate) const HEADER_HEIGHT: f32 = 48.;
pub(crate) const MIN_COL_WIDTH: f32 = 50.;
pub(crate) const RESIZE_HANDLE_WIDTH: f32 = 6.;
pub(crate) const END_PADDING: f32 = 16.;

#[derive(Clone)]
pub(crate) struct DraggedColumnResize {
    pub col_index: usize,
}

impl Render for DraggedColumnResize {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

pub(crate) struct ResizeDragState {
    pub col_index: usize,
    pub last_x: Option<Pixels>,
}

pub struct DataTableColumn {
    pub name: SharedString,
    pub type_name: Option<SharedString>,
    pub(crate) width: Pixels,
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

#[derive(Clone)]
pub struct FkDataRequest {
    pub fk_info: ForeignKeyInfo,
    pub cell_value: SharedString,
}

#[derive(Clone)]
pub struct CellContextMenu {
    pub row_index: usize,
    pub col_index: usize,
    pub column_names: Vec<String>,
    pub row_data: Vec<SharedString>,
    pub position: Point<Pixels>,
}

#[derive(Clone)]
pub struct FkHoverCardData {
    pub fk_info: ForeignKeyInfo,
    pub cell_value: SharedString,
    pub row_index: usize,
    pub col_index: usize,
    pub referenced_row: Option<Vec<(String, String)>>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub drag_offset: Point<Pixels>,
}

#[derive(Clone)]
pub(crate) struct DraggedFkCard;

impl Render for DraggedFkCard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

pub struct DataTableState {
    pub(crate) columns: Vec<DataTableColumn>,
    pub(crate) rows: Arc<Vec<Vec<SharedString>>>,
    pub(crate) table_context: Option<TableContext>,
    pub(crate) scroll_offset: Point<Pixels>,
    pub(crate) viewport_size: Size<Pixels>,
    pub(crate) container_origin: Point<Pixels>,
    pub(crate) resize_drag: Option<ResizeDragState>,
    pub active_fk_card: Option<FkHoverCardData>,
    pub(crate) fk_card_drag_start: Option<Point<Pixels>>,
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
            container_origin: Point::default(),
            resize_drag: None,
            active_fk_card: None,
            fk_card_drag_start: None,
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

    pub(crate) fn content_size(&self) -> Size<Pixels> {
        let row_height = px(32.);
        let header_height = px(48.);
        let total_width: Pixels = self.columns.iter().map(|c| c.width).sum();
        let total_height = header_height + (row_height * self.rows.len() as f32);
        Size {
            width: (total_width + px(END_PADDING)).max(px(100.)),
            height: total_height,
        }
    }

    pub(crate) fn on_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut Context<Self>) {
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

    pub fn set_container_origin(&mut self, origin: Point<Pixels>) {
        self.container_origin = origin;
    }

    pub(crate) fn calculate_cell_position(&self, row_index: usize, col_index: usize) -> Point<Pixels> {
        let col_x: Pixels = self.columns.iter().take(col_index).map(|c| c.width).sum();
        let cell_x = col_x - self.scroll_offset.x;
        let cell_y = px(HEADER_HEIGHT) + px(ROW_HEIGHT) * row_index as f32 - self.scroll_offset.y + px(ROW_HEIGHT);

        point(
            self.container_origin.x + cell_x,
            self.container_origin.y + cell_y,
        )
    }

    pub fn show_fk_card(
        &mut self,
        row_index: usize,
        col_index: usize,
        cell_value: SharedString,
        fk_info: ForeignKeyInfo,
        cx: &mut Context<Self>,
    ) {
        if cell_value.as_ref() == "NULL" {
            return;
        }

        self.active_fk_card = Some(FkHoverCardData {
            fk_info: fk_info.clone(),
            cell_value: cell_value.clone(),
            row_index,
            col_index,
            referenced_row: None,
            is_loading: true,
            error: None,
            drag_offset: Point::default(),
        });
        self.fk_card_drag_start = None;

        cx.emit(FkDataRequest {
            fk_info,
            cell_value,
        });
        cx.notify();
    }

    pub fn start_fk_card_drag(&mut self, position: Point<Pixels>) {
        self.fk_card_drag_start = Some(position);
    }

    pub fn update_fk_card_drag(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        if let (Some(start), Some(ref mut card)) = (self.fk_card_drag_start, &mut self.active_fk_card) {
            let delta = position - start;
            card.drag_offset = card.drag_offset + delta;
            self.fk_card_drag_start = Some(position);
            cx.notify();
        }
    }

    pub fn end_fk_card_drag(&mut self) {
        self.fk_card_drag_start = None;
    }

    pub fn hide_fk_card(&mut self, cx: &mut Context<Self>) {
        self.active_fk_card = None;
        cx.notify();
    }

    pub fn set_fk_card_data(&mut self, data: Vec<(String, String)>, cx: &mut Context<Self>) {
        if let Some(ref mut card) = self.active_fk_card {
            card.referenced_row = Some(data);
            card.is_loading = false;
            cx.notify();
        }
    }

    pub fn set_fk_card_error(&mut self, error: String, cx: &mut Context<Self>) {
        if let Some(ref mut card) = self.active_fk_card {
            card.is_loading = false;
            card.error = Some(error);
            cx.notify();
        }
    }
}

impl EventEmitter<CellSaveRequested> for DataTableState {}
impl EventEmitter<CellDoubleClicked> for DataTableState {}
impl EventEmitter<FkDataRequest> for DataTableState {}
impl EventEmitter<CellContextMenu> for DataTableState {}
