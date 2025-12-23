use crate::icons::icon_sm;
use crate::postcommander::page::PostCommanderPage;
use crate::postcommander::types::{TableColumn, TableStructureInfo};
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::tooltip::Tooltip;

impl PostCommanderPage {
    pub fn render_structure_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let panel_background = colors.panel_background;
        let border_variant = colors.border_variant;
        let text = colors.text;
        let text_muted = colors.text_muted;

        let active_tab = self
            .active_tab_id
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id));

        let structures = active_tab
            .map(|t| t.table_structures.clone())
            .unwrap_or_default();

        let structure_loading = active_tab.map(|t| t.structure_loading).unwrap_or(false);
        let structure_expanded = active_tab
            .map(|t| t.structure_expanded.clone())
            .unwrap_or_default();

        div()
            .w(px(self.structure_panel_width))
            .h_full()
            .flex()
            .flex_col()
            .bg(rgb(panel_background))
            .border_l_1()
            .border_color(rgb(border_variant))
            .child(self.render_structure_header(text, border_variant))
            .child(
                div()
                    .id("structure-content")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .px_2()
                    .py_1()
                    .when(structure_loading, |el| {
                        el.child(
                            div()
                                .py_4()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(text_muted))
                                        .child("Loading..."),
                                ),
                        )
                    })
                    .when(!structure_loading && structures.is_empty(), |el| {
                        el.child(
                            div()
                                .py_4()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(text_muted))
                                        .child("Run a query to see structure"),
                                ),
                        )
                    })
                    .when(!structure_loading && !structures.is_empty(), |el| {
                        el.children(structures.iter().map(|structure| {
                            let key = format!("{}.{}", structure.schema, structure.table);
                            let is_expanded = structure_expanded.get(&key).copied().unwrap_or(true);
                            self.render_table_node(structure, is_expanded, cx)
                        }))
                    }),
            )
    }

    fn render_structure_header(&self, text: u32, border_variant: u32) -> impl IntoElement {
        div()
            .h(px(36.))
            .px_3()
            .flex()
            .items_center()
            .border_b_1()
            .border_color(rgb(border_variant))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(text))
                    .child("Structure"),
            )
    }

    fn render_table_node(
        &self,
        structure: &TableStructureInfo,
        is_expanded: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let text = colors.text;
        let text_muted = colors.text_muted;
        let element_hover = colors.element_hover;

        let key = format!("{}.{}", structure.schema, structure.table);
        let table_name = structure.table.clone();
        let columns = structure.columns.clone();
        let key_for_click = key.clone();

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .id(SharedString::from(format!("structure-table-{}", key)))
                    .h(px(28.))
                    .px_1()
                    .flex()
                    .items_center()
                    .gap_1()
                    .rounded_md()
                    .cursor_pointer()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(tab_id) = this.active_tab_id.clone() {
                            if let Some(tab) = this.tabs.iter_mut().find(|t| t.id == tab_id) {
                                let current = tab
                                    .structure_expanded
                                    .get(&key_for_click)
                                    .copied()
                                    .unwrap_or(true);
                                tab.structure_expanded.insert(key_for_click.clone(), !current);
                            }
                        }
                        cx.notify();
                    }))
                    .child(icon_sm(
                        if is_expanded {
                            "chevron-down"
                        } else {
                            "chevron-right"
                        },
                        text_muted,
                    ))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text))
                            .child(table_name),
                    ),
            )
            .when(is_expanded, |el| {
                el.child(
                    div()
                        .pl(px(16.))
                        .flex()
                        .flex_col()
                        .children(columns.iter().map(|col| self.render_column_row(col, cx))),
                )
            })
    }

    fn render_column_row(&self, col: &TableColumn, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let text = colors.text;
        let text_muted = colors.text_muted;
        let status_warning = colors.status_warning;
        let accent = colors.accent;

        let column_name = col.name.clone();
        let data_type = col.data_type.clone();
        let is_pk = col.is_primary_key;
        let is_fk = col.is_foreign_key;
        let fk_ref = col.references.clone();

        let tooltip_text = if let Some(ref fk) = fk_ref {
            format!("{}.{}", fk.table, fk.column)
        } else {
            String::new()
        };

        div()
            .id(SharedString::from(format!("col-{}", column_name)))
            .h(px(24.))
            .px_1()
            .flex()
            .items_center()
            .gap_1()
            .when(is_pk, |el| {
                el.child(
                    div()
                        .size(px(16.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(icon_sm("key", status_warning)),
                )
            })
            .when(is_fk && !is_pk, |el| {
                el.child(
                    div()
                        .id(SharedString::from(format!("fk-{}", column_name)))
                        .size(px(16.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(icon_sm("link", accent))
                        .when(!tooltip_text.is_empty(), |el| {
                            el.tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))
                        }),
                )
            })
            .when(!is_pk && !is_fk, |el| el.child(div().size(px(16.))))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .items_center()
                    .gap_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(text))
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .child(column_name),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(text_muted))
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .child(data_type),
                    ),
            )
    }

    pub fn render_structure_resize_handle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let border_variant = colors.border_variant;
        let accent = colors.accent;
        let is_resizing = self.is_resizing_structure;

        div()
            .id("structure-resize-handle")
            .w(px(4.))
            .h_full()
            .cursor_col_resize()
            .bg(transparent_black())
            .when(is_resizing, |el| el.bg(rgb(accent)))
            .hover(move |s| s.bg(rgb(border_variant)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.is_resizing_structure = true;
                    this.resize_structure_start_x = f32::from(event.position.x);
                    this.resize_structure_start_width = this.structure_panel_width;
                    cx.notify();
                }),
            )
    }

    pub fn render_structure_resize_overlay(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("structure-resize-overlay")
            .absolute()
            .inset_0()
            .cursor_col_resize()
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if this.is_resizing_structure {
                    let delta = this.resize_structure_start_x - f32::from(event.position.x);
                    let new_width = (this.resize_structure_start_width + delta)
                        .max(200.0)
                        .min(500.0);
                    this.structure_panel_width = new_width;
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.is_resizing_structure = false;
                    this.save_structure_panel_width(cx);
                    cx.notify();
                }),
            )
    }
}
