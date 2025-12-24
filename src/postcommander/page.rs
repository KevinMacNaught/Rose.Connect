use crate::components::TextInput;
use crate::postcommander::database::{ConnectionConfig, DatabaseManager};
use crate::postcommander::sql_completion::SqlCompletionProvider;
use crate::postcommander::sql_safety::SqlDangerLevel;
use crate::postcommander::types::{CellEditState, ConnectionState, QueryTab, SchemaMap, TabId, TableStructureInfo};
use crate::settings::{AppSettings, ConnectionSettings};
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::menu::PopupMenu;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;

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
    pub(crate) active_tab_id: Option<TabId>,
    pub(crate) db_manager: Arc<DatabaseManager>,
    pub(crate) connection_state: ConnectionState,
    pub(crate) show_connection_dialog: bool,
    pub(crate) input_host: Entity<TextInput>,
    pub(crate) input_port: Entity<TextInput>,
    pub(crate) input_database: Entity<TextInput>,
    pub(crate) input_username: Entity<TextInput>,
    pub(crate) input_password: Entity<TextInput>,
    pub(crate) expanded_nodes: HashSet<String>,
    pub(crate) schemas: Arc<SchemaMap>,
    pub(crate) schemas_loading: bool,
    pub(crate) context_menu: Option<(Entity<PopupMenu>, Point<Pixels>, String, Subscription)>,
    pub(crate) cell_edit: Option<CellEditState>,
    pub(crate) export_menu: Option<(Entity<gpui_component::menu::PopupMenu>, Point<Pixels>, Subscription)>,
    pub(crate) _subscriptions: Vec<Subscription>,
    pub(crate) completion_provider: Rc<SqlCompletionProvider>,
    pub(crate) completion_schemas: Rc<RefCell<SchemaMap>>,
    pub(crate) completion_structures: Rc<RefCell<Vec<TableStructureInfo>>>,
    pub(crate) safety_warning: Option<(SqlDangerLevel, String)>,
    pub(crate) pending_capitalization: Option<(TabId, usize, usize, String)>,
    pub(crate) pending_undo_newline: Option<TabId>,
    cached_connection: ConnectionInfo,
}

#[derive(Default, Clone)]
struct ConnectionInfo {
    host: String,
    port: String,
    database: String,
    username: String,
    password: String,
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

        let cached_connection = ConnectionInfo {
            host: host_val.clone(),
            port: port_val.clone(),
            database: database_val.clone(),
            username: username_val.clone(),
            password: password_val.clone(),
        };

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
            schemas: Arc::new(SchemaMap::new()),
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
            cached_connection,
        }
    }

    pub(crate) fn get_conn_host(&self) -> &str {
        &self.cached_connection.host
    }

    pub(crate) fn get_conn_port(&self) -> &str {
        &self.cached_connection.port
    }

    pub(crate) fn get_conn_database(&self) -> &str {
        &self.cached_connection.database
    }

    pub(crate) fn get_conn_username(&self) -> &str {
        &self.cached_connection.username
    }

    pub(crate) fn get_conn_password(&self) -> &str {
        &self.cached_connection.password
    }

    fn update_cached_connection(&mut self, cx: &App) {
        self.cached_connection.host = self.input_host.read(cx).content().to_string();
        self.cached_connection.port = self.input_port.read(cx).content().to_string();
        self.cached_connection.database = self.input_database.read(cx).content().to_string();
        self.cached_connection.username = self.input_username.read(cx).content().to_string();
        self.cached_connection.password = self.input_password.read(cx).content().to_string();
    }

    pub(crate) fn connect_to_database(&mut self, cx: &mut Context<Self>) {
        self.update_cached_connection(cx);
        let host = self.get_conn_host();
        let port_str = self.get_conn_port();
        let database = self.get_conn_database();
        let username = self.get_conn_username();
        let password = self.get_conn_password();

        let config = ConnectionConfig {
            name: "Local PostgreSQL".to_string(),
            host: host.to_string(),
            port: port_str.parse().unwrap_or(5432),
            database: database.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        };

        let conn_settings = ConnectionSettings {
            host: host.to_string(),
            port: port_str.to_string(),
            database: database.to_string(),
            username: username.to_string(),
            password: password.to_string(),
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
                        this.schemas = Arc::new(schemas);
                    }
                    _ => {
                        *this.completion_schemas.borrow_mut() = SchemaMap::new();
                        this.schemas = Arc::new(SchemaMap::new());
                    }
                }
                cx.notify();
            });
        })
        .detach();
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
