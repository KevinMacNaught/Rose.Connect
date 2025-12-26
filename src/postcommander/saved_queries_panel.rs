use crate::components::TextInputElement;
use crate::icons::icon_sm;
use crate::postcommander::PostCommanderPage;
use crate::settings::{AppSettings, SavedQueryEntry};
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;

impl PostCommanderPage {
    pub fn render_saved_queries_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
        let saved = settings
            .postcommander()
            .saved_queries
            .as_ref()
            .map(|s| s.entries.clone())
            .unwrap_or_default();

        let filtered_entries = if self.saved_queries_search_filter.is_empty() {
            saved.clone()
        } else {
            let filter_lower = self.saved_queries_search_filter.to_lowercase();
            saved
                .iter()
                .filter(|e| {
                    e.name.to_lowercase().contains(&filter_lower)
                        || e.sql.to_lowercase().contains(&filter_lower)
                })
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
            .child(self.render_saved_header(surface, border_variant, text_muted, text_placeholder, text))
            .child(
                div()
                    .id("saved-queries-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .p_2()
                    .children(filtered_entries.iter().enumerate().map(|(index, entry)| {
                        self.render_saved_entry(entry, index, text, text_muted, element_hover, cx)
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
                                        .child(if self.saved_queries_search_filter.is_empty() {
                                            "No saved queries"
                                        } else {
                                            "No matching queries"
                                        }),
                                ),
                        )
                    }),
            )
    }

    fn render_saved_header(
        &self,
        surface: u32,
        border_variant: u32,
        text_muted: u32,
        text_placeholder: u32,
        text: u32,
    ) -> impl IntoElement {
        div()
            .pt_3()
            .px_3()
            .pb_2()
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
                                self.saved_queries_search_input.clone(),
                                text,
                                text_placeholder,
                            )),
                    ),
            )
    }

    fn render_saved_entry(
        &self,
        entry: &SavedQueryEntry,
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

        let entry_sql = entry.sql.clone();
        let entry_sql_for_menu = entry.sql.clone();
        let entry_id_for_menu = entry.id.clone();
        let folder_display = entry.folder.clone();

        let context_menu_id = self
            .overlays.saved_query_menu
            .as_ref()
            .map(|(_, _, id, _)| id.clone());
        let is_context_target = context_menu_id.as_ref() == Some(&entry.id);

        div()
            .id(SharedString::from(format!("saved-entry-{}", index)))
            .px_2()
            .py_2()
            .mb_1()
            .flex()
            .flex_col()
            .gap_1()
            .rounded_md()
            .when(is_context_target, |el| el.bg(rgb(element_hover)))
            .when(!is_context_target, |el| el.hover(move |s| s.bg(rgb(element_hover))))
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
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                    this.deploy_saved_query_context_menu(
                        entry_id_for_menu.clone(),
                        entry_sql_for_menu.clone(),
                        event.position,
                        window,
                        cx,
                    );
                }),
            )
            .child(
                div()
                    .flex()
                    .items_start()
                    .gap_2()
                    .child(icon_sm("bookmark", text_muted))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(text))
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .text_ellipsis()
                                    .child(entry.name.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(text_muted))
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .text_ellipsis()
                                    .child(sql_preview),
                            ),
                    ),
            )
            .when_some(folder_display, |el, folder| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .text_xs()
                        .text_color(rgb(text_muted))
                        .child(icon_sm("folder", text_muted))
                        .child(folder),
                )
            })
    }
}
