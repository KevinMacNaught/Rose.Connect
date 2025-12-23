use crate::icons::{icon_md, icon_sm};
use crate::postcommander::page::PostCommanderPage;
use crate::postcommander::types::ConnectionState;
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;

impl PostCommanderPage {
    pub fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let panel_background = colors.panel_background;
        let border_variant = colors.border_variant;
        let surface = colors.surface;
        let text_muted = colors.text_muted;
        let text_placeholder = colors.text_placeholder;
        let text = colors.text;
        let element_hover = colors.element_hover;
        let accent = colors.accent;
        let status_success = colors.status_success;
        let status_warning = colors.status_warning;
        let status_error = colors.status_error;

        let is_connected = matches!(self.connection_state, ConnectionState::Connected);
        let status_color = match &self.connection_state {
            ConnectionState::Connected => status_success,
            ConnectionState::Connecting => status_warning,
            ConnectionState::Disconnected => text_muted,
            ConnectionState::Error(_) => status_error,
        };

        let status_text = match &self.connection_state {
            ConnectionState::Connected => "Connected".to_string(),
            ConnectionState::Connecting => "Connecting...".to_string(),
            ConnectionState::Disconnected => "Disconnected".to_string(),
            ConnectionState::Error(_) => "Error".to_string(),
        };

        let error_text = match &self.connection_state {
            ConnectionState::Connecting => "Connecting...".to_string(),
            ConnectionState::Error(e) => format!("Error: {}", e),
            _ => "Not connected".to_string(),
        };

        let host = self.get_conn_host(cx);
        let port = self.get_conn_port(cx);
        let database = self.get_conn_database(cx);

        div()
            .w(px(self.sidebar_width))
            .h_full()
            .flex()
            .flex_col()
            .bg(rgb(panel_background))
            .border_r_1()
            .border_color(rgb(border_variant))
            .child(self.render_sidebar_header(surface, border_variant, text_muted, text_placeholder))
            .child(self.render_connect_button(cx, element_hover, accent, text, text_muted))
            .when(is_connected, |el| {
                el.child(self.render_tree_view(cx, host, port, database, element_hover, text_muted, text, accent, status_success))
            })
            .when(!is_connected, |el| {
                el.child(self.render_disconnected_state(text_muted, error_text))
            })
            .child(self.render_sidebar_footer(border_variant, status_color, text_muted, status_text))
    }

    fn render_sidebar_header(
        &self,
        surface: u32,
        border_variant: u32,
        text_muted: u32,
        text_placeholder: u32,
    ) -> impl IntoElement {
        div()
            .pt(px(36.))
            .px_3()
            .pb_2()
            .child(
                div()
                    .h(px(28.))
                    .px_2()
                    .flex()
                    .items_center()
                    .gap_2()
                    .rounded_md()
                    .bg(rgb(surface))
                    .border_1()
                    .border_color(rgb(border_variant))
                    .child(icon_sm("search", text_muted))
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(rgb(text_placeholder))
                            .child("Filter tables..."),
                    ),
            )
    }

    fn render_connect_button(
        &self,
        cx: &mut Context<Self>,
        element_hover: u32,
        accent: u32,
        text: u32,
        text_muted: u32,
    ) -> impl IntoElement {
        div()
            .px_3()
            .py_2()
            .child(
                div()
                    .id("connect-server")
                    .px_2()
                    .py_2()
                    .flex()
                    .items_center()
                    .gap_2()
                    .rounded_md()
                    .cursor_pointer()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.show_connection_dialog = true;
                        cx.notify();
                    }))
                    .child(icon_sm("plug", accent))
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(rgb(text))
                            .child("Connect to Server"),
                    )
                    .child(icon_sm("plus", text_muted)),
            )
    }

    fn render_tree_view(
        &self,
        cx: &mut Context<Self>,
        host: String,
        port: String,
        database: String,
        element_hover: u32,
        text_muted: u32,
        text: u32,
        accent: u32,
        status_success: u32,
    ) -> impl IntoElement {
        let server_expanded = self.expanded_nodes.contains("server");
        let db_expanded = self.expanded_nodes.contains("database");
        let schemas_loading = self.schemas_loading;
        let schemas: Vec<_> = self
            .schemas
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        div()
            .id("sidebar-tree-scroll")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .p_2()
            .child(
                div()
                    .id("server-node")
                    .px_2()
                    .py_1()
                    .flex()
                    .items_center()
                    .gap_2()
                    .rounded_md()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_node("server", cx);
                    }))
                    .child(icon_sm(
                        if server_expanded { "chevron-down" } else { "chevron-right" },
                        text_muted,
                    ))
                    .child(icon_sm("server", accent))
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(rgb(text))
                            .child(format!("{}:{}", host, port)),
                    )
                    .child(div().size(px(8.)).rounded_full().bg(rgb(status_success))),
            )
            .when(server_expanded, |el| {
                el.child(self.render_database_node(
                    cx,
                    database,
                    db_expanded,
                    schemas_loading,
                    schemas,
                    element_hover,
                    text_muted,
                    text,
                ))
            })
    }

    fn render_database_node(
        &self,
        cx: &mut Context<Self>,
        database: String,
        db_expanded: bool,
        schemas_loading: bool,
        schemas: Vec<(String, super::types::SchemaObjects)>,
        element_hover: u32,
        text_muted: u32,
        text: u32,
    ) -> impl IntoElement {
        div().pl_4().child(
            div()
                .id("database-node")
                .px_2()
                .py_1()
                .flex()
                .items_center()
                .gap_2()
                .rounded_md()
                .hover(move |s| s.bg(rgb(element_hover)))
                .cursor_pointer()
                .on_click(cx.listener(|this, _, _, cx| {
                    this.toggle_node("database", cx);
                }))
                .child(icon_sm(
                    if db_expanded { "chevron-down" } else { "chevron-right" },
                    text_muted,
                ))
                .child(icon_sm("database", text_muted))
                .child(div().text_sm().text_color(rgb(text)).child(database.clone())),
        )
        .when(db_expanded, |el| {
            el.child(
                div()
                    .pl_4()
                    .when(schemas_loading, |el| {
                        el.child(
                            div()
                                .px_2()
                                .py_1()
                                .text_xs()
                                .text_color(rgb(text_muted))
                                .child("Loading..."),
                        )
                    })
                    .when(!schemas_loading && schemas.is_empty(), |el| {
                        el.child(
                            div()
                                .px_2()
                                .py_1()
                                .text_xs()
                                .text_color(rgb(text_muted))
                                .child("No schemas found"),
                        )
                    })
                    .children(schemas.iter().map(|(schema_name, objects)| {
                        self.render_schema_node(cx, schema_name, objects, element_hover, text_muted, text)
                    })),
            )
        })
    }

    fn render_schema_node(
        &self,
        cx: &mut Context<Self>,
        schema_name: &str,
        objects: &super::types::SchemaObjects,
        element_hover: u32,
        text_muted: u32,
        text: u32,
    ) -> impl IntoElement {
        let schema_key = format!("schema:{}", schema_name);
        let schema_expanded = self.expanded_nodes.contains(&schema_key);
        let tables_key = format!("tables:{}", schema_name);
        let views_key = format!("views:{}", schema_name);
        let tables_expanded = self.expanded_nodes.contains(&tables_key);
        let views_expanded = self.expanded_nodes.contains(&views_key);
        let tables = objects.tables.clone();
        let views = objects.views.clone();
        let schema_name_clone = schema_name.to_string();
        let schema_name_tables = schema_name.to_string();
        let schema_name_views = schema_name.to_string();

        div()
            .child(
                div()
                    .id(SharedString::from(format!("schema-{}", schema_name)))
                    .px_2()
                    .py_1()
                    .flex()
                    .items_center()
                    .gap_2()
                    .rounded_md()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.toggle_node(&format!("schema:{}", schema_name_clone), cx);
                    }))
                    .child(icon_sm(
                        if schema_expanded { "chevron-down" } else { "chevron-right" },
                        text_muted,
                    ))
                    .child(icon_sm("folder", text_muted))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(text))
                            .child(schema_name.to_string()),
                    ),
            )
            .when(schema_expanded, |el| {
                el.child(
                    div()
                        .pl_4()
                        .when(!tables.is_empty(), |el| {
                            el.child(self.render_tables_node(
                                cx,
                                &tables,
                                tables_expanded,
                                schema_name_tables,
                                element_hover,
                                text_muted,
                                text,
                            ))
                        })
                        .when(!views.is_empty(), |el| {
                            el.child(self.render_views_node(
                                cx,
                                &views,
                                views_expanded,
                                schema_name_views,
                                element_hover,
                                text_muted,
                                text,
                            ))
                        }),
                )
            })
    }

    fn render_tables_node(
        &self,
        cx: &mut Context<Self>,
        tables: &[String],
        tables_expanded: bool,
        schema_name: String,
        element_hover: u32,
        text_muted: u32,
        text: u32,
    ) -> impl IntoElement {
        let schema_name_click = schema_name.clone();
        let schema_name_for_items = schema_name.clone();
        let tables_clone = tables.to_vec();
        let context_menu_table = self
            .context_menu
            .as_ref()
            .map(|(_, _, t, _)| t.clone());

        div()
            .child(
                div()
                    .id(SharedString::from(format!("tables-{}", schema_name)))
                    .px_2()
                    .py_1()
                    .flex()
                    .items_center()
                    .gap_2()
                    .rounded_md()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.toggle_node(&format!("tables:{}", schema_name_click), cx);
                    }))
                    .child(icon_sm(
                        if tables_expanded { "chevron-down" } else { "chevron-right" },
                        text_muted,
                    ))
                    .child(icon_sm("table", text_muted))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(text))
                            .child(format!("Tables ({})", tables_clone.len())),
                    ),
            )
            .when(tables_expanded, |el| {
                el.child(
                    div().pl_4().children(tables_clone.iter().map(|table| {
                        let table_name = table.clone();
                        let table_name_dbl = table.clone();
                        let schema_for_menu = schema_name_for_items.clone();
                        let schema_for_dbl = schema_name_for_items.clone();
                        let is_context_target = context_menu_table.as_ref() == Some(table);

                        div()
                            .id(SharedString::from(format!("table-{}-{}", schema_for_menu, table_name)))
                            .px_2()
                            .py_1()
                            .flex()
                            .items_center()
                            .gap_2()
                            .rounded_md()
                            .when(is_context_target, |el| el.bg(rgb(element_hover)))
                            .when(!is_context_target, |el| el.hover(move |s| s.bg(rgb(element_hover))))
                            .cursor_pointer()
                            .child(icon_sm("table-2", text_muted))
                            .child(
                                div()
                                    .min_w_0()
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .text_ellipsis()
                                    .text_sm()
                                    .text_color(rgb(text))
                                    .child(table.clone()),
                            )
                            .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
                                if event.click_count() == 2 {
                                    this.query_table(&schema_for_dbl, &table_name_dbl, window, cx);
                                }
                            }))
                            .on_mouse_down(
                                MouseButton::Right,
                                cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                                    this.deploy_table_context_menu(
                                        event.position,
                                        schema_for_menu.clone(),
                                        table_name.clone(),
                                        window,
                                        cx,
                                    );
                                }),
                            )
                    })),
                )
            })
    }

    fn render_views_node(
        &self,
        cx: &mut Context<Self>,
        views: &[String],
        views_expanded: bool,
        schema_name: String,
        element_hover: u32,
        text_muted: u32,
        text: u32,
    ) -> impl IntoElement {
        let schema_name_click = schema_name.clone();
        let schema_name_for_items = schema_name.clone();
        let views_clone = views.to_vec();

        div()
            .child(
                div()
                    .id(SharedString::from(format!("views-{}", schema_name)))
                    .px_2()
                    .py_1()
                    .flex()
                    .items_center()
                    .gap_2()
                    .rounded_md()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.toggle_node(&format!("views:{}", schema_name_click), cx);
                    }))
                    .child(icon_sm(
                        if views_expanded { "chevron-down" } else { "chevron-right" },
                        text_muted,
                    ))
                    .child(icon_sm("eye", text_muted))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(text))
                            .child(format!("Views ({})", views_clone.len())),
                    ),
            )
            .when(views_expanded, |el| {
                el.child(
                    div().pl_4().children(views_clone.iter().map(|view| {
                        let view_name = view.clone();
                        let schema_for_view = schema_name_for_items.clone();

                        div()
                            .id(SharedString::from(format!("view-{}-{}", schema_for_view, view_name)))
                            .px_2()
                            .py_1()
                            .flex()
                            .items_center()
                            .gap_2()
                            .rounded_md()
                            .hover(move |s| s.bg(rgb(element_hover)))
                            .cursor_pointer()
                            .child(icon_sm("eye", text_muted))
                            .child(
                                div()
                                    .min_w_0()
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .text_ellipsis()
                                    .text_sm()
                                    .text_color(rgb(text))
                                    .child(view.clone()),
                            )
                            .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
                                if event.click_count() == 2 {
                                    this.query_table(&schema_for_view, &view_name, window, cx);
                                }
                            }))
                    })),
                )
            })
    }

    fn render_disconnected_state(&self, text_muted: u32, error_text: String) -> impl IntoElement {
        div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(icon_md("unplug", text_muted))
                    .child(div().text_sm().text_color(rgb(text_muted)).child(error_text)),
            )
    }

    fn render_sidebar_footer(
        &self,
        border_variant: u32,
        status_color: u32,
        text_muted: u32,
        status_text: String,
    ) -> impl IntoElement {
        div()
            .px_3()
            .py_2()
            .border_t_1()
            .border_color(rgb(border_variant))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().size(px(8.)).rounded_full().bg(rgb(status_color)))
                    .child(div().text_xs().text_color(rgb(text_muted)).child(status_text)),
            )
    }
}
