use crate::components_test::ComponentsTest;
use crate::icons::icon_sm;
use crate::icons_page::IconsPage;
use crate::kanban::KanbanBoard;
use crate::postcommander::PostCommanderPage;
use crate::settings::{AppSettings, WindowBoundsSettings};
use crate::theme::ActiveTheme;
use gpui::prelude::FluentBuilder;
use gpui::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CurrentView {
    Workspace,
    Icons,
    Components,
    PostCommander,
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
}

struct NavItem {
    icon: &'static str,
    label: &'static str,
    view: Option<CurrentView>,
}

impl Render for MainLayout {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        let sidebar_bg = colors.element;
        let sidebar_item_hover = colors.element_hover;
        let sidebar_item_selected = colors.element_active;
        let text_dark = colors.text;
        let text_muted = colors.text_muted;

        let nav_items = vec![
            NavItem { icon: "database", label: "PostCommander", view: Some(CurrentView::PostCommander) },
            NavItem { icon: "grid", label: "Workspace", view: Some(CurrentView::Workspace) },
            NavItem { icon: "layers", label: "Components", view: Some(CurrentView::Components) },
            NavItem { icon: "star", label: "Icons", view: Some(CurrentView::Icons) },
            NavItem { icon: "settings", label: "Settings", view: None },
        ];

        let current_view = self.current_view;

        div()
            .flex()
            .size_full()
            .bg(rgb(colors.background))
            .child(
                div()
                    .w(px(220.))
                    .h_full()
                    .bg(rgb(sidebar_bg))
                    .border_r_1()
                    .border_color(rgb(colors.border_variant))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .h(px(52.))
                            .pt(px(36.))
                            .flex()
                            .items_center()
                            .px_4()
                            .child(
                                div()
                                    .flex()
                                    .flex_1()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .size(px(24.))
                                                    .rounded_md()
                                                    .bg(rgb(colors.text))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(colors.background))
                                                            .child("◇"),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(text_dark))
                                                    .child("Beyond UI"),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .pt_4()
                            .px_2()
                            .children(nav_items.into_iter().map(|item| {
                                let is_selected = item.view == Some(current_view);
                                let bg = if is_selected { sidebar_item_selected } else { sidebar_bg };
                                let hover_bg = sidebar_item_hover;
                                let view = item.view;

                                div()
                                    .id(SharedString::from(format!("nav-{}", item.label)))
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(rgb(bg))
                                    .hover(move |s| s.bg(rgb(hover_bg)))
                                    .cursor_pointer()
                                    .when_some(view, |el, v| {
                                        el.on_click(cx.listener(move |this, _, _, cx| {
                                            this.set_view(v, cx);
                                        }))
                                    })
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_3()
                                            .child(icon_sm(item.icon, text_muted))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(text_dark))
                                                    .child(item.label),
                                            ),
                                    )
                            })),
                    )
                    .child(
                        div()
                            .p_3()
                            .m_2()
                            .rounded_lg()
                            .bg(rgb(colors.surface))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(icon_sm("mail", text_dark))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(text_dark))
                                            .child("Need support?"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(text_muted))
                                    .mt_1()
                                    .child("Get in touch with our agents"),
                            )
                            .child(
                                div()
                                    .mt_3()
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(rgb(text_dark))
                                    .cursor_pointer()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(colors.background))
                                            .text_center()
                                            .child("Contact us"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .p_3()
                            .border_t_1()
                            .border_color(rgb(colors.border_variant))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .size(px(32.))
                                            .rounded_full()
                                            .bg(rgb(0xfde68a))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x92400e))
                                                    .child("AT"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(text_dark))
                                                    .child("Anna Taylor"),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(text_muted))
                                                    .child("anna.t@email.com"),
                                            ),
                                    ),
                            ),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .child(match self.current_view {
                        CurrentView::Workspace => self.kanban_board.clone().into_any_element(),
                        CurrentView::Icons => self.icons_page.clone().into_any_element(),
                        CurrentView::Components => self.components_test.clone().into_any_element(),
                        CurrentView::PostCommander => self.postcommander_page.clone().into_any_element(),
                    }),
            )
    }
}
