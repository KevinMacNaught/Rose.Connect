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
