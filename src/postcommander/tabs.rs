use crate::components::DataTableState;
use crate::postcommander::page::PostCommanderPage;
use crate::postcommander::sql_format::maybe_capitalize_last_word;
use crate::postcommander::types::QueryTab;
use gpui::*;
use gpui_component::input::{InputEvent, InputState};
use std::collections::HashMap;
use std::rc::Rc;

impl PostCommanderPage {
    pub(crate) fn add_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = format!("tab-{}", self.tabs.len() + 1);
        let tab_id = id.clone();
        let database = self.get_conn_database(cx);

        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("sql".to_string())
                .line_number(true)
                .soft_wrap(true)
                .default_value("SELECT * FROM ")
                .placeholder("Enter SQL query...")
        });

        let provider = self.completion_provider.clone();
        editor.update(cx, |editor, cx| {
            editor.set_cursor_position(
                gpui_component::input::Position { line: 0, character: 14 },
                window,
                cx,
            );
            editor.lsp.completion_provider = Some(Rc::clone(&provider) as Rc<dyn gpui_component::input::CompletionProvider>);
        });

        let editor_sub = cx.subscribe(&editor, move |this, editor_entity, event: &InputEvent, cx| {
            match event {
                InputEvent::Change => {
                    Self::handle_editor_change(this, &tab_id, editor_entity, cx);
                }
                InputEvent::PressEnter { secondary: true } => {
                    this.pending_undo_newline = Some(tab_id.clone());
                    this.execute_query(cx);
                }
                _ => {}
            }
        });
        self._subscriptions.push(editor_sub);

        let table_state = cx.new(|cx| DataTableState::new(cx));

        let sub = cx.subscribe(&table_state, Self::handle_cell_save);
        self._subscriptions.push(sub);
        let sub2 = cx.subscribe(&table_state, Self::handle_cell_double_click);
        self._subscriptions.push(sub2);
        let sub3 = cx.subscribe(&table_state, Self::handle_fk_data_request);
        self._subscriptions.push(sub3);

        let tab = QueryTab {
            id: id.clone(),
            name: format!("Query {}", self.tabs.len() + 1),
            database,
            editor,
            table_state,
            table_context: None,
            result: None,
            error: None,
            is_loading: false,
            last_export_message: None,
            table_structures: vec![],
            structure_loading: false,
            structure_expanded: HashMap::new(),
        };
        self.tabs.push(tab);
        self.active_tab_id = Some(id);
        cx.notify();
    }

    pub(crate) fn query_table(&mut self, schema: &str, table: &str, window: &mut Window, cx: &mut Context<Self>) {
        let id = format!("tab-{}", self.tabs.len() + 1);
        let tab_id = id.clone();
        let database = self.get_conn_database(cx);
        let sql = format!("SELECT * FROM \"{}\".\"{}\" LIMIT 100;", schema, table);
        let cursor_pos = sql.len() as u32;

        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("sql".to_string())
                .line_number(true)
                .soft_wrap(true)
                .default_value(&sql)
                .placeholder("Enter SQL query...")
        });

        let provider = self.completion_provider.clone();
        editor.update(cx, |editor, cx| {
            editor.set_cursor_position(
                gpui_component::input::Position { line: 0, character: cursor_pos },
                window,
                cx,
            );
            editor.lsp.completion_provider = Some(Rc::clone(&provider) as Rc<dyn gpui_component::input::CompletionProvider>);
        });

        let editor_sub = cx.subscribe(&editor, move |this, editor_entity, event: &InputEvent, cx| {
            match event {
                InputEvent::Change => {
                    Self::handle_editor_change(this, &tab_id, editor_entity, cx);
                }
                InputEvent::PressEnter { secondary: true } => {
                    this.pending_undo_newline = Some(tab_id.clone());
                    this.execute_query(cx);
                }
                _ => {}
            }
        });
        self._subscriptions.push(editor_sub);

        let table_state = cx.new(|cx| DataTableState::new(cx));

        let sub = cx.subscribe(&table_state, Self::handle_cell_save);
        self._subscriptions.push(sub);
        let sub2 = cx.subscribe(&table_state, Self::handle_cell_double_click);
        self._subscriptions.push(sub2);
        let sub3 = cx.subscribe(&table_state, Self::handle_fk_data_request);
        self._subscriptions.push(sub3);

        let tab = QueryTab {
            id: id.clone(),
            name: format!("{}.{}", schema, table),
            database,
            editor,
            table_state,
            table_context: None,
            result: None,
            error: None,
            is_loading: false,
            last_export_message: None,
            table_structures: vec![],
            structure_loading: false,
            structure_expanded: HashMap::new(),
        };
        self.tabs.push(tab);
        self.active_tab_id = Some(id);
        cx.notify();

        self.execute_query(cx);
    }

    pub(crate) fn close_tab(&mut self, tab_id: &str, cx: &mut Context<Self>) {
        let closed_index = self.tabs.iter().position(|t| t.id == tab_id);
        self.tabs.retain(|t| t.id != tab_id);
        if self.active_tab_id.as_deref() == Some(tab_id) {
            self.active_tab_id = closed_index
                .and_then(|i| {
                    if i > 0 {
                        self.tabs.get(i - 1)
                    } else {
                        self.tabs.first()
                    }
                })
                .map(|t| t.id.clone());
        }
        cx.notify();
    }

    fn handle_editor_change(
        this: &mut Self,
        tab_id: &str,
        editor_entity: Entity<InputState>,
        cx: &mut Context<Self>,
    ) {
        let Some(tab) = this.tabs.iter().find(|t| t.id == tab_id) else {
            return;
        };
        if tab.editor != editor_entity {
            return;
        }

        let text = editor_entity.read(cx).value().to_string();
        if text.is_empty() {
            return;
        }

        let last_char = text.chars().last().unwrap_or(' ');
        if let Some((start, end, replacement)) = maybe_capitalize_last_word(&text, last_char) {
            this.pending_capitalization = Some((tab_id.to_string(), start, end, replacement));
            cx.notify();
        }
    }
}
