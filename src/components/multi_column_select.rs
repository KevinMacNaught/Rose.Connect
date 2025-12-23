use gpui::*;
use gpui_component::{h_flex, v_flex};
use crate::components::TextInput;
use crate::icons::icon_sm;
use crate::theme::ActiveTheme;

pub trait SelectItem: Clone + 'static {
    fn columns(&self) -> Vec<String>;
    fn search_text(&self) -> String;
    fn display(&self) -> String;
}

#[derive(Clone)]
pub struct ColumnDef {
    pub header: SharedString,
    pub width: Option<Pixels>,
}

impl ColumnDef {
    pub fn new(header: impl Into<SharedString>) -> Self {
        Self {
            header: header.into(),
            width: None,
        }
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

pub struct MultiColumnSelectState<T: SelectItem> {
    items: Vec<T>,
    pub selected: Option<T>,
    open: bool,
    search_input: Entity<TextInput>,
}

impl<T: SelectItem> MultiColumnSelectState<T> {
    pub fn new(cx: &mut Context<Self>, items: Vec<T>) -> Self {
        let theme = cx.theme();
        let colors = theme.colors();
        let text_color = colors.text;
        let text_muted = colors.text_muted;

        let search_input = cx.new(|cx| {
            let mut input = TextInput::new(cx, "Search...");
            input.set_colors(text_color, text_muted);
            input
        });
        Self {
            items,
            selected: None,
            open: false,
            search_input,
        }
    }

    pub fn selected(&self) -> Option<&T> {
        self.selected.as_ref()
    }

    #[allow(dead_code)]
    pub fn set_selected(&mut self, item: Option<T>) {
        self.selected = item;
    }

    #[allow(dead_code)]
    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
    }

    #[allow(dead_code)]
    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.open = false;
        cx.notify();
    }

    #[allow(dead_code)]
    pub fn search_input(&self) -> &Entity<TextInput> {
        &self.search_input
    }

    #[allow(dead_code)]
    pub fn clear_search(&mut self, cx: &mut Context<Self>) {
        self.search_input.update(cx, |input, _| {
            input.set_content("");
        });
    }

    pub fn get_filtered_items(&self, cx: &App) -> Vec<T> {
        let search_text = self.search_input.read(cx).content().to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                if search_text.is_empty() {
                    true
                } else {
                    item.search_text().to_lowercase().contains(&search_text)
                }
            })
            .cloned()
            .collect()
    }
}

#[derive(IntoElement)]
pub struct MultiColumnSelect<T: SelectItem> {
    state: Entity<MultiColumnSelectState<T>>,
    columns: Vec<ColumnDef>,
    searchable: bool,
    placeholder: SharedString,
    dropdown_width: Pixels,
}

impl<T: SelectItem> MultiColumnSelect<T> {
    pub fn new(state: Entity<MultiColumnSelectState<T>>) -> Self {
        Self {
            state,
            columns: vec![],
            searchable: true,
            placeholder: "Select...".into(),
            dropdown_width: px(500.),
        }
    }

    pub fn columns(mut self, columns: Vec<ColumnDef>) -> Self {
        self.columns = columns;
        self
    }

    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn dropdown_width(mut self, width: Pixels) -> Self {
        self.dropdown_width = width;
        self
    }
}

impl<T: SelectItem> RenderOnce for MultiColumnSelect<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        let text = colors.text;
        let text_muted = colors.text_muted;
        let border = colors.border;
        let background = colors.background;
        let surface = colors.surface;
        let element_hover = colors.element_hover;
        let element_bg = colors.element;

        let (is_open, selected, search_input, filtered_items) = {
            let state = self.state.read(cx);
            let is_open = state.open;
            let selected = state.selected.clone();
            let search_input = state.search_input.clone();
            let filtered_items = state.get_filtered_items(cx);
            (is_open, selected, search_input, filtered_items)
        };

        let has_selection = selected.is_some();
        let selected_display = selected.map(|s| s.display()).unwrap_or_default();

        let state_entity = self.state.clone();
        let trigger = {
            let state_for_click = state_entity.clone();
            div()
                .id("multi-select-trigger")
                .px_3()
                .py_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(border))
                .bg(rgb(background))
                .cursor_pointer()
                .hover(move |s| s.bg(rgb(element_hover)))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_sm()
                        .text_color(if has_selection { rgb(text) } else { rgb(text_muted) })
                        .child(if has_selection {
                            selected_display
                        } else {
                            self.placeholder.to_string()
                        }),
                )
                .child(icon_sm("chevron-down", text_muted))
                .on_mouse_down(MouseButton::Left, {
                    move |_, _, cx| {
                        state_for_click.update(cx, |state, cx| {
                            state.toggle();
                            cx.notify();
                        });
                    }
                })
        };

        let dropdown = if is_open {
            let header_row = h_flex()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(rgb(border))
                .bg(rgb(element_bg))
                .children(self.columns.iter().map(|col| {
                    let col_div = div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(text_muted))
                        .child(col.header.clone());
                    if let Some(w) = col.width {
                        col_div.w(w)
                    } else {
                        col_div.flex_1()
                    }
                }));

            let search_row = if self.searchable {
                Some(
                    div()
                        .px_2()
                        .py_2()
                        .border_b_1()
                        .border_color(rgb(border))
                        .child(search_input.clone()),
                )
            } else {
                None
            };

            let columns_for_rows = self.columns.clone();
            let items_list = div()
                .id("multi-select-scroll")
                .max_h(px(250.))
                .overflow_y_scroll()
                .children(filtered_items.into_iter().enumerate().map(|(idx, item)| {
                    let item_for_click = item.clone();
                    let state_for_click = state_entity.clone();
                    let search_input_for_click = search_input.clone();
                    let col_values = item.columns();

                    div()
                        .id(ElementId::Name(format!("select-row-{}", idx).into()))
                        .px_3()
                        .py_2()
                        .cursor_pointer()
                        .hover(move |s| s.bg(rgb(element_hover)))
                        .flex()
                        .children(columns_for_rows.iter().zip(col_values.into_iter()).map(|(col, value)| {
                            let col_div = div()
                                .text_sm()
                                .text_color(rgb(text))
                                .child(value);
                            if let Some(w) = col.width {
                                col_div.w(w)
                            } else {
                                col_div.flex_1()
                            }
                        }))
                        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                            state_for_click.update(cx, |state, cx| {
                                state.selected = Some(item_for_click.clone());
                                state.open = false;
                                cx.notify();
                            });
                            search_input_for_click.update(cx, |input, _| {
                                input.set_content("");
                            });
                        })
                }));

            let panel = div()
                .id("multi-select-panel")
                .occlude()
                .w(self.dropdown_width)
                .bg(rgb(surface))
                .border_1()
                .border_color(rgb(border))
                .rounded_lg()
                .shadow_lg()
                .overflow_hidden()
                .child(
                    v_flex()
                        .child(header_row)
                        .children(search_row)
                        .child(items_list),
                );

            Some(
                deferred(
                    div()
                        .absolute()
                        .top(px(42.))
                        .left_0()
                        .child(panel),
                )
                .with_priority(1),
            )
        } else {
            None
        };

        div()
            .relative()
            .child(trigger)
            .children(dropdown)
    }
}
