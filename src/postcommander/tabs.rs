use crate::components::DataTableState;
use crate::postcommander::page::PostCommanderPage;
use crate::postcommander::types::QueryTab;
use gpui::*;
use gpui_component::input::InputState;

impl PostCommanderPage {
    pub(crate) fn add_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = format!("tab-{}", self.tabs.len() + 1);
        let database = self.get_conn_database(cx);

        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("sql".to_string())
                .line_number(true)
                .soft_wrap(true)
                .default_value("SELECT * FROM ")
                .placeholder("Enter SQL query...")
        });

        editor.update(cx, |editor, cx| {
            editor.set_cursor_position(
                gpui_component::input::Position { line: 0, character: 14 },
                window,
                cx,
            );
        });

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
        };
        self.tabs.push(tab);
        self.active_tab_id = Some(id);
        cx.notify();
    }

    pub(crate) fn query_table(&mut self, schema: &str, table: &str, window: &mut Window, cx: &mut Context<Self>) {
        let id = format!("tab-{}", self.tabs.len() + 1);
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

        editor.update(cx, |editor, cx| {
            editor.set_cursor_position(
                gpui_component::input::Position { line: 0, character: cursor_pos },
                window,
                cx,
            );
        });

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
        };
        self.tabs.push(tab);
        self.active_tab_id = Some(id);
        cx.notify();

        self.execute_query(cx);
    }

    pub(crate) fn close_tab(&mut self, tab_id: &str, cx: &mut Context<Self>) {
        self.tabs.retain(|t| t.id != tab_id);
        if self.active_tab_id.as_deref() == Some(tab_id) {
            self.active_tab_id = self.tabs.last().map(|t| t.id.clone());
        }
        cx.notify();
    }
}
