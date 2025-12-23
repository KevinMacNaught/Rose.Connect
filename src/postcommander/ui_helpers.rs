use crate::icons::icon_sm;
use crate::postcommander::database::QueryResult;
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn parse_table_from_select(sql: &str) -> Option<(String, String)> {
    let sql_upper = sql.to_uppercase();
    let sql_trimmed = sql.trim();

    if !sql_upper.trim_start().starts_with("SELECT") {
        return None;
    }

    let from_pos = sql_upper.find(" FROM ")?;
    let after_from = &sql_trimmed[from_pos + 6..];
    let after_from = after_from.trim_start();

    let end_pos = after_from
        .to_uppercase()
        .find(" WHERE ")
        .or_else(|| after_from.to_uppercase().find(" ORDER "))
        .or_else(|| after_from.to_uppercase().find(" LIMIT "))
        .or_else(|| after_from.to_uppercase().find(" GROUP "))
        .or_else(|| after_from.to_uppercase().find(" HAVING "))
        .or_else(|| after_from.to_uppercase().find(" JOIN "))
        .or_else(|| after_from.find(';'))
        .unwrap_or(after_from.len());

    let table_part = after_from[..end_pos].trim();

    if table_part.to_uppercase().contains(" JOIN ") || table_part.contains(',') {
        return None;
    }

    let (schema, table) = if table_part.contains('.') {
        let parts: Vec<&str> = table_part.splitn(2, '.').collect();
        if parts.len() == 2 {
            (
                parts[0].trim().trim_matches('"').to_string(),
                parts[1].trim().trim_matches('"').to_string(),
            )
        } else {
            return None;
        }
    } else {
        ("public".to_string(), table_part.trim_matches('"').to_string())
    };

    if table.is_empty() {
        return None;
    }

    Some((schema, table))
}

#[allow(dead_code)]
pub fn render_form_field(
    label: &str,
    value: &str,
    text_muted: u32,
    text: u32,
    element: u32,
    border_variant: u32,
) -> impl IntoElement {
    div()
        .child(
            div()
                .text_sm()
                .text_color(rgb(text_muted))
                .mb_1()
                .child(label.to_string()),
        )
        .child(
            div()
                .h(px(32.))
                .px_3()
                .flex()
                .items_center()
                .rounded_md()
                .bg(rgb(element))
                .border_1()
                .border_color(rgb(border_variant))
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(text))
                        .child(value.to_string()),
                ),
        )
}

#[allow(dead_code)]
pub fn render_results_table(
    result: &QueryResult,
    background: u32,
    surface: u32,
    element: u32,
    element_hover: u32,
    border_variant: u32,
    text: u32,
    text_muted: u32,
) -> impl IntoElement {
    if result.columns.is_empty() {
        return div()
            .flex()
            .items_center()
            .justify_center()
            .h_full()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(text_muted))
                    .child("Query executed successfully (no rows returned)"),
            )
            .into_any_element();
    }

    div()
        .rounded_md()
        .border_1()
        .border_color(rgb(border_variant))
        .overflow_hidden()
        .child(
            div()
                .flex()
                .bg(rgb(element))
                .border_b_1()
                .border_color(rgb(border_variant))
                .children(result.columns.iter().map(|col| {
                    let name = col.name.clone();
                    let type_name = col.type_name.clone();
                    div()
                        .flex_1()
                        .min_w(px(100.))
                        .h(px(32.))
                        .px_3()
                        .flex()
                        .items_center()
                        .border_r_1()
                        .border_color(rgb(border_variant))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(text))
                                        .child(name),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(text_muted))
                                        .child(type_name),
                                ),
                        )
                })),
        )
        .children(result.rows.iter().enumerate().take(50).map(|(idx, row)| {
            let bg = if idx % 2 == 0 { background } else { surface };

            div()
                .flex()
                .bg(rgb(bg))
                .hover(move |s| s.bg(rgb(element_hover)))
                .border_b_1()
                .border_color(rgb(border_variant))
                .children(row.iter().map(|cell| {
                    let is_null = cell.as_ref() == "NULL";

                    div()
                        .flex_1()
                        .min_w(px(100.))
                        .h(px(32.))
                        .px_3()
                        .flex()
                        .items_center()
                        .border_r_1()
                        .border_color(rgb(border_variant))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(if is_null { text_muted } else { text }))
                                .when(is_null, |el| el.italic())
                                .child(cell.clone()),
                        )
                }))
        }))
        .into_any_element()
}

#[allow(dead_code)]
pub struct TreeNodeColors {
    pub text: u32,
    pub text_muted: u32,
    pub element_hover: u32,
    pub accent: u32,
    pub status_success: u32,
}

#[allow(dead_code)]
pub fn render_tree_node(
    id: impl Into<SharedString>,
    icon: &str,
    icon_color: u32,
    label: impl Into<SharedString>,
    is_expanded: bool,
    has_children: bool,
    colors: &TreeNodeColors,
) -> impl IntoElement {
    let id = id.into();
    let label = label.into();
    let element_hover = colors.element_hover;
    let text_muted = colors.text_muted;
    let text = colors.text;

    div()
        .id(id)
        .px_2()
        .py_1()
        .flex()
        .items_center()
        .gap_2()
        .rounded_md()
        .hover(move |s| s.bg(rgb(element_hover)))
        .cursor_pointer()
        .when(has_children, |el| {
            el.child(icon_sm(
                if is_expanded { "chevron-down" } else { "chevron-right" },
                text_muted,
            ))
        })
        .when(!has_children, |el| el.child(div().w(px(16.))))
        .child(icon_sm(icon, icon_color))
        .child(
            div()
                .text_sm()
                .text_color(rgb(text))
                .child(label),
        )
}
