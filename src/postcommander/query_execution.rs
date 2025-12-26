use crate::components::{DataTableColumn, FkDataRequest};
use crate::postcommander::sql::{analyze_sql, format_sql, SqlDangerLevel};
use crate::postcommander::types::TableContext;
use crate::postcommander::ui_helpers::parse_table_from_select;
use crate::postcommander::PostCommanderPage;
use crate::settings::{AppSettings, QueryHistoryEntry, QueryHistorySettings, QueryHistoryStatus};
use chrono::Utc;
use gpui::*;
use gpui_component::input::Position;
use std::sync::Arc;
use std::time::Instant;

impl PostCommanderPage {
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

    pub(crate) fn cancel_query(&mut self, cx: &mut Context<Self>) {
        let Some(tab_id) = self.active_tab_id.clone() else {
            return;
        };

        let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) else {
            return;
        };

        if !tab.is_loading {
            return;
        }

        let sql = tab.editor.read(cx).value().to_string();
        let formatted_sql = format_sql(&sql);
        let execution_ms = tab.query_start_time.map(|t| t.elapsed().as_millis() as u64);
        let database = tab.database.clone();

        let entry = QueryHistoryEntry {
            sql: formatted_sql,
            timestamp: Utc::now().to_rfc3339(),
            execution_ms,
            status: QueryHistoryStatus::Cancelled,
            database: Some(database),
        };

        AppSettings::update_global(cx, |settings| {
            let pc = settings.postcommander_mut();
            let history = pc.query_history.get_or_insert_with(QueryHistorySettings::default);
            history.add_entry(entry);
        });
        AppSettings::get_global(cx).save();

        tab.query_task = None;
        tab.is_loading = false;
        tab.query_start_time = None;
        tab.error = Some("Query cancelled".to_string());
        cx.notify();
    }

    pub(crate) fn execute_query_internal(&mut self, force: bool, cx: &mut Context<Self>) {
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
        tab.query_start_time = Some(Instant::now());
        cx.notify();

        let parsed_table = parse_table_from_select(&sql);
        let rx = self.db_manager.execute(sql.clone());
        let tab_id_clone = tab_id.clone();
        let db_manager = self.db_manager.clone();
        let sql_for_history = sql.clone();
        let database_for_history = tab.database.clone();

        cx.spawn({
            let tab_id_for_refresh = tab_id.clone();
            async move |this, cx| {
                loop {
                    cx.background_executor().timer(std::time::Duration::from_millis(100)).await;
                    let should_continue = this.update(cx, |this, cx| {
                        if let Some(tab) = this.tabs.iter().find(|t| t.id == tab_id_for_refresh) {
                            if tab.is_loading {
                                cx.notify();
                                return true;
                            }
                        }
                        false
                    }).unwrap_or(false);

                    if !should_continue {
                        break;
                    }
                }
            }
        }).detach();

        let task = cx.spawn(async move |this, cx| {
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

                            let execution_ms = tab.query_start_time.map(|t| t.elapsed().as_millis() as u64);

                            let entry = QueryHistoryEntry {
                                sql: sql_for_history.clone(),
                                timestamp: Utc::now().to_rfc3339(),
                                execution_ms,
                                status: QueryHistoryStatus::Success,
                                database: Some(database_for_history.clone()),
                            };

                            AppSettings::update_global(cx, |settings| {
                                let pc = settings.postcommander_mut();
                                let history = pc.query_history.get_or_insert_with(QueryHistorySettings::default);
                                history.add_entry(entry);
                            });
                            AppSettings::get_global(cx).save();

                            tab.table_state.update(cx, |state, _cx| {
                                state.set_columns(columns);
                                state.set_rows(rows);
                            });
                            tab.result = Some(query_result);
                            tab.is_loading = false;
                            tab.query_start_time = None;
                            tab.query_task = None;
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
                                            Ok(Ok(fks)) => Arc::new(fks
                                                .into_iter()
                                                .map(|fk| (fk.column_name.clone(), fk))
                                                .collect()),
                                            _ => Arc::new(std::collections::HashMap::new()),
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
                            let execution_ms = tab.query_start_time.map(|t| t.elapsed().as_millis() as u64);

                            let entry = QueryHistoryEntry {
                                sql: sql_for_history.clone(),
                                timestamp: Utc::now().to_rfc3339(),
                                execution_ms,
                                status: QueryHistoryStatus::Error(e.to_string()),
                                database: Some(database_for_history.clone()),
                            };

                            AppSettings::update_global(cx, |settings| {
                                let pc = settings.postcommander_mut();
                                let history = pc.query_history.get_or_insert_with(QueryHistorySettings::default);
                                history.add_entry(entry);
                            });
                            AppSettings::get_global(cx).save();

                            tab.table_state.update(cx, |state, cx| {
                                state.clear();
                                cx.notify();
                            });
                            tab.error = Some(e.to_string());
                            tab.is_loading = false;
                            tab.query_start_time = None;
                            tab.query_task = None;
                        }
                    }
                    Err(_) => {
                        if let Some(tab) = this.tabs.iter_mut().find(|t| t.id == tab_id_clone) {
                            let execution_ms = tab.query_start_time.map(|t| t.elapsed().as_millis() as u64);

                            let entry = QueryHistoryEntry {
                                sql: sql_for_history.clone(),
                                timestamp: Utc::now().to_rfc3339(),
                                execution_ms,
                                status: QueryHistoryStatus::Error("Query execution failed".to_string()),
                                database: Some(database_for_history.clone()),
                            };

                            AppSettings::update_global(cx, |settings| {
                                let pc = settings.postcommander_mut();
                                let history = pc.query_history.get_or_insert_with(QueryHistorySettings::default);
                                history.add_entry(entry);
                            });
                            AppSettings::get_global(cx).save();

                            tab.table_state.update(cx, |state, cx| {
                                state.clear();
                                cx.notify();
                            });
                            tab.error = Some("Query execution failed".to_string());
                            tab.is_loading = false;
                            tab.query_start_time = None;
                            tab.query_task = None;
                        }
                    }
                }
                cx.notify();
            });
        });

        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
            tab.query_task = Some(task);
        }
    }

    pub(crate) fn handle_fk_data_request(
        &mut self,
        table_state: Entity<crate::components::DataTableState>,
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

    pub(crate) fn explain_query(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(tab_id) = self.active_tab_id else {
            return;
        };

        let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) else {
            return;
        };

        tab.editor.update(cx, |editor, cx| {
            let current_text = editor.value().to_string();
            let trimmed = current_text.trim();
            if trimmed.is_empty() {
                return;
            }

            let new_text = format!("EXPLAIN {}", trimmed);
            let cursor_pos = new_text.len() as u32;
            editor.set_value(new_text, window, cx);
            editor.set_cursor_position(Position { line: 0, character: cursor_pos }, window, cx);
        });

        self.execute_query(cx);
    }

    pub(crate) fn explain_analyze_query(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(tab_id) = self.active_tab_id else {
            return;
        };

        let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) else {
            return;
        };

        tab.editor.update(cx, |editor, cx| {
            let current_text = editor.value().to_string();
            let trimmed = current_text.trim();
            if trimmed.is_empty() {
                return;
            }

            let new_text = format!("EXPLAIN ANALYZE {}", trimmed);
            let cursor_pos = new_text.len() as u32;
            editor.set_value(new_text, window, cx);
            editor.set_cursor_position(Position { line: 0, character: cursor_pos }, window, cx);
        });

        self.execute_query(cx);
    }

    pub(crate) fn toggle_comment(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(tab_id) = self.active_tab_id else {
            return;
        };

        let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) else {
            return;
        };

        tab.editor.update(cx, |editor, cx| {
            let current_text = editor.value().to_string();
            let cursor = editor.cursor_position();

            let lines: Vec<String> = current_text.split('\n').map(|s| s.to_string()).collect();
            if lines.is_empty() {
                return;
            }

            let current_line_idx = cursor.line as usize;
            if current_line_idx >= lines.len() {
                return;
            }

            let current_line = &lines[current_line_idx];
            let trimmed = current_line.trim_start();

            let new_line = if trimmed.starts_with("-- ") {
                current_line.replacen("-- ", "", 1)
            } else if trimmed.starts_with("--") {
                current_line.replacen("--", "", 1)
            } else {
                let leading_spaces = current_line.len() - trimmed.len();
                format!("{}-- {}", " ".repeat(leading_spaces), trimmed)
            };

            let mut new_lines = lines.clone();
            new_lines[current_line_idx] = new_line.clone();
            let new_text = new_lines.join("\n");

            let new_cursor_pos = new_line.len().min(cursor.character as usize) as u32;
            editor.set_value(new_text, window, cx);
            editor.set_cursor_position(
                Position {
                    line: cursor.line,
                    character: new_cursor_pos
                },
                window,
                cx,
            );
        });
    }

    pub(crate) fn format_query(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(tab_id) = self.active_tab_id else {
            return;
        };

        let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) else {
            return;
        };

        tab.editor.update(cx, |editor, cx| {
            let current_text = editor.value().to_string();
            if current_text.trim().is_empty() {
                return;
            }

            let formatted = sqlformat::format(
                &current_text,
                &sqlformat::QueryParams::None,
                sqlformat::FormatOptions {
                    indent: sqlformat::Indent::Spaces(2),
                    uppercase: true,
                    lines_between_queries: 1,
                },
            );

            editor.set_value(formatted, window, cx);
        });
    }

    pub(crate) fn show_ai_assistant_placeholder(&mut self, cx: &mut Context<Self>) {
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_secs(3))
                .await;
            let _ = this.update(cx, |this, cx| {
                this.temporary_message = None;
                cx.notify();
            });
        });

        self.temporary_message = Some(("AI SQL Assistant coming soon".to_string(), task));
        cx.notify();
    }
}
