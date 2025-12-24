use crate::components::{DataTableColumn, DataTableState, FkDataRequest, TextInput};
use crate::postcommander::database::{ConnectionConfig, DatabaseManager};
use crate::postcommander::sql_completion::SqlCompletionProvider;
use crate::postcommander::sql_format::format_sql;
use crate::postcommander::sql_safety::{analyze_sql, SqlDangerLevel};
use crate::postcommander::types::{CellEditState, ConnectionState, QueryTab, SchemaMap, TableContext, TableStructureInfo};
use crate::postcommander::ui_helpers::parse_table_from_select;
use crate::settings::{AppSettings, ConnectionSettings};
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::menu::PopupMenu;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

pub struct PostCommanderPage {
    pub(crate) sidebar_width: f32,
    pub(crate) is_resizing: bool,
    pub(crate) resize_start_x: f32,
    pub(crate) resize_start_width: f32,
    pub(crate) editor_height: f32,
    pub(crate) is_resizing_editor: bool,
    pub(crate) resize_start_y: f32,
    pub(crate) resize_start_editor_height: f32,
    pub(crate) structure_panel_width: f32,
    pub(crate) is_resizing_structure: bool,
    pub(crate) resize_structure_start_x: f32,
    pub(crate) resize_structure_start_width: f32,
    pub(crate) tabs: Vec<QueryTab>,
    pub(crate) active_tab_id: Option<String>,
    pub(crate) db_manager: Arc<DatabaseManager>,
    pub(crate) connection_state: ConnectionState,
    pub(crate) show_connection_dialog: bool,
    pub(crate) input_host: Entity<TextInput>,
    pub(crate) input_port: Entity<TextInput>,
    pub(crate) input_database: Entity<TextInput>,
    pub(crate) input_username: Entity<TextInput>,
    pub(crate) input_password: Entity<TextInput>,
    pub(crate) expanded_nodes: HashSet<String>,
    pub(crate) schemas: SchemaMap,
    pub(crate) schemas_loading: bool,
    pub(crate) context_menu: Option<(Entity<PopupMenu>, Point<Pixels>, String, Subscription)>,
    pub(crate) cell_edit: Option<CellEditState>,
    pub(crate) export_menu: Option<(Entity<gpui_component::menu::PopupMenu>, Point<Pixels>, Subscription)>,
    pub(crate) _subscriptions: Vec<Subscription>,
    pub(crate) completion_provider: Rc<SqlCompletionProvider>,
    pub(crate) completion_schemas: Rc<RefCell<SchemaMap>>,
    pub(crate) completion_structures: Rc<RefCell<Vec<TableStructureInfo>>>,
    pub(crate) safety_warning: Option<(SqlDangerLevel, String)>,
    pub(crate) pending_capitalization: Option<(String, usize, usize, String)>,
    pub(crate) pending_undo_newline: Option<String>,
}

impl PostCommanderPage {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings = AppSettings::get_global(cx);
        let pc_settings = settings.postcommander();
        let saved_conn = pc_settings.connection.clone();
        let saved_expanded = pc_settings.expanded_nodes.clone();
        let saved_sidebar_width = pc_settings.sidebar_width;
        let saved_editor_height = pc_settings.editor_height;
        let saved_structure_panel_width = pc_settings.structure_panel_width;
        let conn = saved_conn.clone().unwrap_or_default();

        let host_val = if conn.host.is_empty() { "localhost".to_string() } else { conn.host };
        let port_val = if conn.port.is_empty() { "5432".to_string() } else { conn.port };
        let database_val = if conn.database.is_empty() { "postgres".to_string() } else { conn.database };
        let username_val = if conn.username.is_empty() { "postgres".to_string() } else { conn.username };
        let password_val = conn.password;

        let has_saved_connection = saved_conn.is_some()
            && !host_val.is_empty()
            && !database_val.is_empty()
            && !username_val.is_empty();

        let input_host = cx.new(|cx| {
            let mut input = TextInput::new(cx, "localhost");
            input.set_content(host_val);
            input
        });
        let input_port = cx.new(|cx| {
            let mut input = TextInput::new(cx, "5432");
            input.set_content(port_val);
            input
        });
        let input_database = cx.new(|cx| {
            let mut input = TextInput::new(cx, "database");
            input.set_content(database_val);
            input
        });
        let input_username = cx.new(|cx| {
            let mut input = TextInput::new(cx, "username");
            input.set_content(username_val);
            input
        });
        let input_password = cx.new(|cx| {
            let mut input = TextInput::new(cx, "password");
            input.set_content(password_val);
            input.set_masked(true);
            input
        });

        if has_saved_connection {
            cx.spawn(async move |this, cx| {
                let _ = this.update(cx, |this, cx| {
                    this.connect_to_database(cx);
                });
            })
            .detach();
        }

        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_micros(100)).await;
                let result = this.update(cx, |_, cx| {
                    cx.notify();
                });
                if result.is_err() {
                    break;
                }
            }
        })
        .detach();

        let completion_provider = Rc::new(SqlCompletionProvider::new());
        let completion_schemas = completion_provider.schemas_ref();
        let completion_structures = completion_provider.table_structures_ref();

        Self {
            sidebar_width: saved_sidebar_width.unwrap_or(240.0),
            is_resizing: false,
            resize_start_x: 0.0,
            resize_start_width: 0.0,
            editor_height: saved_editor_height.unwrap_or(200.0),
            is_resizing_editor: false,
            resize_start_y: 0.0,
            resize_start_editor_height: 0.0,
            structure_panel_width: saved_structure_panel_width.unwrap_or(280.0),
            is_resizing_structure: false,
            resize_structure_start_x: 0.0,
            resize_structure_start_width: 0.0,
            tabs: vec![],
            active_tab_id: None,
            db_manager: Arc::new(DatabaseManager::new()),
            connection_state: if has_saved_connection {
                ConnectionState::Connecting
            } else {
                ConnectionState::Disconnected
            },
            show_connection_dialog: false,
            input_host,
            input_port,
            input_database,
            input_username,
            input_password,
            expanded_nodes: saved_expanded
                .map(|v| v.into_iter().collect())
                .unwrap_or_default(),
            schemas: SchemaMap::new(),
            schemas_loading: false,
            context_menu: None,
            cell_edit: None,
            export_menu: None,
            _subscriptions: vec![],
            completion_provider,
            completion_schemas,
            completion_structures,
            safety_warning: None,
            pending_capitalization: None,
            pending_undo_newline: None,
        }
    }

    pub(crate) fn get_conn_host(&self, cx: &App) -> String {
        self.input_host.read(cx).content().to_string()
    }

    pub(crate) fn get_conn_port(&self, cx: &App) -> String {
        self.input_port.read(cx).content().to_string()
    }

    pub(crate) fn get_conn_database(&self, cx: &App) -> String {
        self.input_database.read(cx).content().to_string()
    }

    pub(crate) fn get_conn_username(&self, cx: &App) -> String {
        self.input_username.read(cx).content().to_string()
    }

    pub(crate) fn get_conn_password(&self, cx: &App) -> String {
        self.input_password.read(cx).content().to_string()
    }

    pub(crate) fn connect_to_database(&mut self, cx: &mut Context<Self>) {
        let host = self.get_conn_host(cx);
        let port_str = self.get_conn_port(cx);
        let database = self.get_conn_database(cx);
        let username = self.get_conn_username(cx);
        let password = self.get_conn_password(cx);

        let config = ConnectionConfig {
            name: "Local PostgreSQL".to_string(),
            host: host.clone(),
            port: port_str.parse().unwrap_or(5432),
            database: database.clone(),
            username: username.clone(),
            password: password.clone(),
        };

        let conn_settings = ConnectionSettings {
            host: host.clone(),
            port: port_str.clone(),
            database: database.clone(),
            username: username.clone(),
            password: password.clone(),
        };

        self.connection_state = ConnectionState::Connecting;
        self.show_connection_dialog = false;
        cx.notify();

        let rx = self.db_manager.connect(config);
        cx.spawn(async move |this, cx| {
            let result = rx.await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(Ok(())) => {
                        this.connection_state = ConnectionState::Connected;
                        this.expanded_nodes.insert("server".to_string());
                        let nodes: Vec<String> = this.expanded_nodes.iter().cloned().collect();
                        AppSettings::update_global(cx, |settings| {
                            let pc = settings.postcommander_mut();
                            pc.connection = Some(conn_settings.clone());
                            pc.expanded_nodes = Some(nodes);
                        });
                        AppSettings::get_global(cx).save();
                        if this.expanded_nodes.contains("database") && this.schemas.is_empty() {
                            this.fetch_schema_objects(cx);
                        }
                    }
                    Ok(Err(e)) => {
                        this.connection_state = ConnectionState::Error(e.to_string());
                    }
                    Err(_) => {
                        this.connection_state = ConnectionState::Error("Connection failed".to_string());
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub(crate) fn execute_query(&mut self, cx: &mut Context<Self>) {
        self.execute_query_internal(false, cx);
    }

    pub(crate) fn execute_query_force(&mut self, cx: &mut Context<Self>) {
        self.safety_warning = None;
        self.execute_query_internal(true, cx);
    }

    pub(crate) fn cancel_dangerous_query(&mut self, cx: &mut Context<Self>) {
        self.safety_warning = None;
        cx.notify();
    }

    fn execute_query_internal(&mut self, force: bool, cx: &mut Context<Self>) {
        let Some(tab_id) = self.active_tab_id.clone() else {
            return;
        };

        let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) else {
            return;
        };

        let raw_sql = tab.editor.read(cx).value().to_string();
        if raw_sql.trim().is_empty() {
            return;
        }

        let sql = format_sql(&raw_sql);

        if !force {
            let danger_level = analyze_sql(&sql);
            match danger_level {
                SqlDangerLevel::Safe => {}
                SqlDangerLevel::Warning(ref msg) | SqlDangerLevel::Dangerous(ref msg) => {
                    self.safety_warning = Some((danger_level.clone(), msg.clone()));
                    cx.notify();
                    return;
                }
            }
        }

        tab.is_loading = true;
        tab.error = None;
        tab.table_context = None;
        cx.notify();

        let parsed_table = parse_table_from_select(&sql);
        let rx = self.db_manager.execute(sql);
        let tab_id_clone = tab_id.clone();
        let db_manager = self.db_manager.clone();

        cx.spawn(async move |this, cx| {
            let result = rx.await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(Ok(query_result)) => {
                        if let Some(tab) = this.tabs.iter_mut().find(|t| t.id == tab_id_clone) {
                            let columns: Vec<DataTableColumn> = query_result
                                .columns
                                .iter()
                                .map(|c| {
                                    DataTableColumn::new(c.name.clone())
                                        .type_name(c.type_name.clone())
                                })
                                .collect();
                            let rows = query_result.rows.clone();

                            tab.table_state.update(cx, |state, _cx| {
                                state.set_columns(columns);
                                state.set_rows(rows);
                            });
                            tab.result = Some(query_result);
                            tab.is_loading = false;
                            tab.error = None;

                            if let Some((schema, table)) = parsed_table.clone() {
                                let tab_id_for_pk = tab_id_clone.clone();
                                let pk_rx = db_manager.fetch_primary_keys(schema.clone(), table.clone());
                                let fk_rx = db_manager.fetch_foreign_keys(schema.clone(), table.clone());
                                let struct_rx = db_manager.fetch_table_structure(schema.clone(), table.clone());

                                cx.spawn(async move |this, cx| {
                                    let pk_result = pk_rx.await;
                                    let fk_result = fk_rx.await;
                                    let struct_result = struct_rx.await;

                                    let _ = this.update(cx, |this, cx| {
                                        let primary_keys = match pk_result {
                                            Ok(Ok(pks)) => pks,
                                            _ => vec![],
                                        };

                                        let foreign_keys = match fk_result {
                                            Ok(Ok(fks)) => fks
                                                .into_iter()
                                                .map(|fk| (fk.column_name.clone(), fk))
                                                .collect(),
                                            _ => std::collections::HashMap::new(),
                                        };

                                        let context = TableContext {
                                            schema: schema.clone(),
                                            table: table.clone(),
                                            primary_keys,
                                            foreign_keys,
                                        };

                                        if let Some(tab) = this.tabs.iter_mut().find(|t| t.id == tab_id_for_pk) {
                                            tab.table_context = Some(context.clone());
                                            tab.table_state.update(cx, |state, _cx| {
                                                state.set_table_context(Some(context));
                                            });

                                            if let Ok(Ok(structure)) = struct_result {
                                                let key = format!("{}.{}", structure.schema, structure.table);
                                                tab.table_structures = vec![structure.clone()];
                                                tab.structure_expanded.insert(key, true);
                                                *this.completion_structures.borrow_mut() = vec![structure];
                                            }
                                        }

                                        cx.notify();
                                    });
                                }).detach();
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        if let Some(tab) = this.tabs.iter_mut().find(|t| t.id == tab_id_clone) {
                            tab.table_state.update(cx, |state, cx| {
                                state.clear();
                                cx.notify();
                            });
                            tab.error = Some(e.to_string());
                            tab.is_loading = false;
                        }
                    }
                    Err(_) => {
                        if let Some(tab) = this.tabs.iter_mut().find(|t| t.id == tab_id_clone) {
                            tab.table_state.update(cx, |state, cx| {
                                state.clear();
                                cx.notify();
                            });
                            tab.error = Some("Query execution failed".to_string());
                            tab.is_loading = false;
                        }
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub(crate) fn handle_fk_data_request(
        &mut self,
        table_state: Entity<DataTableState>,
        event: &FkDataRequest,
        cx: &mut Context<Self>,
    ) {
        let db_manager = self.db_manager.clone();
        let fk_info = event.fk_info.clone();
        let cell_value = event.cell_value.to_string();

        let rx = db_manager.fetch_fk_referenced_row(
            fk_info.referenced_schema.clone(),
            fk_info.referenced_table.clone(),
            fk_info.referenced_column.clone(),
            cell_value,
        );

        cx.spawn(async move |_this, cx| {
            let result = rx.await;
            let _ = table_state.update(cx, |state, cx| {
                match result {
                    Ok(Ok(data)) => {
                        state.set_fk_card_data(data, cx);
                    }
                    Ok(Err(e)) => {
                        state.set_fk_card_error(e.to_string(), cx);
                    }
                    Err(_) => {
                        state.set_fk_card_error("Failed to fetch data".to_string(), cx);
                    }
                }
            });
        })
        .detach();
    }

    pub(crate) fn deploy_export_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        use gpui_component::menu::PopupMenuItem;

        let e_save_csv = cx.entity().downgrade();
        let e_save_json = cx.entity().downgrade();
        let e_save_md = cx.entity().downgrade();
        let e_copy_csv = cx.entity().downgrade();
        let e_copy_json = cx.entity().downgrade();
        let e_copy_md = cx.entity().downgrade();

        let menu = PopupMenu::build(window, cx, move |menu, _window, _cx| {
            menu.item(
                PopupMenuItem::new("Save as CSV").on_click({
                    let entity = e_save_csv.clone();
                    move |_, _window, cx| {
                        if let Some(page) = entity.upgrade() {
                            page.update(cx, |page, cx| {
                                let _ = page.save_csv_to_file(cx);
                            });
                        }
                    }
                }),
            )
            .item(
                PopupMenuItem::new("Save as JSON").on_click({
                    let entity = e_save_json.clone();
                    move |_, _window, cx| {
                        if let Some(page) = entity.upgrade() {
                            page.update(cx, |page, cx| {
                                let _ = page.save_json_to_file(cx);
                            });
                        }
                    }
                }),
            )
            .item(
                PopupMenuItem::new("Save as Markdown").on_click({
                    let entity = e_save_md.clone();
                    move |_, _window, cx| {
                        if let Some(page) = entity.upgrade() {
                            page.update(cx, |page, cx| {
                                let _ = page.save_markdown_to_file(cx);
                            });
                        }
                    }
                }),
            )
            .separator()
            .item(
                PopupMenuItem::new("Copy as CSV").on_click({
                    let entity = e_copy_csv.clone();
                    move |_, _window, cx| {
                        if let Some(page) = entity.upgrade() {
                            page.update(cx, |page, cx| {
                                if let Some(csv) = page.export_to_csv(cx) {
                                    page.copy_to_clipboard(csv, cx);
                                }
                            });
                        }
                    }
                }),
            )
            .item(
                PopupMenuItem::new("Copy as JSON").on_click({
                    let entity = e_copy_json.clone();
                    move |_, _window, cx| {
                        if let Some(page) = entity.upgrade() {
                            page.update(cx, |page, cx| {
                                if let Some(json) = page.export_to_json(cx) {
                                    page.copy_to_clipboard(json, cx);
                                }
                            });
                        }
                    }
                }),
            )
            .item(
                PopupMenuItem::new("Copy as Markdown").on_click({
                    let entity = e_copy_md.clone();
                    move |_, _window, cx| {
                        if let Some(page) = entity.upgrade() {
                            page.update(cx, |page, cx| {
                                if let Some(md) = page.export_to_markdown(cx) {
                                    page.copy_to_clipboard(md, cx);
                                }
                            });
                        }
                    }
                }),
            )
        });

        let subscription = cx.subscribe(&menu, |this, _, _: &DismissEvent, cx| {
            this.export_menu = None;
            cx.notify();
        });

        self.export_menu = Some((menu, position, subscription));
        cx.notify();
    }

    pub(crate) fn deploy_table_context_menu(
        &mut self,
        position: Point<Pixels>,
        schema: String,
        table: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        use gpui_component::menu::PopupMenuItem;

        let schema_clone = schema.clone();
        let table_clone = table.clone();
        let entity = cx.entity().downgrade();

        let menu = PopupMenu::build(window, cx, move |menu, _window, _cx| {
            let schema = schema_clone.clone();
            let table = table_clone.clone();
            let entity = entity.clone();

            menu.item(
                PopupMenuItem::new("Select Top 100").on_click(move |_, window, cx| {
                    if let Some(page) = entity.upgrade() {
                        page.update(cx, |page, cx| {
                            page.query_table(&schema, &table, window, cx);
                        });
                    }
                }),
            )
        });

        let subscription = cx.subscribe(&menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu = None;
            cx.notify();
        });

        self.context_menu = Some((menu, position, table, subscription));
        cx.notify();
    }

    pub(crate) fn toggle_node(&mut self, node_id: &str, cx: &mut Context<Self>) {
        if self.expanded_nodes.contains(node_id) {
            self.expanded_nodes.remove(node_id);
        } else {
            self.expanded_nodes.insert(node_id.to_string());
            if node_id == "database" && self.schemas.is_empty() && !self.schemas_loading {
                self.fetch_schema_objects(cx);
            }
        }
        self.save_expanded_nodes(cx);
        cx.notify();
    }

    fn save_expanded_nodes(&self, cx: &mut Context<Self>) {
        let nodes: Vec<String> = self.expanded_nodes.iter().cloned().collect();
        AppSettings::update_global(cx, |settings| {
            settings.postcommander_mut().expanded_nodes = Some(nodes);
        });
        AppSettings::get_global(cx).save();
    }

    fn save_sidebar_width(&self, cx: &mut Context<Self>) {
        AppSettings::update_global(cx, |settings| {
            settings.postcommander_mut().sidebar_width = Some(self.sidebar_width);
        });
        AppSettings::get_global(cx).save();
    }

    fn save_editor_height(&self, cx: &mut Context<Self>) {
        AppSettings::update_global(cx, |settings| {
            settings.postcommander_mut().editor_height = Some(self.editor_height);
        });
        AppSettings::get_global(cx).save();
    }

    pub(crate) fn save_structure_panel_width(&self, cx: &mut Context<Self>) {
        AppSettings::update_global(cx, |settings| {
            settings.postcommander_mut().structure_panel_width = Some(self.structure_panel_width);
        });
        AppSettings::get_global(cx).save();
    }

    fn fetch_schema_objects(&mut self, cx: &mut Context<Self>) {
        self.schemas_loading = true;
        cx.notify();

        let sql = r#"
            SELECT
                table_schema,
                table_name,
                table_type
            FROM information_schema.tables
            WHERE table_schema NOT IN ('pg_catalog', 'information_schema')
            ORDER BY table_schema, table_type, table_name
        "#
        .to_string();

        let rx = self.db_manager.execute(sql);

        cx.spawn(async move |this, cx| {
            let result = rx.await;
            let _ = this.update(cx, |this, cx| {
                this.schemas_loading = false;
                match result {
                    Ok(Ok(query_result)) => {
                        let mut schemas: SchemaMap = SchemaMap::new();
                        for row in query_result.rows.iter() {
                            if row.len() >= 3 {
                                let schema = row[0].to_string();
                                let name = row[1].to_string();
                                let obj_type = row[2].as_ref();

                                let entry = schemas.entry(schema).or_default();
                                if obj_type == "VIEW" {
                                    entry.views.push(name);
                                } else {
                                    entry.tables.push(name);
                                }
                            }
                        }
                        *this.completion_schemas.borrow_mut() = schemas.clone();
                        this.schemas = schemas;
                    }
                    _ => {
                        *this.completion_schemas.borrow_mut() = SchemaMap::new();
                        this.schemas = SchemaMap::new();
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn render_resize_handle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let border_variant = colors.border_variant;
        let accent = colors.accent;
        let is_resizing = self.is_resizing;

        div()
            .id("sidebar-resize-handle")
            .w(px(4.))
            .h_full()
            .cursor_col_resize()
            .bg(transparent_black())
            .when(is_resizing, |el| el.bg(rgb(accent)))
            .hover(move |s| s.bg(rgb(border_variant)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.is_resizing = true;
                    this.resize_start_x = f32::from(event.position.x);
                    this.resize_start_width = this.sidebar_width;
                    cx.notify();
                }),
            )
    }

    fn render_resize_overlay(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("resize-overlay")
            .absolute()
            .inset_0()
            .cursor_col_resize()
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if this.is_resizing {
                    let delta = f32::from(event.position.x) - this.resize_start_x;
                    let new_width = (this.resize_start_width + delta).clamp(180.0, 500.0);
                    this.sidebar_width = new_width;
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.is_resizing = false;
                    this.save_sidebar_width(cx);
                    cx.notify();
                }),
            )
    }

    pub(crate) fn render_editor_resize_handle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let border_variant = colors.border_variant;
        let accent = colors.accent;
        let is_resizing = self.is_resizing_editor;

        div()
            .id("editor-resize-handle")
            .w_full()
            .h(px(4.))
            .cursor_row_resize()
            .bg(transparent_black())
            .when(is_resizing, |el| el.bg(rgb(accent)))
            .hover(move |s| s.bg(rgb(border_variant)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.is_resizing_editor = true;
                    this.resize_start_y = f32::from(event.position.y);
                    this.resize_start_editor_height = this.editor_height;
                    cx.notify();
                }),
            )
    }

    fn render_editor_resize_overlay(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("editor-resize-overlay")
            .absolute()
            .inset_0()
            .cursor_row_resize()
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if this.is_resizing_editor {
                    let delta = f32::from(event.position.y) - this.resize_start_y;
                    let new_height = (this.resize_start_editor_height + delta).clamp(100.0, 600.0);
                    this.editor_height = new_height;
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.is_resizing_editor = false;
                    this.save_editor_height(cx);
                    cx.notify();
                }),
            )
    }

    fn render_safety_warning_dialog(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        let Some((danger_level, message)) = &self.safety_warning else {
            return div().into_any_element();
        };

        let is_dangerous = matches!(danger_level, SqlDangerLevel::Dangerous(_));
        let title = if is_dangerous { "Dangerous Query" } else { "Warning" };
        let icon_color = if is_dangerous { colors.status_error } else { colors.status_warning };

        div()
            .id("safety-warning-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x00000080))
            .on_mouse_down(MouseButton::Left, |_, _, _| {})
            .child(
                div()
                    .w(px(420.0))
                    .bg(rgb(colors.surface))
                    .border_1()
                    .border_color(rgb(colors.border))
                    .rounded_lg()
                    .shadow_lg()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                crate::icons::icon("alert-triangle", px(24.0), icon_color)
                            )
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(rgb(colors.text))
                                    .child(title)
                            )
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(colors.text_muted))
                            .child(message.clone())
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                div()
                                    .id("cancel-btn")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_md()
                                    .bg(rgb(colors.element))
                                    .text_sm()
                                    .text_color(rgb(colors.text))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(colors.element_hover)))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.cancel_dangerous_query(cx);
                                    }))
                                    .child("Cancel")
                            )
                            .child(
                                div()
                                    .id("proceed-btn")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_md()
                                    .bg(rgb(icon_color))
                                    .text_sm()
                                    .text_color(rgb(colors.accent_foreground))
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.9))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.execute_query_force(cx);
                                    }))
                                    .child("Execute Anyway")
                            )
                    )
            )
            .into_any_element()
    }
}

impl Render for PostCommanderPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some((tab_id, start, end, replacement)) = self.pending_capitalization.take() {
            if let Some(tab) = self.tabs.iter().find(|t| t.id == tab_id) {
                tab.editor.update(cx, |editor, cx| {
                    let text = editor.value().to_string();
                    let cursor = text.len();
                    if start < text.len() && end <= text.len() {
                        let new_text = format!("{}{}{}", &text[..start], replacement, &text[end..]);
                        editor.set_value(new_text, window, cx);
                        editor.set_cursor_position(
                            gpui_component::input::Position { line: 0, character: cursor as u32 },
                            window,
                            cx,
                        );
                    }
                });
            }
        }

        if let Some(tab_id) = self.pending_undo_newline.take() {
            if let Some(tab) = self.tabs.iter().find(|t| t.id == tab_id) {
                tab.editor.update(cx, |editor, cx| {
                    let text = editor.value().to_string();
                    if text.ends_with('\n') {
                        let new_text = text.trim_end_matches('\n').to_string();
                        let cursor = new_text.len();
                        editor.set_value(new_text, window, cx);
                        editor.set_cursor_position(
                            gpui_component::input::Position { line: 0, character: cursor as u32 },
                            window,
                            cx,
                        );
                    }
                });
            }
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let background = colors.background;

        let has_tabs = !self.tabs.is_empty();
        let show_dialog = self.show_connection_dialog;
        let is_resizing = self.is_resizing;
        let is_resizing_editor = self.is_resizing_editor;
        let is_resizing_structure = self.is_resizing_structure;
        let show_cell_edit = self.cell_edit.is_some();
        let show_safety_warning = self.safety_warning.is_some();
        let context_menu = self
            .context_menu
            .as_ref()
            .map(|(menu, pos, _, _)| (menu.clone(), *pos));
        let export_menu = self
            .export_menu
            .as_ref()
            .map(|(menu, pos, _)| (menu.clone(), *pos));

        div()
            .id("postcommander-page")
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(background))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                if event.keystroke.key == "enter" && event.keystroke.modifiers.platform {
                    this.execute_query(cx);
                }
            }))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .child(self.render_sidebar(cx))
                    .child(self.render_resize_handle(cx))
                    .child(
                        div()
                            .flex_1()
                            .min_h_0()
                            .flex()
                            .flex_col()
                            .child(self.render_tabs_bar(cx))
                            .when(has_tabs, |el| {
                                el.child(
                                    div()
                                        .flex()
                                        .h(px(self.editor_height))
                                        .child(self.render_query_editor_content(cx))
                                        .child(self.render_structure_resize_handle(cx))
                                        .child(self.render_structure_panel(cx)),
                                )
                                .child(self.render_editor_resize_handle(cx))
                                .child(self.render_results_area(cx))
                            })
                            .when(!has_tabs, |el| el.child(self.render_empty_state(cx))),
                    ),
            )
            .when(show_dialog, |el| el.child(self.render_connection_dialog(cx)))
            .when(is_resizing, |el| el.child(self.render_resize_overlay(cx)))
            .when(is_resizing_editor, |el| el.child(self.render_editor_resize_overlay(cx)))
            .when(is_resizing_structure, |el| el.child(self.render_structure_resize_overlay(cx)))
            .when_some(context_menu, |el, (menu, position)| {
                let window_size = window.bounds().size;
                el.child(
                    deferred(
                        anchored().child(
                            div()
                                .w(window_size.width)
                                .h(window_size.height)
                                .occlude()
                                .child(
                                    anchored()
                                        .position(position)
                                        .anchor(Corner::TopLeft)
                                        .child(menu),
                                ),
                        ),
                    )
                    .with_priority(1),
                )
            })
            .when(show_cell_edit, |el| {
                el.child(deferred(self.render_cell_edit_modal(window, cx)).with_priority(2))
            })
            .when_some(export_menu, |el, (menu, position)| {
                let window_size = window.bounds().size;
                el.child(
                    deferred(
                        anchored().child(
                            div()
                                .w(window_size.width)
                                .h(window_size.height)
                                .occlude()
                                .child(
                                    anchored()
                                        .position(position)
                                        .anchor(Corner::TopRight)
                                        .child(menu),
                                ),
                        ),
                    )
                    .with_priority(1),
                )
            })
            .when(show_safety_warning, |el| {
                el.child(deferred(self.render_safety_warning_dialog(cx)).with_priority(3))
            })
    }
}
