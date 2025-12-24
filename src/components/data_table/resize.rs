use gpui::*;

use super::types::{DataTableState, DraggedColumnResize, MIN_COL_WIDTH};

impl DataTableState {
    pub fn start_resize_drag(&mut self, col_index: usize) {
        self.resize_drag = Some(super::types::ResizeDragState {
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
}

pub(crate) fn render_resize_handle(
    col_idx: usize,
    state_entity: Entity<DataTableState>,
) -> impl IntoElement {
    use super::types::RESIZE_HANDLE_WIDTH;

    let state_for_drag_start = state_entity.clone();
    let state_for_drag_move = state_entity.clone();
    let state_for_drag_end = state_entity;

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
}
