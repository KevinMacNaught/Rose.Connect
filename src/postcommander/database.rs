use crate::postcommander::types::{ForeignKeyInfo, ForeignKeyRef, TableColumn, TableStructureInfo};
use anyhow::Result;
use gpui::SharedString;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{Column, Row, TypeInfo, ValueRef};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    #[allow(dead_code)]
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl ConnectionConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Debug, Clone)]
pub struct QueryColumn {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone)]
pub enum CellValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
}

impl CellValue {
    pub fn display(&self) -> String {
        match self {
            CellValue::Null => "NULL".to_string(),
            CellValue::Bool(b) => b.to_string(),
            CellValue::Int(i) => i.to_string(),
            CellValue::Float(f) => f.to_string(),
            CellValue::Text(s) => s.clone(),
            CellValue::Bytes(b) => format!("[{} bytes]", b.len()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<QueryColumn>,
    pub rows: Arc<Vec<Vec<SharedString>>>,
    #[allow(dead_code)]
    pub rows_affected: u64,
    pub execution_time_ms: u64,
}

pub enum DatabaseCommand {
    Connect {
        config: ConnectionConfig,
        response: tokio::sync::oneshot::Sender<Result<()>>,
    },
    #[allow(dead_code)]
    Disconnect {
        response: tokio::sync::oneshot::Sender<Result<()>>,
    },
    Execute {
        sql: String,
        response: tokio::sync::oneshot::Sender<Result<QueryResult>>,
    },
    FetchPrimaryKeys {
        schema: String,
        table: String,
        response: tokio::sync::oneshot::Sender<Result<Vec<String>>>,
    },
    FetchForeignKeys {
        schema: String,
        table: String,
        response: tokio::sync::oneshot::Sender<Result<Vec<ForeignKeyInfo>>>,
    },
    FetchFkReferencedRow {
        referenced_schema: String,
        referenced_table: String,
        referenced_column: String,
        value: String,
        response: tokio::sync::oneshot::Sender<Result<Vec<(String, String)>>>,
    },
    FetchTableStructure {
        schema: String,
        table: String,
        response: tokio::sync::oneshot::Sender<Result<TableStructureInfo>>,
    },
}

pub struct DatabaseManager {
    #[allow(dead_code)]
    runtime: Arc<Runtime>,
    command_tx: mpsc::UnboundedSender<DatabaseCommand>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime"),
        );

        let (command_tx, mut command_rx) = mpsc::unbounded_channel::<DatabaseCommand>();

        let rt = runtime.clone();
        std::thread::spawn(move || {
            rt.block_on(async move {
                let mut pool: Option<PgPool> = None;

                while let Some(cmd) = command_rx.recv().await {
                    match cmd {
                        DatabaseCommand::Connect { config, response } => {
                            let result = PgPool::connect(&config.connection_string()).await;
                            match result {
                                Ok(p) => {
                                    pool = Some(p);
                                    let _ = response.send(Ok(()));
                                }
                                Err(e) => {
                                    let _ = response.send(Err(anyhow::anyhow!("{}", e)));
                                }
                            }
                        }
                        DatabaseCommand::Disconnect { response } => {
                            if let Some(p) = pool.take() {
                                p.close().await;
                            }
                            let _ = response.send(Ok(()));
                        }
                        DatabaseCommand::Execute { sql, response } => {
                            if let Some(ref p) = pool {
                                let start = std::time::Instant::now();
                                let result = execute_query(p, &sql).await;
                                let elapsed = start.elapsed().as_millis() as u64;
                                match result {
                                    Ok(mut qr) => {
                                        qr.execution_time_ms = elapsed;
                                        let _ = response.send(Ok(qr));
                                    }
                                    Err(e) => {
                                        let _ = response.send(Err(e));
                                    }
                                }
                            } else {
                                let _ = response.send(Err(anyhow::anyhow!("Not connected")));
                            }
                        }
                        DatabaseCommand::FetchPrimaryKeys { schema, table, response } => {
                            if let Some(ref p) = pool {
                                let result = fetch_primary_keys(p, &schema, &table).await;
                                let _ = response.send(result);
                            } else {
                                let _ = response.send(Err(anyhow::anyhow!("Not connected")));
                            }
                        }
                        DatabaseCommand::FetchForeignKeys { schema, table, response } => {
                            if let Some(ref p) = pool {
                                let result = fetch_foreign_keys(p, &schema, &table).await;
                                let _ = response.send(result);
                            } else {
                                let _ = response.send(Err(anyhow::anyhow!("Not connected")));
                            }
                        }
                        DatabaseCommand::FetchFkReferencedRow {
                            referenced_schema,
                            referenced_table,
                            referenced_column,
                            value,
                            response,
                        } => {
                            if let Some(ref p) = pool {
                                let result = fetch_fk_referenced_row(
                                    p,
                                    &referenced_schema,
                                    &referenced_table,
                                    &referenced_column,
                                    &value,
                                )
                                .await;
                                let _ = response.send(result);
                            } else {
                                let _ = response.send(Err(anyhow::anyhow!("Not connected")));
                            }
                        }
                        DatabaseCommand::FetchTableStructure { schema, table, response } => {
                            if let Some(ref p) = pool {
                                let result = fetch_table_structure(p, &schema, &table).await;
                                let _ = response.send(result);
                            } else {
                                let _ = response.send(Err(anyhow::anyhow!("Not connected")));
                            }
                        }
                    }
                }
            });
        });

        Self {
            runtime,
            command_tx,
        }
    }

    pub fn connect(
        &self,
        config: ConnectionConfig,
    ) -> tokio::sync::oneshot::Receiver<Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx.send(DatabaseCommand::Connect {
            config,
            response: tx,
        });
        rx
    }

    #[allow(dead_code)]
    pub fn disconnect(&self) -> tokio::sync::oneshot::Receiver<Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx.send(DatabaseCommand::Disconnect { response: tx });
        rx
    }

    pub fn execute(&self, sql: String) -> tokio::sync::oneshot::Receiver<Result<QueryResult>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx.send(DatabaseCommand::Execute { sql, response: tx });
        rx
    }

    pub fn fetch_primary_keys(
        &self,
        schema: String,
        table: String,
    ) -> tokio::sync::oneshot::Receiver<Result<Vec<String>>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx.send(DatabaseCommand::FetchPrimaryKeys {
            schema,
            table,
            response: tx,
        });
        rx
    }

    pub fn fetch_foreign_keys(
        &self,
        schema: String,
        table: String,
    ) -> tokio::sync::oneshot::Receiver<Result<Vec<ForeignKeyInfo>>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx.send(DatabaseCommand::FetchForeignKeys {
            schema,
            table,
            response: tx,
        });
        rx
    }

    pub fn fetch_fk_referenced_row(
        &self,
        referenced_schema: String,
        referenced_table: String,
        referenced_column: String,
        value: String,
    ) -> tokio::sync::oneshot::Receiver<Result<Vec<(String, String)>>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx.send(DatabaseCommand::FetchFkReferencedRow {
            referenced_schema,
            referenced_table,
            referenced_column,
            value,
            response: tx,
        });
        rx
    }

    pub fn fetch_table_structure(
        &self,
        schema: String,
        table: String,
    ) -> tokio::sync::oneshot::Receiver<Result<TableStructureInfo>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx.send(DatabaseCommand::FetchTableStructure {
            schema,
            table,
            response: tx,
        });
        rx
    }
}

async fn execute_query(pool: &PgPool, sql: &str) -> Result<QueryResult> {
    let rows: Vec<PgRow> = sqlx::query(sql).fetch_all(pool).await?;

    if rows.is_empty() {
        return Ok(QueryResult {
            columns: vec![],
            rows: Arc::new(vec![]),
            rows_affected: 0,
            execution_time_ms: 0,
        });
    }

    let columns: Vec<QueryColumn> = rows[0]
        .columns()
        .iter()
        .map(|c| QueryColumn {
            name: c.name().to_string(),
            type_name: c.type_info().name().to_string(),
        })
        .collect();

    let result_rows: Vec<Vec<SharedString>> = rows
        .iter()
        .map(|row| {
            row.columns()
                .iter()
                .enumerate()
                .map(|(i, col)| {
                    let cell = extract_cell_value(row, i, col.type_info().name());
                    SharedString::from(cell.display())
                })
                .collect()
        })
        .collect();

    let row_count = result_rows.len() as u64;

    Ok(QueryResult {
        columns,
        rows: Arc::new(result_rows),
        rows_affected: row_count,
        execution_time_ms: 0,
    })
}

async fn fetch_primary_keys(pool: &PgPool, schema: &str, table: &str) -> Result<Vec<String>> {
    let sql = r#"
        SELECT kcu.column_name
        FROM information_schema.table_constraints tc
        JOIN information_schema.key_column_usage kcu
            ON tc.constraint_name = kcu.constraint_name
            AND tc.table_schema = kcu.table_schema
        WHERE tc.constraint_type = 'PRIMARY KEY'
            AND tc.table_schema = $1
            AND tc.table_name = $2
        ORDER BY kcu.ordinal_position
    "#;

    let rows: Vec<PgRow> = sqlx::query(sql)
        .bind(schema)
        .bind(table)
        .fetch_all(pool)
        .await?;

    let pk_columns: Vec<String> = rows
        .iter()
        .filter_map(|row| row.try_get::<String, _>("column_name").ok())
        .collect();

    Ok(pk_columns)
}

async fn fetch_foreign_keys(
    pool: &PgPool,
    schema: &str,
    table: &str,
) -> Result<Vec<ForeignKeyInfo>> {
    let sql = r#"
        SELECT
            kcu.column_name,
            ccu.table_schema AS referenced_schema,
            ccu.table_name AS referenced_table,
            ccu.column_name AS referenced_column
        FROM information_schema.table_constraints tc
        JOIN information_schema.key_column_usage kcu
            ON tc.constraint_name = kcu.constraint_name
            AND tc.table_schema = kcu.table_schema
        JOIN information_schema.constraint_column_usage ccu
            ON tc.constraint_name = ccu.constraint_name
        WHERE tc.constraint_type = 'FOREIGN KEY'
            AND tc.table_schema = $1
            AND tc.table_name = $2
    "#;

    let rows: Vec<PgRow> = sqlx::query(sql)
        .bind(schema)
        .bind(table)
        .fetch_all(pool)
        .await?;

    let fk_info: Vec<ForeignKeyInfo> = rows
        .iter()
        .filter_map(|row| {
            Some(ForeignKeyInfo {
                column_name: row.try_get("column_name").ok()?,
                referenced_schema: row.try_get("referenced_schema").ok()?,
                referenced_table: row.try_get("referenced_table").ok()?,
                referenced_column: row.try_get("referenced_column").ok()?,
            })
        })
        .collect();

    Ok(fk_info)
}

async fn fetch_fk_referenced_row(
    pool: &PgPool,
    schema: &str,
    table: &str,
    column: &str,
    value: &str,
) -> Result<Vec<(String, String)>> {
    let sql = format!(
        r#"SELECT * FROM "{}"."{}" WHERE "{}"::text = $1 LIMIT 1"#,
        schema, table, column
    );

    let rows: Vec<PgRow> = sqlx::query(&sql).bind(value).fetch_all(pool).await?;

    if rows.is_empty() {
        return Ok(vec![]);
    }

    let row = &rows[0];
    let result: Vec<(String, String)> = row
        .columns()
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let col_name = col.name().to_string();
            let cell = extract_cell_value(row, i, col.type_info().name());
            (col_name, cell.display())
        })
        .collect();

    Ok(result)
}

fn extract_cell_value(row: &PgRow, index: usize, type_name: &str) -> CellValue {
    if row.try_get_raw(index).map(|v| v.is_null()).unwrap_or(true) {
        return CellValue::Null;
    }

    match type_name {
        "BOOL" => row
            .try_get::<bool, _>(index)
            .map(CellValue::Bool)
            .unwrap_or(CellValue::Null),
        "INT2" | "INT4" | "INT8" => row
            .try_get::<i64, _>(index)
            .map(CellValue::Int)
            .unwrap_or_else(|_| {
                row.try_get::<i32, _>(index)
                    .map(|v| CellValue::Int(v as i64))
                    .unwrap_or(CellValue::Null)
            }),
        "FLOAT4" | "FLOAT8" | "NUMERIC" => row
            .try_get::<f64, _>(index)
            .map(CellValue::Float)
            .unwrap_or(CellValue::Null),
        "BYTEA" => row
            .try_get::<Vec<u8>, _>(index)
            .map(CellValue::Bytes)
            .unwrap_or(CellValue::Null),
        _ => row
            .try_get::<String, _>(index)
            .map(CellValue::Text)
            .unwrap_or(CellValue::Null),
    }
}

async fn fetch_table_structure(
    pool: &PgPool,
    schema: &str,
    table: &str,
) -> Result<TableStructureInfo> {
    let sql = r#"
        SELECT
            c.column_name,
            c.data_type,
            c.is_nullable,
            c.column_default,
            CASE WHEN pk.column_name IS NOT NULL THEN true ELSE false END as is_primary_key,
            CASE WHEN fk.column_name IS NOT NULL THEN true ELSE false END as is_foreign_key,
            fk.foreign_table_schema,
            fk.foreign_table_name,
            fk.foreign_column_name
        FROM information_schema.columns c
        LEFT JOIN (
            SELECT ku.column_name
            FROM information_schema.table_constraints tc
            JOIN information_schema.key_column_usage ku ON tc.constraint_name = ku.constraint_name
                AND tc.table_schema = ku.table_schema
            WHERE tc.table_schema = $1 AND tc.table_name = $2 AND tc.constraint_type = 'PRIMARY KEY'
        ) pk ON c.column_name = pk.column_name
        LEFT JOIN (
            SELECT
                kcu.column_name,
                ccu.table_schema as foreign_table_schema,
                ccu.table_name as foreign_table_name,
                ccu.column_name as foreign_column_name
            FROM information_schema.table_constraints tc
            JOIN information_schema.key_column_usage kcu ON tc.constraint_name = kcu.constraint_name
                AND tc.table_schema = kcu.table_schema
            JOIN information_schema.constraint_column_usage ccu ON tc.constraint_name = ccu.constraint_name
            WHERE tc.table_schema = $1 AND tc.table_name = $2 AND tc.constraint_type = 'FOREIGN KEY'
        ) fk ON c.column_name = fk.column_name
        WHERE c.table_schema = $1 AND c.table_name = $2
        ORDER BY c.ordinal_position
    "#;

    let rows: Vec<PgRow> = sqlx::query(sql)
        .bind(schema)
        .bind(table)
        .fetch_all(pool)
        .await?;

    let columns: Vec<TableColumn> = rows
        .iter()
        .filter_map(|row| {
            let name: String = row.try_get("column_name").ok()?;
            let data_type: String = row.try_get("data_type").ok()?;
            let is_nullable_str: String = row.try_get("is_nullable").ok()?;
            let nullable = is_nullable_str == "YES";
            let default_value: Option<String> = row.try_get("column_default").ok();
            let is_primary_key: bool = row.try_get("is_primary_key").ok()?;
            let is_foreign_key: bool = row.try_get("is_foreign_key").ok()?;

            let references = if is_foreign_key {
                let fk_schema: Option<String> = row.try_get("foreign_table_schema").ok();
                let fk_table: Option<String> = row.try_get("foreign_table_name").ok();
                let fk_column: Option<String> = row.try_get("foreign_column_name").ok();
                match (fk_schema, fk_table, fk_column) {
                    (Some(s), Some(t), Some(c)) => Some(ForeignKeyRef {
                        schema: s,
                        table: t,
                        column: c,
                    }),
                    _ => None,
                }
            } else {
                None
            };

            Some(TableColumn {
                name,
                data_type,
                nullable,
                default_value,
                is_primary_key,
                is_foreign_key,
                references,
            })
        })
        .collect();

    Ok(TableStructureInfo {
        schema: schema.to_string(),
        table: table.to_string(),
        columns,
    })
}
