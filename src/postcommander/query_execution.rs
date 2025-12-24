use crate::components::{DataTableColumn, FkDataRequest};
use crate::postcommander::sql_format::format_sql;
use crate::postcommander::sql_safety::{analyze_sql, SqlDangerLevel};
use crate::postcommander::types::TableContext;
use crate::postcommander::ui_helpers::parse_table_from_select;
use crate::postcommander::PostCommanderPage;
use gpui::*;
use std::sync::Arc;

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
}
