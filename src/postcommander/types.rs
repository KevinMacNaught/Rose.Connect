use crate::components::DataTableState;
use crate::postcommander::database::QueryResult;
use gpui::{Entity, SharedString, Task};
use gpui_component::input::InputState;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SidebarTab {
    #[default]
    Schema,
    History,
    Saved,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TabId(u64);

impl TabId {
    pub fn new() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

impl std::fmt::Display for TabId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct ForeignKeyInfo {
    pub column_name: String,
    pub referenced_schema: String,
    pub referenced_table: String,
    pub referenced_column: String,
}

#[derive(Clone, Debug)]
pub struct ForeignKeyRef {
    pub schema: String,
    pub table: String,
    pub column: String,
}

#[derive(Clone, Debug)]
pub struct TableColumn {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub is_primary_key: bool,
    pub is_foreign_key: bool,
    pub references: Option<ForeignKeyRef>,
}

#[derive(Clone, Debug)]
pub struct TableStructureInfo {
    pub schema: String,
    pub table: String,
    pub columns: Vec<TableColumn>,
}

#[derive(Clone, Debug, Default)]
pub struct TableContext {
    pub schema: String,
    pub table: String,
    pub primary_keys: Vec<String>,
    pub foreign_keys: Arc<HashMap<String, ForeignKeyInfo>>,
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

pub struct QueryTab {
    pub id: TabId,
    pub name: String,
    pub database: String,
    pub editor: Entity<InputState>,
    pub table_state: Entity<DataTableState>,
    pub table_context: Option<TableContext>,
    pub result: Option<QueryResult>,
    pub error: Option<String>,
    pub is_loading: bool,
    pub query_start_time: Option<Instant>,
    pub query_task: Option<Task<()>>,
    pub last_export_message: Option<String>,
    pub table_structures: Vec<TableStructureInfo>,
    pub structure_loading: bool,
    pub structure_expanded: HashMap<String, bool>,
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
