use crate::components::DataTable;
use crate::icons::icon_sm;
use crate::postcommander::page::PostCommanderPage;
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::input::Input;

impl PostCommanderPage {
    pub fn render_query_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let panel_background = colors.panel_background;
        let border_variant = colors.border_variant;
        let editor_background = colors.elevated_surface;
        let text_muted = colors.text_muted;
        let element = colors.element;
        let element_hover = colors.element_hover;
        let accent = colors.accent;
        let accent_foreground = colors.accent_foreground;
        let editor_height = self.editor_height;

        let active_tab = self
            .active_tab_id
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id));

        let editor = active_tab.map(|t| t.editor.clone());
        let is_loading = active_tab.map(|t| t.is_loading).unwrap_or(false);

        div()
            .flex()
            .flex_col()
            .h(px(editor_height))
            .child(
                div()
                    .h(px(36.))
                    .px_2()
                    .flex()
                    .items_center()
                    .gap_2()
                    .bg(rgb(panel_background))
                    .border_b_1()
                    .border_color(rgb(border_variant))
                    .child(
                        div()
                            .id("execute-btn")
                            .h(px(28.))
                            .px_3()
                            .flex()
                            .items_center()
                            .gap_2()
                            .rounded_md()
                            .bg(rgb(accent))
                            .cursor_pointer()
                            .when(!is_loading, |el| el.hover(|s| s.opacity(0.9)))
                            .when(is_loading, |el| el.opacity(0.6))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.execute_query(cx);
                            }))
                            .child(icon_sm(
                                if is_loading { "loader-2" } else { "play" },
                                accent_foreground,
                            ))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(accent_foreground))
                                    .child(if is_loading { "Running..." } else { "Execute" }),
                            ),
                    )
                    .child(
                        div()
                            .id("ai-btn")
                            .h(px(28.))
                            .px_3()
                            .flex()
                            .items_center()
                            .gap_2()
                            .rounded_md()
                            .bg(rgb(element))
                            .cursor_pointer()
                            .hover(move |s| s.bg(rgb(element_hover)))
                            .child(icon_sm("sparkles", text_muted))
                            .child(div().text_sm().text_color(rgb(text_muted)).child("AI")),
                    )
                    .child(div().flex_1()),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .bg(rgb(editor_background))
                    .when_some(editor, |el, editor| {
                        el.child(
                            Input::new(&editor)
                                .appearance(false)
                                .p(px(8.))
                                .h(px(editor_height - 36.))
                                .font_family("monospace")
                        )
                    }),
            )
    }

    pub fn render_results_area(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let background = colors.background;
        let panel_background = colors.panel_background;
        let border_variant = colors.border_variant;
        let text_muted = colors.text_muted;
        let element_hover = colors.element_hover;
        let status_success = colors.status_success;
        let status_error = colors.status_error;
        let status_error_background = colors.status_error_background;
        let status_error_border = colors.status_error_border;

        let active_tab_id = self.active_tab_id.clone();
        let active_tab = active_tab_id
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id));

        let result = active_tab.and_then(|t| t.result.clone());
        let table_state = active_tab.map(|t| t.table_state.clone());
        let error = active_tab.and_then(|t| t.error.clone());
        let is_loading = active_tab.map(|t| t.is_loading).unwrap_or(false);

        let (execution_time, row_count) = result
            .as_ref()
            .map(|r| (r.execution_time_ms, r.rows.len()))
            .unwrap_or((0, 0));

        div()
            .flex_1()
            .min_h_0()
            .flex()
            .flex_col()
            .bg(rgb(background))
            .child(self.render_results_header(
                cx,
                result.is_some(),
                is_loading,
                error.is_some(),
                execution_time,
                row_count,
                panel_background,
                border_variant,
                text_muted,
                status_success,
                status_error,
                element_hover,
            ))
            .when_some(error.clone(), |el, err| {
                el.child(
                    div()
                        .flex_1()
                        .p_3()
                        .child(self.render_error_display(
                            err,
                            status_error,
                            status_error_background,
                            status_error_border,
                        )),
                )
            })
            .when(result.is_some() && error.is_none(), |el| {
                el.when_some(table_state, |el, state| {
                    el.child(
                        div()
                            .flex_1()
                            .min_h_0()
                            .min_w_0()
                            .child(DataTable::new(state))
                    )
                })
            })
            .when(result.is_none() && error.is_none() && !is_loading, |el| {
                el.child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(text_muted))
                                .child("Run a query to see results"),
                        ),
                )
            })
    }

    fn render_results_header(
        &self,
        cx: &mut Context<Self>,
        has_result: bool,
        is_loading: bool,
        has_error: bool,
        execution_time: u64,
        row_count: usize,
        panel_background: u32,
        border_variant: u32,
        text_muted: u32,
        status_success: u32,
        status_error: u32,
        element_hover: u32,
    ) -> impl IntoElement {
        div()
            .h(px(32.))
            .px_3()
            .flex()
            .items_center()
            .gap_3()
            .bg(rgb(panel_background))
            .border_t_1()
            .border_b_1()
            .border_color(rgb(border_variant))
            .when(has_result, |el| {
                el.child(
                    div()
                        .text_xs()
                        .text_color(rgb(text_muted))
                        .child(format!("{}ms", execution_time)),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(status_success))
                        .child(format!("{} rows", row_count)),
                )
            })
            .when(is_loading, |el| {
                el.child(
                    div()
                        .text_xs()
                        .text_color(rgb(text_muted))
                        .child("Executing query..."),
                )
            })
            .when(has_error, |el| {
                el.child(
                    div()
                        .text_xs()
                        .text_color(rgb(status_error))
                        .child("Query failed"),
                )
            })
            .child(div().flex_1())
            .when(has_result, |el| {
                el.child(
                    div()
                        .id("export-btn")
                        .h(px(24.))
                        .px_2()
                        .flex()
                        .items_center()
                        .gap_1()
                        .rounded_md()
                        .cursor_pointer()
                        .hover(move |s| s.bg(rgb(element_hover)))
                        .on_click(cx.listener(|this, event: &ClickEvent, window, cx| {
                            this.deploy_export_menu(event.position(), window, cx);
                        }))
                        .child(icon_sm("download", text_muted))
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(text_muted))
                                .child("Export"),
                        )
                        .child(icon_sm("chevron-down", text_muted)),
                )
            })
    }

    fn render_error_display(
        &self,
        error: String,
        status_error: u32,
        status_error_background: u32,
        status_error_border: u32,
    ) -> impl IntoElement {
        div()
            .p_4()
            .rounded_md()
            .bg(rgb(status_error_background))
            .border_1()
            .border_color(rgb(status_error_border))
            .child(
                div()
                    .flex()
                    .items_start()
                    .gap_2()
                    .child(icon_sm("alert-circle", status_error))
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(rgb(status_error))
                            .child(error),
                    ),
            )
    }

    pub fn render_tabs_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let panel_background = colors.panel_background;
        let border_variant = colors.border_variant;
        let text = colors.text;
        let text_muted = colors.text_muted;
        let element_hover = colors.element_hover;
        let element_active = colors.element_active;
        let accent = colors.accent;

        let active_tab_id = self.active_tab_id.clone();

        div()
            .h(px(36.))
            .flex()
            .items_center()
            .bg(rgb(panel_background))
            .border_b_1()
            .border_color(rgb(border_variant))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .overflow_hidden()
                    .children(self.tabs.iter().map(|tab| {
                        let is_active = active_tab_id.as_ref() == Some(&tab.id);
                        let tab_id = tab.id.clone();
                        let close_id = tab.id.clone();
                        let tab_name = tab.name.clone();
                        let tab_db = tab.database.clone();

                        div()
                            .id(SharedString::from(format!("tab-{}", tab.id)))
                            .h_full()
                            .px_3()
                            .flex()
                            .items_center()
                            .gap_2()
                            .cursor_pointer()
                            .border_b_2()
                            .when(is_active, |el| el.border_color(rgb(accent)))
                            .when(!is_active, |el| el.border_color(transparent_black()))
                            .hover(move |s| s.bg(rgb(element_hover)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.active_tab_id = Some(tab_id.clone());
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(if is_active { text } else { text_muted }))
                                    .child(tab_name),
                            )
                            .child(div().text_xs().text_color(rgb(text_muted)).child(tab_db))
                            .child(
                                div()
                                    .id(SharedString::from(format!("close-{}", close_id)))
                                    .size(px(16.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(2.))
                                    .hover(move |s| s.bg(rgb(element_active)))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.close_tab(&close_id, cx);
                                    }))
                                    .child(icon_sm("x", text_muted)),
                            )
                    })),
            )
            .child(
                div()
                    .id("add-tab")
                    .size(px(28.))
                    .mx_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .cursor_pointer()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.add_tab(window, cx);
                    }))
                    .child(icon_sm("plus", text_muted)),
            )
    }

    pub fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let background = colors.background;
        let surface = colors.surface;
        let text = colors.text;
        let text_muted = colors.text_muted;
        let accent = colors.accent;
        let accent_foreground = colors.accent_foreground;

        let is_connected = matches!(self.connection_state, super::types::ConnectionState::Connected);

        div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(background))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .size(px(64.))
                            .rounded_full()
                            .bg(rgb(surface))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(icon_sm("terminal", text_muted)),
                    )
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text))
                            .child(if is_connected {
                                "Ready to query"
                            } else {
                                "Connect to a database"
                            }),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(text_muted))
                            .child(if is_connected {
                                "Open a new query tab to start exploring your data"
                            } else {
                                "Click 'Connect to Server' in the sidebar to get started"
                            }),
                    )
                    .when(is_connected, |el| {
                        el.child(
                            div()
                                .id("new-query-btn")
                                .mt_2()
                                .px_4()
                                .py_2()
                                .rounded_md()
                                .bg(rgb(accent))
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.add_tab(window, cx);
                                }))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(icon_sm("plus", accent_foreground))
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(rgb(accent_foreground))
                                                .child("New Query"),
                                        ),
                                ),
                        )
                    })
                    .when(!is_connected, |el| {
                        el.child(
                            div()
                                .id("connect-btn-empty")
                                .mt_2()
                                .px_4()
                                .py_2()
                                .rounded_md()
                                .bg(rgb(accent))
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.show_connection_dialog = true;
                                    cx.notify();
                                }))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(icon_sm("plug", accent_foreground))
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(rgb(accent_foreground))
                                                .child("Connect"),
                                        ),
                                ),
                        )
                    }),
            )
    }

    pub fn render_status_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let panel_background = colors.panel_background;
        let border_variant = colors.border_variant;
        let text = colors.text;
        let text_muted = colors.text_muted;
        let status_success = colors.status_success;
        let status_warning = colors.status_warning;
        let status_error = colors.status_error;

        let status_color = match &self.connection_state {
            super::types::ConnectionState::Connected => status_success,
            super::types::ConnectionState::Connecting => status_warning,
            super::types::ConnectionState::Disconnected => text_muted,
            super::types::ConnectionState::Error(_) => status_error,
        };

        let is_connected = matches!(self.connection_state, super::types::ConnectionState::Connected);

        let status_text = match &self.connection_state {
            super::types::ConnectionState::Connected => format!("{}:{}", self.get_conn_host(cx), self.get_conn_port(cx)),
            super::types::ConnectionState::Connecting => "Connecting...".to_string(),
            super::types::ConnectionState::Disconnected => "Not connected".to_string(),
            super::types::ConnectionState::Error(e) => format!("Error: {}", e),
        };

        let database = self.get_conn_database(cx);

        div()
            .h(px(24.))
            .px_3()
            .flex()
            .items_center()
            .bg(rgb(panel_background))
            .border_t_1()
            .border_color(rgb(border_variant))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().size(px(8.)).rounded_full().bg(rgb(status_color)))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text))
                            .child(status_text),
                    )
                    .when(is_connected, |el| {
                        el.child(div().text_xs().text_color(rgb(text_muted)).child("→"))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(text_muted))
                                    .child(database),
                            )
                    }),
            )
            .child(div().flex_1())
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(text_muted))
                    .child("v0.1.0"),
            )
    }
}
