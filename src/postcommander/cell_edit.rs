use crate::components::{CellDoubleClicked, CellSaveRequested, DataTableState};
use crate::postcommander::page::PostCommanderPage;
use crate::postcommander::types::CellEditState;
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::input::InputState;

impl PostCommanderPage {
    pub(crate) fn handle_cell_save(
        &mut self,
        table_state: Entity<DataTableState>,
        event: &CellSaveRequested,
        cx: &mut Context<Self>,
    ) {
        let Some(tab) = self.tabs.iter().find(|t| t.table_state == table_state) else {
            return;
        };

        let Some(context) = &tab.table_context else {
            table_state.clone().update(cx, |state, cx| {
                state.set_edit_error(Some("No table context for editing".to_string()), cx);
            });
            return;
        };

        let Some(result) = &tab.result else {
            return;
        };

        let row_index = event.row_index;
        let col_index = event.col_index;
        let new_value = event.new_value.clone();

        let rows = table_state.read(cx).rows();
        let Some(row) = rows.get(row_index) else {
            return;
        };

        let mut pk_conditions = Vec::new();
        for pk_col in &context.primary_keys {
            if let Some((pk_idx, _)) = result.columns.iter().enumerate().find(|(_, c)| &c.name == pk_col) {
                if let Some(pk_value) = row.get(pk_idx) {
                    pk_conditions.push(format!("\"{}\" = '{}'", pk_col, pk_value.replace('\'', "''")));
                }
            }
        }

        if pk_conditions.is_empty() {
            table_state.clone().update(cx, |state, cx| {
                state.set_edit_error(Some("No primary key values found".to_string()), cx);
            });
            return;
        }

        let column_name = result.columns.get(col_index).map(|c| c.name.clone()).unwrap_or_default();
        let sql = format!(
            "UPDATE \"{}\".\"{}\" SET \"{}\" = '{}' WHERE {}",
            context.schema,
            context.table,
            column_name,
            new_value.replace('\'', "''"),
            pk_conditions.join(" AND ")
        );

        table_state.clone().update(cx, |state, cx| {
            state.set_edit_saving(true, cx);
        });

        let rx = self.db_manager.execute(sql);

        cx.spawn(async move |_this, cx| {
            let result = rx.await;
            let _ = table_state.update(cx, |state, cx| {
                match result {
                    Ok(Ok(_)) => {
                        state.update_cell_value(row_index, col_index, SharedString::from(new_value));
                        state.finish_editing(cx);
                    }
                    Ok(Err(e)) => {
                        state.set_edit_error(Some(e.to_string()), cx);
                    }
                    Err(_) => {
                        state.set_edit_error(Some("Update failed".to_string()), cx);
                    }
                }
            });
        })
        .detach();
    }

    pub(crate) fn handle_cell_double_click(
        &mut self,
        _table_state: Entity<DataTableState>,
        event: &CellDoubleClicked,
        cx: &mut Context<Self>,
    ) {
        self.cell_edit = Some(CellEditState {
            row_index: event.row_index,
            col_index: event.col_index,
            column_name: event.column_name.clone(),
            original_value: event.current_value.clone(),
            editor: None,
            is_saving: false,
            error: None,
        });
        cx.notify();
    }

    pub(crate) fn save_cell_edit(&mut self, cx: &mut Context<Self>) {
        let Some(edit) = self.cell_edit.as_ref() else {
            return;
        };

        let Some(tab) = self.active_tab_id.as_ref().and_then(|id| self.tabs.iter().find(|t| &t.id == id)) else {
            return;
        };

        let Some(context) = &tab.table_context else {
            if let Some(ref mut edit) = self.cell_edit {
                edit.error = Some("No table context for editing".to_string());
            }
            cx.notify();
            return;
        };

        let Some(result) = &tab.result else {
            return;
        };

        let Some(editor) = &edit.editor else {
            return;
        };
        let new_value = editor.read(cx).value().to_string();
        let row_index = edit.row_index;
        let col_index = edit.col_index;

        let table_state = tab.table_state.clone();
        let rows = table_state.read(cx).rows();
        let Some(row) = rows.get(row_index) else {
            return;
        };

        let mut pk_conditions = Vec::new();
        for pk_col in &context.primary_keys {
            if let Some((pk_idx, _)) = result.columns.iter().enumerate().find(|(_, c)| &c.name == pk_col) {
                if let Some(pk_value) = row.get(pk_idx) {
                    pk_conditions.push(format!("\"{}\" = '{}'", pk_col, pk_value.replace('\'', "''")));
                }
            }
        }

        if pk_conditions.is_empty() {
            if let Some(ref mut edit) = self.cell_edit {
                edit.error = Some("No primary key values found".to_string());
            }
            cx.notify();
            return;
        }

        let column_name = result.columns.get(col_index).map(|c| c.name.clone()).unwrap_or_default();
        let sql = format!(
            "UPDATE \"{}\".\"{}\" SET \"{}\" = '{}' WHERE {}",
            context.schema,
            context.table,
            column_name,
            new_value.replace('\'', "''"),
            pk_conditions.join(" AND ")
        );

        if let Some(ref mut edit) = self.cell_edit {
            edit.is_saving = true;
        }
        cx.notify();

        let rx = self.db_manager.execute(sql);

        cx.spawn(async move |this, cx| {
            let result = rx.await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(Ok(_)) => {
                        table_state.update(cx, |state, _cx| {
                            state.update_cell_value(row_index, col_index, SharedString::from(new_value));
                        });
                        this.cell_edit = None;
                    }
                    Ok(Err(e)) => {
                        if let Some(ref mut edit) = this.cell_edit {
                            edit.error = Some(e.to_string());
                            edit.is_saving = false;
                        }
                    }
                    Err(_) => {
                        if let Some(ref mut edit) = this.cell_edit {
                            edit.error = Some("Update failed".to_string());
                            edit.is_saving = false;
                        }
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub(crate) fn cancel_cell_edit(&mut self, cx: &mut Context<Self>) {
        self.cell_edit = None;
        cx.notify();
    }

    pub(crate) fn render_cell_edit_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(ref mut edit) = self.cell_edit {
            if edit.editor.is_none() {
                let original_value: String = edit.original_value.to_string();
                let editor = cx.new(move |cx| {
                    let mut state = InputState::new(window, cx)
                        .placeholder("Enter value...")
                        .soft_wrap(true);
                    state.set_value(&original_value, window, cx);
                    state
                });
                edit.editor = Some(editor);
            }
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let surface = colors.surface;
        let text = colors.text;
        let text_muted = colors.text_muted;
        let border = colors.border;
        let _element = colors.element;
        let element_hover = colors.element_hover;
        let accent = colors.accent;
        let accent_foreground = colors.accent_foreground;
        let status_error = colors.status_error;

        let edit = self.cell_edit.as_ref();
        let column_name = edit.map(|e| e.column_name.clone()).unwrap_or_default();
        let editor = edit.and_then(|e| e.editor.clone());
        let is_saving = edit.map(|e| e.is_saving).unwrap_or(false);
        let error = edit.and_then(|e| e.error.clone());

        let gold_border = 0xD4A574;

        div()
            .id("cell-edit-backdrop")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0., 0., 0., 0.5))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                this.cancel_cell_edit(cx);
            }))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                if event.keystroke.key == "escape" {
                    this.cancel_cell_edit(cx);
                } else if event.keystroke.key == "enter"
                    && event.keystroke.modifiers.platform
                {
                    this.save_cell_edit(cx);
                }
            }))
            .child(
                div()
                    .id("cell-edit-modal")
                    .occlude()
                    .w(px(400.))
                    .bg(rgb(surface))
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(border))
                    .shadow_xl()
                    .child(
                        div()
                            .px_3()
                            .py_2()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(text_muted))
                                    .child(column_name),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .text_xs()
                                    .text_color(rgb(text_muted))
                                    .child(
                                        div()
                                            .flex()
                                            .gap_1()
                                            .items_center()
                                            .child("⌘+↵")
                                            .child("save"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_1()
                                            .items_center()
                                            .child("esc")
                                            .child("cancel"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .p_3()
                            .child(
                                div()
                                    .w_full()
                                    .h(px(120.))
                                    .rounded_md()
                                    .border_2()
                                    .border_color(rgb(gold_border))
                                    .bg(rgb(surface))
                                    .overflow_hidden()
                                    .when_some(editor, |el, editor| {
                                        el.child(
                                            gpui_component::input::Input::new(&editor)
                                                .appearance(false)
                                                .p(px(8.))
                                                .cleanable(false)
                                        )
                                    }),
                            )
                            .when_some(error, |el, err| {
                                el.child(
                                    div()
                                        .mt_2()
                                        .text_xs()
                                        .text_color(rgb(status_error))
                                        .child(err),
                                )
                            }),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_2()
                            .flex()
                            .justify_center()
                            .gap_3()
                            .child(
                                div()
                                    .id("cancel-edit-btn")
                                    .px_4()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(move |s| s.bg(rgb(element_hover)))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.cancel_cell_edit(cx);
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(text))
                                            .child("Cancel"),
                                    ),
                            )
                            .child(
                                div()
                                    .id("save-edit-btn")
                                    .px_4()
                                    .py_1()
                                    .rounded_md()
                                    .bg(rgb(accent))
                                    .cursor_pointer()
                                    .when(!is_saving, |el| el.hover(|s| s.opacity(0.9)))
                                    .when(is_saving, |el| el.opacity(0.6))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.save_cell_edit(cx);
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(accent_foreground))
                                            .child(if is_saving { "Saving..." } else { "Save" }),
                                    ),
                            ),
                    ),
            )
    }
}
