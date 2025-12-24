use crate::components_test::ComponentsTest;
use crate::icons::icon_md;
use crate::icons_page::IconsPage;
use crate::kanban::KanbanBoard;
use crate::postcommander::PostCommanderPage;
use crate::settings::{AppSettings, WindowBoundsSettings};
use crate::settings_window::open_settings_window;
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::tooltip::Tooltip;

const TITLEBAR_HEIGHT: f32 = 36.0;
const FOOTER_HEIGHT: f32 = 24.0;

#[derive(Clone, Copy, PartialEq)]
pub enum CurrentView {
    Workspace,
    Icons,
    Components,
    PostCommander,
}

impl CurrentView {
    fn title(&self) -> &'static str {
        match self {
            CurrentView::Workspace => "Workspace",
            CurrentView::Icons => "Icons",
            CurrentView::Components => "Components",
            CurrentView::PostCommander => "PostCommander",
        }
    }
}

pub struct MainLayout {
    current_view: CurrentView,
    kanban_board: Entity<KanbanBoard>,
    icons_page: Entity<IconsPage>,
    components_test: Entity<ComponentsTest>,
    postcommander_page: Entity<PostCommanderPage>,
}

impl MainLayout {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let kanban_board = cx.new(|cx| KanbanBoard::load(cx));
        let icons_page = cx.new(|_cx| IconsPage::new());
        let components_test = cx.new(|cx| ComponentsTest::new(window, cx));
        let postcommander_page = cx.new(|cx| PostCommanderPage::new(window, cx));

        cx.observe_window_bounds(window, |_, window, cx| {
            let bounds = window.bounds();
            AppSettings::update_global(cx, |settings| {
                settings.window_bounds = Some(WindowBoundsSettings::from_bounds(bounds));
            });
        })
        .detach();

        Self {
            current_view: CurrentView::PostCommander,
            kanban_board,
            icons_page,
            components_test,
            postcommander_page,
        }
    }

    fn set_view(&mut self, view: CurrentView, cx: &mut Context<Self>) {
        self.current_view = view;
        cx.notify();
    }

    fn render_icon_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let sidebar_bg = colors.panel_background;
        let sidebar_item_hover = colors.element_hover;
        let sidebar_item_selected = colors.element_active;
        let text_color = colors.text;
        let text_muted = colors.text_muted;
        let border_variant = colors.border_variant;
        let current_view = self.current_view;

        let nav_items: Vec<_> = vec![
            ("database", "PostCommander", Some(CurrentView::PostCommander)),
            ("grid", "Workspace", Some(CurrentView::Workspace)),
            ("layers", "Components", Some(CurrentView::Components)),
            ("star", "Icons", Some(CurrentView::Icons)),
        ]
        .into_iter()
        .map(|(icon, label, view)| {
            let is_selected = view == Some(current_view);
            let icon_color = if is_selected { text_color } else { text_muted };

            div()
                .id(SharedString::from(format!("nav-btn-{}", label)))
                .size(px(40.))
                .flex()
                .items_center()
                .justify_center()
                .rounded_md()
                .when(is_selected, |el| el.bg(rgb(sidebar_item_selected)))
                .hover(move |s| s.bg(rgb(sidebar_item_hover)))
                .cursor_pointer()
                .when_some(view, |el, v| {
                    el.on_click(cx.listener(move |this, _, _, cx| {
                        this.set_view(v, cx);
                    }))
                })
                .child(icon_md(icon, icon_color))
                .tooltip(move |window, cx| Tooltip::new(label).build(window, cx))
        })
        .collect();

        let settings_icon = div()
            .id("nav-btn-settings")
            .size(px(40.))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .hover(move |s| s.bg(rgb(sidebar_item_hover)))
            .cursor_pointer()
            .on_click(|_, _, cx| open_settings_window(cx))
            .child(icon_md("settings", text_muted))
            .tooltip(move |window, cx| Tooltip::new("Settings").build(window, cx));

        div()
            .w(px(52.))
            .h_full()
            .bg(rgb(sidebar_bg))
            .border_r_1()
            .border_color(rgb(border_variant))
            .flex()
            .flex_col()
            .items_center()
            .py_2()
            .gap_1()
            .children(nav_items)
            .child(div().flex_1())
            .child(settings_icon)
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let shell_bg = colors.element;
        let text_color = colors.text;
        let border_variant = colors.border_variant;

        div()
            .h(px(TITLEBAR_HEIGHT))
            .w_full()
            .bg(rgb(shell_bg))
            .border_b_1()
            .border_color(rgb(border_variant))
            .flex()
            .items_center()
            .justify_between()
            .pl(px(90.))
            .pr_4()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(text_color))
                    .child(self.current_view.title()),
            )
    }

    fn render_footer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let shell_bg = colors.element;
        let text_muted = colors.text_muted;
        let border_variant = colors.border_variant;

        div()
            .h(px(FOOTER_HEIGHT))
            .w_full()
            .bg(rgb(shell_bg))
            .border_t_1()
            .border_color(rgb(border_variant))
            .flex()
            .items_center()
            .px_4()
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(text_muted))
                    .child("Ready"),
            )
    }
}

impl Render for MainLayout {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let background = colors.background;

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.render_header(cx))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .child(self.render_icon_sidebar(cx))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .min_h_0()
                            .bg(rgb(background))
                            .child(match self.current_view {
                                CurrentView::Workspace => self.kanban_board.clone().into_any_element(),
                                CurrentView::Icons => self.icons_page.clone().into_any_element(),
                                CurrentView::Components => self.components_test.clone().into_any_element(),
                                CurrentView::PostCommander => self.postcommander_page.clone().into_any_element(),
                            }),
                    ),
            )
            .child(self.render_footer(cx))
    }
}
