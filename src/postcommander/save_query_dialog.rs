use crate::icons::icon_sm;
use crate::postcommander::page::PostCommanderPage;
use crate::settings::{AppSettings, SavedQueryEntry, SavedQueriesSettings};
use crate::theme::ActiveTheme;
use gpui::*;
use uuid::Uuid;

impl PostCommanderPage {
    pub fn render_save_query_dialog(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let surface = colors.surface;
        let border = colors.border;
        let text = colors.text;
        let text_muted = colors.text_muted;
        let element = colors.element;
        let element_hover = colors.element_hover;
        let accent = colors.accent;
        let accent_foreground = colors.accent_foreground;
        let border_variant = colors.border_variant;
        let background = colors.background;

        let is_editing = self.save_query_dialog.editing_id.is_some();
        let title = if is_editing { "Edit Query" } else { "Save Query" };
        let button_text = if is_editing { "Update" } else { "Save" };

        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .id("save-query-backdrop")
                    .absolute()
                    .inset_0()
                    .bg(rgba(0x00000088))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.save_query_dialog.is_visible = false;
                        this.save_query_dialog.editing_id = None;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .id("save-query-dialog-panel")
                    .relative()
                    .w(px(420.))
                    .p_5()
                    .rounded_xl()
                    .bg(rgb(background))
                    .border_1()
                    .border_color(rgb(border))
                    .shadow_xl()
                    .occlude()
                    .child(self.render_save_dialog_header(cx, title, text, text_muted, element_hover))
                    .child(self.render_save_dialog_form(cx, text_muted, text, surface, border_variant))
                    .child(self.render_save_dialog_buttons(cx, element, element_hover, text, accent, accent_foreground, button_text)),
            )
    }

    fn render_save_dialog_header(
        &self,
        cx: &mut Context<Self>,
        title: &str,
        text: u32,
        text_muted: u32,
        element_hover: u32,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .mb_5()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(icon_sm("bookmark", text_muted))
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(text))
                            .child(title.to_string()),
                    ),
            )
            .child(
                div()
                    .id("close-save-dialog")
                    .size(px(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .cursor_pointer()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.save_query_dialog.is_visible = false;
                        this.save_query_dialog.editing_id = None;
                        cx.notify();
                    }))
                    .child(icon_sm("x", text_muted)),
            )
    }

    fn render_save_dialog_form(
        &self,
        cx: &mut Context<Self>,
        text_muted: u32,
        text: u32,
        surface: u32,
        border_variant: u32,
    ) -> impl IntoElement {
        self.save_query_dialog.input_name.update(cx, |input, _| {
            input.set_colors(text, text_muted);
        });
        self.save_query_dialog.input_folder.update(cx, |input, _| {
            input.set_colors(text, text_muted);
        });
        self.save_query_dialog.input_description.update(cx, |input, _| {
            input.set_colors(text, text_muted);
        });

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text_muted))
                            .child("Name"),
                    )
                    .child(
                        div()
                            .id("input-container-name")
                            .h(px(36.))
                            .px_3()
                            .flex()
                            .items_center()
                            .rounded_lg()
                            .bg(rgb(surface))
                            .border_1()
                            .border_color(rgb(border_variant))
                            .child(
                                div()
                                    .w_full()
                                    .text_sm()
                                    .line_height(px(20.))
                                    .child(self.save_query_dialog.input_name.clone()),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text_muted))
                            .child("Folder (optional)"),
                    )
                    .child(
                        div()
                            .id("input-container-folder")
                            .h(px(36.))
                            .px_3()
                            .flex()
                            .items_center()
                            .rounded_lg()
                            .bg(rgb(surface))
                            .border_1()
                            .border_color(rgb(border_variant))
                            .child(
                                div()
                                    .w_full()
                                    .text_sm()
                                    .line_height(px(20.))
                                    .child(self.save_query_dialog.input_folder.clone()),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text_muted))
                            .child("Description (optional)"),
                    )
                    .child(
                        div()
                            .id("input-container-description")
                            .h(px(36.))
                            .px_3()
                            .flex()
                            .items_center()
                            .rounded_lg()
                            .bg(rgb(surface))
                            .border_1()
                            .border_color(rgb(border_variant))
                            .child(
                                div()
                                    .w_full()
                                    .text_sm()
                                    .line_height(px(20.))
                                    .child(self.save_query_dialog.input_description.clone()),
                            ),
                    ),
            )
    }

    fn render_save_dialog_buttons(
        &self,
        cx: &mut Context<Self>,
        element: u32,
        element_hover: u32,
        text: u32,
        accent: u32,
        accent_foreground: u32,
        button_text: &str,
    ) -> impl IntoElement {
        let btn_text = button_text.to_string();

        div()
            .flex()
            .justify_end()
            .gap_3()
            .mt_6()
            .child(
                div()
                    .id("cancel-save-btn")
                    .h(px(36.))
                    .px_5()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_lg()
                    .bg(rgb(element))
                    .cursor_pointer()
                    .hover(move |s| s.bg(rgb(element_hover)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.save_query_dialog.is_visible = false;
                        this.save_query_dialog.editing_id = None;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text))
                            .child("Cancel"),
                    ),
            )
            .child(
                div()
                    .id("save-query-btn")
                    .h(px(36.))
                    .px_5()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .rounded_lg()
                    .bg(rgb(accent))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.9))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.do_save_query(cx);
                    }))
                    .child(icon_sm("bookmark", accent_foreground))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(accent_foreground))
                            .child(btn_text),
                    ),
            )
    }

    pub fn open_save_query_dialog(&mut self, cx: &mut Context<Self>) {
        let sql = if let Some(tab) = self.tabs.iter().find(|t| Some(t.id) == self.active_tab_id) {
            tab.editor.read(cx).value().to_string()
        } else {
            String::new()
        };

        if sql.trim().is_empty() {
            return;
        }

        self.save_query_dialog.input_name.update(cx, |input, _| {
            input.set_content(String::new());
        });
        self.save_query_dialog.input_folder.update(cx, |input, _| {
            input.set_content(String::new());
        });
        self.save_query_dialog.input_description.update(cx, |input, _| {
            input.set_content(String::new());
        });
        self.save_query_dialog.editing_id = None;
        self.save_query_dialog.is_visible = true;
        cx.notify();
    }

    fn do_save_query(&mut self, cx: &mut Context<Self>) {
        let name = self.save_query_dialog.input_name.read(cx).content().to_string();
        if name.trim().is_empty() {
            return;
        }

        let folder_str = self.save_query_dialog.input_folder.read(cx).content().to_string();
        let folder = if folder_str.trim().is_empty() { None } else { Some(folder_str) };
        let description_str = self.save_query_dialog.input_description.read(cx).content().to_string();
        let description = if description_str.trim().is_empty() { None } else { Some(description_str) };

        let sql = if let Some(tab) = self.tabs.iter().find(|t| Some(t.id) == self.active_tab_id) {
            tab.editor.read(cx).value().to_string()
        } else {
            String::new()
        };

        if sql.trim().is_empty() {
            return;
        }

        let now = chrono::Utc::now().to_rfc3339();

        if let Some(editing_id) = &self.save_query_dialog.editing_id {
            let id = editing_id.clone();
            AppSettings::update_global(cx, |settings| {
                let pc = settings.postcommander_mut();
                let saved = pc.saved_queries.get_or_insert_with(SavedQueriesSettings::default);
                saved.update_entry(&id, |entry| {
                    entry.name = name.clone();
                    entry.folder = folder.clone();
                    entry.description = description.clone();
                    entry.sql = sql.clone();
                });
            });
        } else {
            let entry = SavedQueryEntry {
                id: Uuid::new_v4().to_string(),
                name,
                sql,
                folder,
                description,
                created_at: now,
                last_used: None,
            };

            AppSettings::update_global(cx, |settings| {
                let pc = settings.postcommander_mut();
                let saved = pc.saved_queries.get_or_insert_with(SavedQueriesSettings::default);
                saved.add_entry(entry);
            });
        }

        AppSettings::get_global(cx).save();

        self.save_query_dialog.is_visible = false;
        self.save_query_dialog.editing_id = None;
        cx.notify();
    }
}
