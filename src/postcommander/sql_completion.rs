use anyhow::Result;
use gpui::{Context, Task};
use gpui_component::input::{CompletionProvider, InputState};
use gpui_component::Rope;
use lsp_types::{
    CompletionContext, CompletionItem, CompletionItemKind, CompletionResponse,
};
use std::cell::RefCell;
use std::rc::Rc;

use super::types::{SchemaMap, TableStructureInfo};

pub struct SqlCompletionProvider {
    schemas: Rc<RefCell<SchemaMap>>,
    table_structures: Rc<RefCell<Vec<TableStructureInfo>>>,
}

impl SqlCompletionProvider {
    pub fn new() -> Self {
        Self {
            schemas: Rc::new(RefCell::new(SchemaMap::new())),
            table_structures: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn schemas_ref(&self) -> Rc<RefCell<SchemaMap>> {
        self.schemas.clone()
    }

    pub fn table_structures_ref(&self) -> Rc<RefCell<Vec<TableStructureInfo>>> {
        self.table_structures.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SqlContext {
    AfterSelect,
    AfterFrom,
    AfterJoin,
    AfterWhere,
    AfterDot,
    AfterComma,
    Unknown,
}

struct SqlContextInfo {
    context: SqlContext,
    prefix: Option<String>,
    filter: String,
}

fn detect_sql_context(text: &str, offset: usize) -> SqlContextInfo {
    let before_cursor = &text[..offset.min(text.len())];
    let before_upper = before_cursor.to_uppercase();

    let current_word_start = before_cursor
        .rfind(|c: char| c.is_whitespace() || c == ',' || c == '(' || c == ')')
        .map(|i| i + 1)
        .unwrap_or(0);
    let current_word = &before_cursor[current_word_start..];

    if before_cursor.ends_with('.') {
        return SqlContextInfo {
            context: SqlContext::AfterDot,
            prefix: None,
            filter: String::new(),
        };
    }

    if current_word.contains('.') {
        let parts: Vec<&str> = current_word.split('.').collect();
        if parts.len() >= 2 {
            let prefix = parts[0].trim_matches('"').to_string();
            let filter = parts[1].trim_matches('"').to_string();
            return SqlContextInfo {
                context: SqlContext::AfterDot,
                prefix: Some(prefix),
                filter,
            };
        }
    }

    if before_cursor.ends_with(',') || before_cursor.ends_with(", ") {
        if before_upper.rfind("SELECT").is_some() {
            if let Some(from_pos) = before_upper.rfind("FROM") {
                if let Some(select_pos) = before_upper.rfind("SELECT") {
                    if select_pos > from_pos {
                        return SqlContextInfo {
                            context: SqlContext::AfterComma,
                            prefix: None,
                            filter: String::new(),
                        };
                    }
                }
            } else {
                return SqlContextInfo {
                    context: SqlContext::AfterComma,
                    prefix: None,
                    filter: String::new(),
                };
            }
        }
    }

    let words: Vec<&str> = before_cursor.split_whitespace().collect();
    let filter = words.last().map(|s| s.to_string()).unwrap_or_default();

    if let Some(last_word) = words.last() {
        let upper = last_word.to_uppercase();
        match upper.as_str() {
            "SELECT" => return SqlContextInfo { context: SqlContext::AfterSelect, prefix: None, filter: String::new() },
            "FROM" => return SqlContextInfo { context: SqlContext::AfterFrom, prefix: None, filter: String::new() },
            "JOIN" | "INNER" | "LEFT" | "RIGHT" | "OUTER" | "CROSS" => {
                return SqlContextInfo { context: SqlContext::AfterJoin, prefix: None, filter: String::new() }
            }
            "WHERE" | "AND" | "OR" => return SqlContextInfo { context: SqlContext::AfterWhere, prefix: None, filter: String::new() },
            _ => {}
        }
    }

    if words.len() >= 2 {
        let second_last = words[words.len() - 2].to_uppercase();
        match second_last.as_str() {
            "SELECT" => return SqlContextInfo { context: SqlContext::AfterSelect, prefix: None, filter },
            "FROM" => return SqlContextInfo { context: SqlContext::AfterFrom, prefix: None, filter },
            "JOIN" => return SqlContextInfo { context: SqlContext::AfterJoin, prefix: None, filter },
            "WHERE" | "AND" | "OR" => return SqlContextInfo { context: SqlContext::AfterWhere, prefix: None, filter },
            _ => {}
        }
    }

    SqlContextInfo { context: SqlContext::Unknown, prefix: None, filter }
}

fn get_identifier_before_dot(text: &str, offset: usize) -> Option<String> {
    let before_cursor = &text[..offset.min(text.len())];
    if !before_cursor.ends_with('.') {
        return None;
    }

    let without_dot = &before_cursor[..before_cursor.len() - 1];
    let words: Vec<&str> = without_dot.split_whitespace().collect();
    words.last().map(|s| s.trim_matches('"').to_string())
}

fn is_schema_name(name: &str, schemas: &SchemaMap) -> bool {
    schemas.contains_key(name) || schemas.keys().any(|k| k.eq_ignore_ascii_case(name))
}

const SQL_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "AND", "OR", "NOT", "IN", "LIKE", "ILIKE",
    "BETWEEN", "IS", "NULL", "TRUE", "FALSE", "ORDER", "BY", "ASC", "DESC",
    "LIMIT", "OFFSET", "GROUP", "HAVING", "JOIN", "INNER", "LEFT", "RIGHT",
    "OUTER", "CROSS", "ON", "AS", "DISTINCT", "ALL", "UNION", "INTERSECT",
    "EXCEPT", "EXISTS", "CASE", "WHEN", "THEN", "ELSE", "END", "CAST",
    "INSERT", "INTO", "VALUES", "UPDATE", "SET", "DELETE", "CREATE", "ALTER",
    "DROP", "TABLE", "INDEX", "VIEW", "SCHEMA", "DATABASE", "TRUNCATE",
    "CONSTRAINT", "PRIMARY", "KEY", "FOREIGN", "REFERENCES", "UNIQUE",
    "DEFAULT", "CHECK", "CASCADE", "NULLS", "FIRST", "LAST",
];

const SQL_FUNCTIONS: &[&str] = &[
    "COUNT", "SUM", "AVG", "MIN", "MAX", "COALESCE", "NULLIF", "GREATEST",
    "LEAST", "NOW", "CURRENT_TIMESTAMP", "CURRENT_DATE", "CURRENT_TIME",
    "DATE_TRUNC", "DATE_PART", "EXTRACT", "AGE", "INTERVAL",
    "UPPER", "LOWER", "TRIM", "LTRIM", "RTRIM", "LENGTH", "SUBSTRING",
    "CONCAT", "REPLACE", "SPLIT_PART", "REGEXP_REPLACE", "REGEXP_MATCHES",
    "ROUND", "FLOOR", "CEIL", "ABS", "POWER", "SQRT", "MOD",
    "ARRAY_AGG", "STRING_AGG", "JSON_AGG", "JSONB_AGG",
    "ROW_NUMBER", "RANK", "DENSE_RANK", "LAG", "LEAD", "FIRST_VALUE",
    "GEN_RANDOM_UUID", "RANDOM",
];

impl CompletionProvider for SqlCompletionProvider {
    fn completions(
        &self,
        text: &Rope,
        offset: usize,
        _trigger: CompletionContext,
        _window: &mut gpui::Window,
        _cx: &mut Context<InputState>,
    ) -> Task<Result<CompletionResponse>> {
        let text_str = text.to_string();
        let ctx_info = detect_sql_context(&text_str, offset);
        let schemas = self.schemas.borrow();
        let structures = self.table_structures.borrow();
        let filter = ctx_info.filter.to_lowercase();

        let matches = |label: &str| -> bool {
            if filter.is_empty() {
                return true;
            }
            label.to_lowercase().starts_with(&filter) || label.to_lowercase().contains(&filter)
        };

        Task::ready(Ok(CompletionResponse::Array(match ctx_info.context {
            SqlContext::AfterSelect | SqlContext::AfterComma => {
                let mut items = Vec::new();

                if matches("*") {
                    items.push(CompletionItem {
                        label: "*".to_string(),
                        kind: Some(CompletionItemKind::OPERATOR),
                        detail: Some("All columns".to_string()),
                        sort_text: Some("0_*".to_string()),
                        ..Default::default()
                    });
                }

                for structure in structures.iter() {
                    for col in &structure.columns {
                        if !matches(&col.name) {
                            continue;
                        }
                        let priority = if col.is_primary_key {
                            "1"
                        } else if col.is_foreign_key {
                            "2"
                        } else {
                            "3"
                        };

                        let mut detail = col.data_type.clone();
                        if col.is_primary_key {
                            detail = format!("PK {}", detail);
                        }
                        if col.is_foreign_key {
                            if let Some(ref fk_ref) = col.references {
                                detail = format!("FK → {}.{} ({})", fk_ref.table, fk_ref.column, detail);
                            }
                        }

                        items.push(CompletionItem {
                            label: col.name.clone(),
                            kind: Some(CompletionItemKind::FIELD),
                            detail: Some(detail),
                            sort_text: Some(format!("{}_{}", priority, col.name.to_lowercase())),
                            ..Default::default()
                        });
                    }
                }

                for func in SQL_FUNCTIONS {
                    if !matches(func) {
                        continue;
                    }
                    items.push(CompletionItem {
                        label: format!("{}()", func),
                        insert_text: Some(format!("{}()", func)),
                        kind: Some(CompletionItemKind::FUNCTION),
                        detail: Some("Function".to_string()),
                        sort_text: Some(format!("5_{}", func.to_lowercase())),
                        ..Default::default()
                    });
                }

                items
            }

            SqlContext::AfterFrom | SqlContext::AfterJoin => {
                let mut items = Vec::new();

                for (schema_name, objects) in schemas.iter() {
                    for table in &objects.tables {
                        let qualified = if schema_name == "public" {
                            table.clone()
                        } else {
                            format!("\"{}\".\"{}\"", schema_name, table)
                        };

                        if matches(&qualified) || matches(table) {
                            items.push(CompletionItem {
                                label: qualified.clone(),
                                kind: Some(CompletionItemKind::CLASS),
                                detail: Some(format!("Table in {}", schema_name)),
                                sort_text: Some(format!("1_{}", table.to_lowercase())),
                                ..Default::default()
                            });
                        }
                    }

                    for view in &objects.views {
                        let qualified = if schema_name == "public" {
                            view.clone()
                        } else {
                            format!("\"{}\".\"{}\"", schema_name, view)
                        };

                        if matches(&qualified) || matches(view) {
                            items.push(CompletionItem {
                                label: qualified.clone(),
                                kind: Some(CompletionItemKind::INTERFACE),
                                detail: Some(format!("View in {}", schema_name)),
                                sort_text: Some(format!("2_{}", view.to_lowercase())),
                                ..Default::default()
                            });
                        }
                    }
                }

                items
            }

            SqlContext::AfterDot => {
                let mut items = Vec::new();

                let name_before_dot = ctx_info.prefix.or_else(|| get_identifier_before_dot(&text_str, offset));
                if let Some(name_before_dot) = name_before_dot {
                    if is_schema_name(&name_before_dot, &schemas) {
                        let schema_key = schemas
                            .keys()
                            .find(|k| k.eq_ignore_ascii_case(&name_before_dot))
                            .cloned();

                        if let Some(schema_name) = schema_key {
                            if let Some(objects) = schemas.get(&schema_name) {
                                for table in &objects.tables {
                                    if matches(table) {
                                        items.push(CompletionItem {
                                            label: table.clone(),
                                            kind: Some(CompletionItemKind::CLASS),
                                            detail: Some(format!("Table in {}", schema_name)),
                                            sort_text: Some(format!("1_{}", table.to_lowercase())),
                                            ..Default::default()
                                        });
                                    }
                                }

                                for view in &objects.views {
                                    if matches(view) {
                                        items.push(CompletionItem {
                                            label: view.clone(),
                                            kind: Some(CompletionItemKind::INTERFACE),
                                            detail: Some(format!("View in {}", schema_name)),
                                            sort_text: Some(format!("2_{}", view.to_lowercase())),
                                            ..Default::default()
                                        });
                                    }
                                }
                            }
                        }
                    } else {
                        for structure in structures.iter() {
                            if structure.table.eq_ignore_ascii_case(&name_before_dot) {
                                for col in &structure.columns {
                                    if !matches(&col.name) {
                                        continue;
                                    }
                                    let priority = if col.is_primary_key {
                                        "0"
                                    } else if col.is_foreign_key {
                                        "1"
                                    } else {
                                        "2"
                                    };

                                    let mut detail = col.data_type.clone();
                                    if col.is_primary_key {
                                        detail = format!("PK {}", detail);
                                    }
                                    if col.is_foreign_key {
                                        if let Some(ref fk_ref) = col.references {
                                            detail = format!("FK → {}.{} ({})", fk_ref.table, fk_ref.column, detail);
                                        }
                                    }

                                    items.push(CompletionItem {
                                        label: col.name.clone(),
                                        kind: Some(CompletionItemKind::FIELD),
                                        detail: Some(detail),
                                        sort_text: Some(format!("{}_{}", priority, col.name.to_lowercase())),
                                        ..Default::default()
                                    });
                                }
                                break;
                            }
                        }
                    }
                }

                items
            }

            SqlContext::AfterWhere => {
                let mut items = Vec::new();

                for structure in structures.iter() {
                    for col in &structure.columns {
                        if !matches(&col.name) {
                            continue;
                        }
                        let priority = if col.is_primary_key {
                            "0"
                        } else if col.is_foreign_key {
                            "1"
                        } else {
                            "2"
                        };

                        items.push(CompletionItem {
                            label: col.name.clone(),
                            kind: Some(CompletionItemKind::FIELD),
                            detail: Some(col.data_type.clone()),
                            sort_text: Some(format!("{}_{}", priority, col.name.to_lowercase())),
                            ..Default::default()
                        });
                    }
                }

                let operators = ["=", "<>", "<", ">", "<=", ">=", "LIKE", "ILIKE", "IN", "BETWEEN", "IS NULL", "IS NOT NULL"];
                for op in operators {
                    if matches(op) {
                        items.push(CompletionItem {
                            label: op.to_string(),
                            kind: Some(CompletionItemKind::OPERATOR),
                            sort_text: Some(format!("9_{}", op)),
                            ..Default::default()
                        });
                    }
                }

                items
            }

            SqlContext::Unknown => {
                let mut items = Vec::new();

                for kw in SQL_KEYWORDS {
                    if matches(kw) {
                        items.push(CompletionItem {
                            label: kw.to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            sort_text: Some(format!("9_{}", kw.to_lowercase())),
                            ..Default::default()
                        });
                    }
                }

                items
            }
        })))
    }

    fn is_completion_trigger(
        &self,
        _offset: usize,
        new_text: &str,
        _cx: &mut Context<InputState>,
    ) -> bool {
        if new_text.is_empty() {
            return false;
        }
        let c = new_text.chars().next().unwrap();
        // Don't trigger on space - let it dismiss the menu and insert normally
        // This prevents the "SELECTFROM" bug where accepting completion eats spaces
        c == '.' || c == ',' || c == '(' || c.is_alphanumeric() || c == '_'
    }
}
