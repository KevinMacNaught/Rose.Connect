use crate::icons::icon_sm;
use crate::settings::AppSettings;
use crate::theme::{ActiveTheme, GlobalTheme, ThemeMeta, ThemeRegistry};
use gpui::prelude::FluentBuilder;
use gpui::*;

struct SettingsWindowHandle(Option<WindowHandle<SettingsWindow>>);

impl Global for SettingsWindowHandle {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SettingsSection {
    General,
    Appearance,
}

impl SettingsSection {
    fn label(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Appearance => "Appearance",
        }
    }

    fn icon_name(&self) -> &'static str {
        match self {
            Self::General => "settings",
            Self::Appearance => "eye",
        }
    }

    fn all() -> &'static [SettingsSection] {
        &[SettingsSection::General, SettingsSection::Appearance]
    }
}

pub struct SettingsWindow {
    themes: Vec<ThemeMeta>,
    selected_theme_index: usize,
    current_section: SettingsSection,
    show_theme_dropdown: bool,
}

impl SettingsWindow {
    pub fn from_data(themes: Vec<ThemeMeta>, current_theme: &str) -> Self {
        let selected_theme_index = themes
            .iter()
            .position(|t| t.name.as_ref() == current_theme)
            .unwrap_or(0);

        Self {
            themes,
            selected_theme_index,
            current_section: SettingsSection::General,
            show_theme_dropdown: false,
        }
    }

    fn select_theme(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected_theme_index = index;
        self.show_theme_dropdown = false;
        if let Some(theme_meta) = self.themes.get(index) {
            let theme_name = theme_meta.name.to_string();
            AppSettings::update_global(cx, |settings| {
                settings.theme_name = theme_name;
            });
            AppSettings::get_global(cx).save();
            GlobalTheme::reload_theme(cx);
        }
    }

    fn select_section(&mut self, section: SettingsSection, cx: &mut Context<Self>) {
        self.current_section = section;
        cx.notify();
    }

    fn toggle_theme_dropdown(&mut self, cx: &mut Context<Self>) {
        self.show_theme_dropdown = !self.show_theme_dropdown;
        cx.notify();
    }

    fn render_search(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        div()
            .flex()
            .items_center()
            .h(px(28.))
            .px(px(8.))
            .mb(px(12.))
            .gap(px(8.))
            .rounded_sm()
            .bg(rgb(colors.editor_background))
            .border_1()
            .border_color(rgb(colors.border))
            .child(icon_sm("search", colors.text_muted))
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(rgb(colors.text_muted))
                    .child("Search settings..."),
            )
    }

    fn render_nav_item(
        &self,
        section: SettingsSection,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        let text_color = colors.text;
        let text_muted = colors.text_muted;
        let element_selected = colors.element_selected;
        let element_hover = colors.element_hover;
        let border_variant = colors.border_variant;

        div()
            .id(SharedString::from(format!("nav-{}", section.label())))
            .flex()
            .items_center()
            .w_full()
            .h(px(28.))
            .px(px(4.))
            .gap(px(10.))
            .rounded_sm()
            .cursor_pointer()
            .text_sm()
            .border_1()
            .border_color(gpui::transparent_black())
            .when(is_selected, |el| {
                el.bg(rgb(element_selected))
                    .border_color(rgb(border_variant))
                    .text_color(rgb(text_color))
            })
            .when(!is_selected, |el| el.text_color(rgb(text_muted)))
            .hover(|style| style.bg(rgb(element_hover)))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.select_section(section, cx);
            }))
            .child(icon_sm(section.icon_name(), if is_selected { text_color } else { text_muted }))
            .child(section.label())
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let search = self.render_search(cx);
        let nav_items: Vec<_> = SettingsSection::all()
            .iter()
            .map(|section| {
                let is_selected = *section == self.current_section;
                self.render_nav_item(*section, is_selected, cx)
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .child(search)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .gap(px(1.))
                    .children(nav_items),
            )
    }

    #[allow(dead_code)]
    fn render_section_header(&self, title: &str, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        div()
            .flex()
            .items_center()
            .pb(px(8.))
            .mb(px(8.))
            .border_b_1()
            .border_color(rgb(colors.border_variant))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(colors.text_muted))
                    .child(title.to_string()),
            )
    }

    #[allow(dead_code)]
    fn render_setting_row(
        &self,
        label: &str,
        description: &str,
        control: impl IntoElement,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        div()
            .flex()
            .w_full()
            .py(px(12.))
            .justify_between()
            .items_start()
            .gap(px(48.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .flex_1()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(colors.text))
                            .child(label.to_string()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(colors.text_muted))
                            .child(description.to_string()),
                    ),
            )
            .child(control)
    }

    fn render_general_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let text_muted = colors.text_muted;
        let border_variant = colors.border_variant;

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .pb(px(8.))
                    .mb(px(8.))
                    .border_b_1()
                    .border_color(rgb(border_variant))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text_muted))
                            .child("General"),
                    ),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(text_muted))
                    .child("General settings will be available here."),
            )
    }

    fn render_appearance_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        let text_color = colors.text;
        let text_muted = colors.text_muted;
        let border_color = colors.border;
        let border_variant = colors.border_variant;
        let element = colors.element;
        let element_hover = colors.element_hover;
        let element_selected = colors.element_selected;
        let elevated_surface = colors.elevated_surface;

        let current_theme_name = self
            .themes
            .get(self.selected_theme_index)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| "One Dark".into());

        let dropdown_button = div()
            .id("theme-dropdown-button")
            .flex()
            .items_center()
            .gap(px(8.))
            .h(px(28.))
            .px(px(12.))
            .min_w(px(160.))
            .border_1()
            .border_color(rgb(border_color))
            .rounded_sm()
            .bg(rgb(element))
            .cursor_pointer()
            .hover(|s| s.bg(rgb(element_hover)))
            .on_click(cx.listener(|this, _, _, cx| {
                this.toggle_theme_dropdown(cx);
            }))
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(rgb(text_color))
                    .child(current_theme_name.clone()),
            )
            .child(icon_sm("chevron-down", text_muted));

        let dropdown_menu = div()
            .absolute()
            .top(px(32.))
            .left_0()
            .min_w(px(200.))
            .bg(rgb(elevated_surface))
            .border_1()
            .border_color(rgb(border_color))
            .rounded_md()
            .shadow_lg()
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .py(px(4.))
                    .children(
                        self.themes
                            .iter()
                            .enumerate()
                            .map(|(idx, theme_meta)| {
                                let is_selected = idx == self.selected_theme_index;
                                let is_dark = !theme_meta.appearance.is_light();
                                let name = theme_meta.name.clone();

                                div()
                                    .id(SharedString::from(format!("theme-{}", idx)))
                                    .flex()
                                    .items_center()
                                    .gap(px(10.))
                                    .h(px(28.))
                                    .px(px(12.))
                                    .mx(px(4.))
                                    .rounded_sm()
                                    .cursor_pointer()
                                    .when(is_selected, |el| el.bg(rgb(element_selected)))
                                    .hover(|style| style.bg(rgb(element_hover)))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.select_theme(idx, cx);
                                    }))
                                    .child(
                                        div()
                                            .size(px(10.))
                                            .rounded_full()
                                            .border_1()
                                            .border_color(rgb(border_variant))
                                            .bg(rgb(if is_dark { 0x3b414d } else { 0xf5f5f5 })),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(if is_selected {
                                                text_color
                                            } else {
                                                text_muted
                                            }))
                                            .child(name),
                                    )
                            })
                            .collect::<Vec<_>>(),
                    ),
            );

        let theme_control = div()
            .relative()
            .child(dropdown_button)
            .when(self.show_theme_dropdown, |el| el.child(dropdown_menu));

        let setting_row = div()
            .flex()
            .w_full()
            .py(px(12.))
            .justify_between()
            .items_start()
            .gap(px(48.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .flex_1()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(text_color))
                            .child("Theme"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(text_muted))
                            .child("Choose a color theme for the application."),
                    ),
            )
            .child(theme_control);

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .pb(px(8.))
                    .mb(px(8.))
                    .border_b_1()
                    .border_color(rgb(border_variant))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text_muted))
                            .child("Appearance"),
                    ),
            )
            .child(setting_row)
    }

    fn render_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let text_color = colors.text;

        let page_title = match self.current_section {
            SettingsSection::General => "General",
            SettingsSection::Appearance => "Appearance",
        };

        let content = match self.current_section {
            SettingsSection::General => self.render_general_section(cx).into_any_element(),
            SettingsSection::Appearance => self.render_appearance_section(cx).into_any_element(),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .pt(px(24.))
            .child(
                div()
                    .px(px(32.))
                    .pb(px(16.))
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(text_color))
                            .child(page_title),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .px(px(32.))
                    .child(content),
            )
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let text_color = colors.text;
        let border_color = colors.border;
        let panel_bg = colors.panel_background;
        let editor_bg = colors.editor_background;

        let sidebar = self.render_sidebar(cx);
        let content = self.render_content(cx);

        div()
            .flex()
            .size_full()
            .text_color(rgb(text_color))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(224.))
                    .h_full()
                    .flex_none()
                    .pt(px(44.))
                    .pb(px(10.))
                    .px(px(10.))
                    .border_r_1()
                    .border_color(rgb(border_color))
                    .bg(rgb(panel_bg))
                    .child(sidebar),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .bg(rgb(editor_bg))
                    .child(content),
            )
    }
}

pub fn open_settings_window(cx: &mut App) {
    if let Some(handle) = cx.try_global::<SettingsWindowHandle>() {
        if let Some(window_handle) = handle.0 {
            if window_handle
                .update(cx, |_, window, _| {
                    window.activate_window();
                })
                .is_ok()
            {
                return;
            }
        }
    }

    let options = WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: Some("Settings".into()),
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.), px(9.))),
        }),
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(800.), px(550.)),
            cx,
        ))),
        kind: WindowKind::Normal,
        ..Default::default()
    };

    let window_handle = cx
        .open_window(options, |_window, cx| {
            let registry = ThemeRegistry::global(cx);
            let themes = registry.list();
            let current_theme = AppSettings::get_global(cx).theme_name.clone();
            cx.new(|_| SettingsWindow::from_data(themes, &current_theme))
        })
        .unwrap();

    cx.set_global(SettingsWindowHandle(Some(window_handle)));
}
