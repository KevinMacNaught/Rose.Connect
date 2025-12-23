use crate::postcommander::page::PostCommanderPage;
use gpui::*;

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
