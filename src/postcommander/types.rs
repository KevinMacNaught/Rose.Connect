use crate::components::DataTableState;
use crate::postcommander::database::QueryResult;
use gpui::{Entity, SharedString};
use gpui_component::input::InputState;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct TableContext {
    pub schema: String,
    pub table: String,
    pub primary_keys: Vec<String>,
}

impl TableContext {
    #[allow(dead_code)]
    pub fn is_editable(&self) -> bool {
        !self.primary_keys.is_empty()
    }
}

#[derive(Clone)]
pub struct CellEditState {
    pub row_index: usize,
    pub col_index: usize,
    pub column_name: SharedString,
    pub original_value: SharedString,
    pub editor: Option<Entity<InputState>>,
    pub is_saving: bool,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct QueryTab {
    pub id: String,
    pub name: String,
    pub database: String,
    pub editor: Entity<InputState>,
    pub table_state: Entity<DataTableState>,
    pub table_context: Option<TableContext>,
    pub result: Option<QueryResult>,
    pub error: Option<String>,
    pub is_loading: bool,
    pub last_export_message: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

#[derive(Clone, Default)]
pub struct SchemaObjects {
    pub tables: Vec<String>,
    pub views: Vec<String>,
}

pub type SchemaMap = HashMap<String, SchemaObjects>;
