use crate::postcommander::page::PostCommanderPage;
use gpui::*;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

impl PostCommanderPage {
    pub(crate) fn export_to_csv(&self, cx: &mut Context<Self>) -> Option<String> {
        let tab = self.active_tab_id.as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))?;

        let result = tab.result.as_ref()?;
        let mut csv = String::new();

        let headers: Vec<String> = result.columns.iter().map(|c| escape_csv(&c.name)).collect();
        csv.push_str(&headers.join(","));
        csv.push('\n');

        let rows = tab.table_state.read(cx).rows();
        for row in rows.iter() {
            let cells: Vec<String> = row.iter().map(|c| escape_csv(c.as_ref())).collect();
            csv.push_str(&cells.join(","));
            csv.push('\n');
        }

        Some(csv)
    }

    pub(crate) fn export_to_json(&self, cx: &mut Context<Self>) -> Option<String> {
        let tab = self.active_tab_id.as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))?;

        let result = tab.result.as_ref()?;
        let column_names: Vec<&str> = result.columns.iter().map(|c| c.name.as_str()).collect();

        let rows = tab.table_state.read(cx).rows();
        let mut json_rows: Vec<String> = Vec::new();

        for row in rows.iter() {
            let mut obj_parts: Vec<String> = Vec::new();
            for (i, cell) in row.iter().enumerate() {
                let key = column_names.get(i).unwrap_or(&"");
                let value = if cell.as_ref() == "NULL" {
                    "null".to_string()
                } else {
                    format!("\"{}\"", escape_json(cell.as_ref()))
                };
                obj_parts.push(format!("\"{}\":{}", key, value));
            }
            json_rows.push(format!("{{{}}}", obj_parts.join(",")));
        }

        Some(format!("[\n  {}\n]", json_rows.join(",\n  ")))
    }

    pub(crate) fn export_to_markdown(&self, cx: &mut Context<Self>) -> Option<String> {
        let tab = self.active_tab_id.as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))?;

        let result = tab.result.as_ref()?;
        let mut md = String::new();

        let headers: Vec<&str> = result.columns.iter().map(|c| c.name.as_str()).collect();
        md.push_str("| ");
        md.push_str(&headers.join(" | "));
        md.push_str(" |\n");

        md.push_str("| ");
        md.push_str(&headers.iter().map(|_| "---").collect::<Vec<_>>().join(" | "));
        md.push_str(" |\n");

        let rows = tab.table_state.read(cx).rows();
        for row in rows.iter().take(100) {
            md.push_str("| ");
            let cells: Vec<String> = row.iter()
                .map(|c| escape_markdown(c.as_ref()))
                .collect();
            md.push_str(&cells.join(" | "));
            md.push_str(" |\n");
        }

        if rows.len() > 100 {
            md.push_str(&format!("\n_({} more rows...)_\n", rows.len() - 100));
        }

        Some(md)
    }

    pub(crate) fn copy_to_clipboard(&self, content: String, cx: &mut Context<Self>) {
        cx.write_to_clipboard(ClipboardItem::new_string(content));
    }

    pub(crate) fn copy_cell_value(value: &str, cx: &mut App) {
        cx.write_to_clipboard(ClipboardItem::new_string(value.to_string()));
    }

    pub(crate) fn copy_row_as_tsv(_columns: &[String], row: &[SharedString], cx: &mut App) {
        let tsv: Vec<String> = row.iter().map(|cell| {
            let s = cell.as_ref();
            if s.contains('\t') || s.contains('\n') || s.contains('\r') {
                format!("\"{}\"", s.replace('"', "\"\""))
            } else {
                s.to_string()
            }
        }).collect();
        cx.write_to_clipboard(ClipboardItem::new_string(tsv.join("\t")));
    }

    pub(crate) fn copy_row_as_json(columns: &[String], row: &[SharedString], cx: &mut App) {
        let mut obj_parts: Vec<String> = Vec::new();
        for (i, cell) in row.iter().enumerate() {
            let key = columns.get(i).map(|s| s.as_str()).unwrap_or("");
            let value = if cell.as_ref() == "NULL" {
                "null".to_string()
            } else {
                format!("\"{}\"", escape_json(cell.as_ref()))
            };
            obj_parts.push(format!("\"{}\":{}", key, value));
        }
        let json = format!("{{{}}}", obj_parts.join(","));
        cx.write_to_clipboard(ClipboardItem::new_string(json));
    }

    pub(crate) fn copy_row_as_insert(
        table_name: Option<&str>,
        columns: &[String],
        row: &[SharedString],
        cx: &mut App,
    ) {
        let table = table_name.unwrap_or("table_name");
        let col_list = columns.join(", ");
        let values: Vec<String> = row.iter().map(|cell| {
            let s = cell.as_ref();
            if s == "NULL" {
                "NULL".to_string()
            } else if s.parse::<i64>().is_ok() || s.parse::<f64>().is_ok() {
                s.to_string()
            } else {
                format!("'{}'", s.replace('\'', "''"))
            }
        }).collect();
        let sql = format!("INSERT INTO {} ({}) VALUES ({});", table, col_list, values.join(", "));
        cx.write_to_clipboard(ClipboardItem::new_string(sql));
    }
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn escape_markdown(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', "<br>")
}

fn generate_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "export".to_string())
}

fn sanitize_filename(s: &str) -> String {
    s.replace('.', "_")
        .replace(' ', "_")
        .replace('/', "_")
        .replace('\\', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

fn generate_filename(tab_name: &str, extension: &str) -> String {
    let base = sanitize_filename(tab_name);
    let base = if base.is_empty() { "export".to_string() } else { base };
    format!("{}_{}.{}", base, generate_timestamp(), extension)
}

fn save_to_file(content: &str, filename: &str) -> Result<PathBuf, String> {
    let downloads = dirs::download_dir()
        .ok_or_else(|| "Could not find Downloads folder".to_string())?;

    let path = downloads.join(filename);

    fs::write(&path, content)
        .map_err(|e| format!("Failed to save: {}", e))?;

    Ok(path)
}

impl PostCommanderPage {
    pub(crate) fn save_csv_to_file(&mut self, cx: &mut Context<Self>) -> Result<String, String> {
        let csv_content = self.export_to_csv(cx)
            .ok_or_else(|| "No data to export".to_string())?;

        let tab_name = self.active_tab_id.as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))
            .map(|t| t.name.as_str())
            .unwrap_or("export");

        let filename = generate_filename(tab_name, "csv");
        let path = save_to_file(&csv_content, &filename)?;

        let message = format!("Saved to {}", path.display());
        self.set_export_message(&message, cx);
        Ok(message)
    }

    pub(crate) fn save_json_to_file(&mut self, cx: &mut Context<Self>) -> Result<String, String> {
        let json_content = self.export_to_json(cx)
            .ok_or_else(|| "No data to export".to_string())?;

        let tab_name = self.active_tab_id.as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))
            .map(|t| t.name.as_str())
            .unwrap_or("export");

        let filename = generate_filename(tab_name, "json");
        let path = save_to_file(&json_content, &filename)?;

        let message = format!("Saved to {}", path.display());
        self.set_export_message(&message, cx);
        Ok(message)
    }

    pub(crate) fn save_markdown_to_file(&mut self, cx: &mut Context<Self>) -> Result<String, String> {
        let md_content = self.export_to_markdown(cx)
            .ok_or_else(|| "No data to export".to_string())?;

        let tab_name = self.active_tab_id.as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))
            .map(|t| t.name.as_str())
            .unwrap_or("export");

        let filename = generate_filename(tab_name, "md");
        let path = save_to_file(&md_content, &filename)?;

        let message = format!("Saved to {}", path.display());
        self.set_export_message(&message, cx);
        Ok(message)
    }

    fn set_export_message(&mut self, message: &str, cx: &mut Context<Self>) {
        if let Some(tab_id) = &self.active_tab_id {
            if let Some(tab) = self.tabs.iter_mut().find(|t| &t.id == tab_id) {
                tab.last_export_message = Some(message.to_string());
                cx.notify();
            }
        }
    }
}
