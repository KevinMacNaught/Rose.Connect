use crate::components::TextInputElement;
use crate::icons::icon_sm;
use crate::postcommander::PostCommanderPage;
use crate::settings::{AppSettings, QueryHistoryEntry, QueryHistoryStatus};
use crate::theme::ActiveTheme;
use chrono::{DateTime, Local};
use gpui::prelude::FluentBuilder;
use gpui::*;

impl PostCommanderPage {
    pub fn render_query_history_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let panel_background = colors.panel_background;
        let surface = colors.surface;
        let border_variant = colors.border_variant;
        let text_muted = colors.text_muted;
        let text_placeholder = colors.text_placeholder;
        let text = colors.text;
        let element_hover = colors.element_hover;

        let settings = AppSettings::get_global(cx);
        let history = settings
            .postcommander()
            .query_history
            .as_ref()
            .map(|h| h.entries.clone())
            .unwrap_or_default();

        let filtered_entries = if self.history_search_filter.is_empty() {
            history.clone()
        } else {
            let filter_lower = self.history_search_filter.to_lowercase();
            history
                .iter()
                .filter(|e| e.sql.to_lowercase().contains(&filter_lower))
                .cloned()
                .collect()
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .flex_1()
            .min_h_0()
            .bg(rgb(panel_background))
            .child(self.render_history_header(surface, border_variant, text_muted, text_placeholder, text, element_hover, cx))
            .child(
                div()
                    .id("query-history-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .p_2()
                    .children(filtered_entries.iter().enumerate().map(|(index, entry)| {
                        self.render_history_entry(entry, index, text, text_muted, element_hover, cx)
                    }))
                    .when(filtered_entries.is_empty(), |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .h_full()
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(text_muted))
                                        .child(if self.history_search_filter.is_empty() {
                                            "No query history"
                                        } else {
                                            "No matching queries"
                                        }),
                                ),
                        )
                    }),
            )
    }

    fn render_history_header(
        &self,
        surface: u32,
        border_variant: u32,
        text_muted: u32,
        text_placeholder: u32,
        text: u32,
        element_hover: u32,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .pt_3()
            .px_3()
            .pb_2()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .h(px(28.))
                    .px_2()
                    .flex()
                    .items_center()
                    .gap_2()
                    .rounded_md()
                    .bg(rgb(surface))
                    .border_1()
                    .border_color(rgb(border_variant))
                    .child(icon_sm("search", text_muted))
                    .child(
                        div()
                            .flex_1()
                            .child(TextInputElement::new(
                                self.history_search_input.clone(),
                                text,
                                text_placeholder,
                            )),
                    ),
            )
            .child(
                div()
                    .id("clear-history-btn")
                    .px_2()
                    .py_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_1()
                    .rounded_md()
                    .bg(rgb(surface))
                    .border_1()
                    .border_color(rgb(border_variant))
                    .text_xs()
                    .text_color(rgb(text_muted))
                    .cursor_pointer()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.clear_query_history(cx);
                    }))
                    .child(icon_sm("trash-2", text_muted))
                    .child("Clear History"),
            )
    }

    fn render_history_entry(
        &self,
        entry: &QueryHistoryEntry,
        index: usize,
        text: u32,
        text_muted: u32,
        element_hover: u32,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let sql_preview = if entry.sql.len() > 60 {
            format!("{}...", &entry.sql[..60])
        } else {
            entry.sql.clone()
        };

        let formatted_time = format_timestamp(&entry.timestamp);
        let execution_time = entry
            .execution_ms
            .map(|ms| format!("{}ms", ms))
            .unwrap_or_else(|| "-".to_string());

        let (status_icon, status_color) = match &entry.status {
            QueryHistoryStatus::Success => ("check", 0x4ade80),
            QueryHistoryStatus::Error(_) => ("x", 0xf87171),
            QueryHistoryStatus::Cancelled => ("circle", text_muted),
        };

        let entry_sql = entry.sql.clone();

        div()
            .id(SharedString::from(format!("history-entry-{}", index)))
            .px_2()
            .py_2()
            .mb_1()
            .flex()
            .flex_col()
            .gap_1()
            .rounded_md()
            .hover(move |s| s.bg(rgb(element_hover)))
            .cursor_pointer()
            .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
                if let Some(tab) = this.tabs.iter().find(|t| Some(t.id) == this.active_tab_id) {
                    tab.editor.update(cx, |editor, cx| {
                        editor.set_value(entry_sql.clone(), window, cx);
                    });
                }

                if event.click_count() == 2 {
                    this.execute_query(cx);
                }
            }))
            .child(
                div()
                    .flex()
                    .items_start()
                    .gap_2()
                    .child(icon_sm(status_icon, status_color))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_sm()
                            .text_color(rgb(text))
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .text_ellipsis()
                            .child(sql_preview),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .text_xs()
                    .text_color(rgb(text_muted))
                    .child(formatted_time)
                    .child(execution_time),
            )
    }
}

fn format_timestamp(timestamp: &str) -> String {
    if let Ok(dt) = DateTime::parse_from_rfc3339(timestamp) {
        let local_dt: DateTime<Local> = dt.with_timezone(&Local);
        let now = Local::now();
        let duration = now.signed_duration_since(local_dt);

        if duration.num_seconds() < 60 {
            "Just now".to_string()
        } else if duration.num_minutes() < 60 {
            format!("{} min ago", duration.num_minutes())
        } else if duration.num_hours() < 24 && local_dt.date_naive() == now.date_naive() {
            local_dt.format("Today %l:%M %p").to_string()
        } else if duration.num_days() < 7 {
            local_dt.format("%b %d %l:%M %p").to_string()
        } else {
            local_dt.format("%b %d").to_string()
        }
    } else {
        timestamp.to_string()
    }
}
