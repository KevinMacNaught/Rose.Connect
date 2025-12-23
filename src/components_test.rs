use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    avatar::Avatar,
    badge::Badge,
    button::{Button, ButtonVariants},
    calendar::{Calendar, CalendarState},
    chart::LineChart,
    checkbox::Checkbox,
    h_flex,
    progress::Progress,
    select::{Select, SelectState, SearchableVec},
    slider::{Slider, SliderState},
    spinner::Spinner,
    switch::Switch,
    v_flex,
    Disableable, Sizable,
};

use crate::components::{ColumnDef, MultiColumnSelect, MultiColumnSelectState, SelectItem};
use crate::theme::ActiveTheme;

#[derive(Clone)]
struct SalesData {
    month: SharedString,
    value: f64,
}

#[derive(Clone)]
struct ManufacturingLot {
    lot_number: &'static str,
    job_number: &'static str,
    description: &'static str,
}

impl SelectItem for ManufacturingLot {
    fn columns(&self) -> Vec<String> {
        vec![
            self.lot_number.to_string(),
            self.job_number.to_string(),
            self.description.to_string(),
        ]
    }

    fn search_text(&self) -> String {
        format!("{} {} {}", self.lot_number, self.job_number, self.description)
    }

    fn display(&self) -> String {
        format!("{} - {}", self.lot_number, self.description)
    }
}

pub struct ComponentsTest {
    calendar_state: Entity<CalendarState>,
    select_state: Entity<SelectState<SearchableVec<&'static str>>>,
    slider_state: Entity<SliderState>,
    sales_data: Vec<SalesData>,
    switch_on: bool,
    checkbox_checked: bool,
    newsletter_checked: bool,
    progress_value: f32,
    lot_select_state: Entity<MultiColumnSelectState<ManufacturingLot>>,
}

impl ComponentsTest {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let calendar_state = cx.new(|cx| CalendarState::new(window, cx));

        let fruits = SearchableVec::new(vec![
            "Apple",
            "Orange",
            "Banana",
            "Grape",
            "Pineapple",
            "Watermelon",
            "Mango",
            "Strawberry",
        ]);
        let select_state = cx.new(|cx| SelectState::new(fruits, None, window, cx).searchable(true));

        let slider_state = cx.new(|_cx| SliderState::new().min(0.0).max(100.0).default_value(50.0));

        let sales_data = vec![
            SalesData { month: "Jan".into(), value: 186.0 },
            SalesData { month: "Feb".into(), value: 305.0 },
            SalesData { month: "Mar".into(), value: 237.0 },
            SalesData { month: "Apr".into(), value: 173.0 },
            SalesData { month: "May".into(), value: 209.0 },
            SalesData { month: "Jun".into(), value: 214.0 },
        ];

        let lots = vec![
            ManufacturingLot { lot_number: "0000-0000", job_number: "Eng Development", description: "Engineering Development" },
            ManufacturingLot { lot_number: "0000-0001", job_number: "Rework", description: "Rework" },
            ManufacturingLot { lot_number: "0607-2003", job_number: "M01-0789200", description: "Manual Inflate RCSP - Medtronic" },
            ManufacturingLot { lot_number: "0607-2004", job_number: "M01-08440", description: "Adult Vent - 20 FR - MDT" },
            ManufacturingLot { lot_number: "0607-2005", job_number: "M18-7101", description: "PVC RCSP 18mm smooth cuff" },
            ManufacturingLot { lot_number: "0607-2006", job_number: "01177", description: "Tipped Dual Lumen Tube 9.0" },
            ManufacturingLot { lot_number: "0607-2007", job_number: "M22-1001", description: "Silicone Catheter Assembly" },
            ManufacturingLot { lot_number: "0607-2008", job_number: "M15-4420", description: "Pediatric Trach Tube 4.5mm" },
        ];

        let lot_select_state = cx.new(|cx| MultiColumnSelectState::new(cx, lots));

        Self {
            calendar_state,
            select_state,
            slider_state,
            sales_data,
            switch_on: false,
            checkbox_checked: true,
            newsletter_checked: false,
            progress_value: 65.0,
            lot_select_state,
        }
    }
}

impl Render for ComponentsTest {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let dropdown_open = self.lot_select_state.read(cx).is_open();

        div()
            .id("components-test")
            .size_full()
            .overflow_y_scroll()
            .bg(rgb(colors.background))
            .p_6()
            .child(
                v_flex()
                    .gap_6()
                    .pb_6()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(colors.text))
                            .child("GPUI Component Test"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(colors.text_muted))
                            .child("Testing gpui-component library integration"),
                    )
                    .child(self.render_custom_select_section(cx))
                    .child(
                        h_flex()
                            .gap_8()
                            .flex_wrap()
                            .child(self.render_buttons_section(cx))
                            .child(self.render_form_controls_section(cx))
                    )
                    .child(
                        h_flex()
                            .gap_8()
                            .flex_wrap()
                            .child(self.render_calendar_section(cx))
                            .child(self.render_select_section(cx))
                    )
                    .child(
                        h_flex()
                            .gap_8()
                            .flex_wrap()
                            .child(self.render_indicators_section(cx))
                            .child(self.render_avatars_section(cx))
                    )
                    .child(self.render_chart_section(cx)),
            )
            .when(dropdown_open, |el| {
                let state = self.lot_select_state.clone();
                el.child(
                    div()
                        .id("lot-dropdown-backdrop")
                        .absolute()
                        .inset_0()
                        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                            state.update(cx, |s, cx| s.close(cx));
                        }),
                )
            })
    }
}

impl ComponentsTest {
    fn render_buttons_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        v_flex()
            .gap_3()
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(colors.text))
                    .child("Buttons"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(colors.border_variant))
                    .bg(rgb(colors.surface))
                    .child(
                        v_flex()
                            .gap_4()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Button::new("btn-primary").label("Primary").primary())
                                    .child(Button::new("btn-secondary").label("Secondary"))
                                    .child(Button::new("btn-danger").label("Danger").danger())
                                    .child(Button::new("btn-success").label("Success").success())
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Button::new("btn-ghost").label("Ghost").ghost())
                                    .child(Button::new("btn-link").label("Link").link())
                                    .child(Button::new("btn-outline").label("Outline").primary().outline())
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Button::new("btn-sm").label("Small").with_size(gpui_component::Size::Small))
                                    .child(Button::new("btn-md").label("Medium"))
                                    .child(Button::new("btn-lg").label("Large").with_size(gpui_component::Size::Large))
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Button::new("btn-loading").label("Loading...").primary().loading(true))
                                    .child(Button::new("btn-disabled").label("Disabled").disabled(true))
                            ),
                    ),
            )
    }

    fn render_form_controls_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let switch_on = self.switch_on;
        let checkbox_checked = self.checkbox_checked;
        let newsletter_checked = self.newsletter_checked;
        let slider_value = self.slider_state.read(cx).value();

        v_flex()
            .gap_3()
            .w(px(320.))
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(colors.text))
                    .child("Form Controls"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(colors.border_variant))
                    .bg(rgb(colors.surface))
                    .child(
                        v_flex()
                            .gap_5()
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(colors.text_muted))
                                            .child("Switch"),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_4()
                                            .child(
                                                Switch::new("switch-1")
                                                    .checked(switch_on)
                                                    .label("Enable notifications")
                                                    .on_click(cx.listener(|this, checked: &bool, _window, cx| {
                                                        this.switch_on = *checked;
                                                        cx.notify();
                                                    })),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(colors.text_muted))
                                            .child(format!("State: {}", if switch_on { "ON" } else { "OFF" })),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(colors.text_muted))
                                            .child("Checkbox"),
                                    )
                                    .child(
                                        Checkbox::new("checkbox-1")
                                            .checked(checkbox_checked)
                                            .label("I agree to the terms")
                                            .on_click(cx.listener(|this, checked: &bool, _window, cx| {
                                                this.checkbox_checked = *checked;
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Checkbox::new("checkbox-2")
                                            .checked(newsletter_checked)
                                            .label("Subscribe to newsletter")
                                            .on_click(cx.listener(|this, checked: &bool, _window, cx| {
                                                this.newsletter_checked = *checked;
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Checkbox::new("checkbox-disabled")
                                            .checked(true)
                                            .label("Disabled checkbox")
                                            .disabled(true),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(colors.text_muted))
                                            .child("Slider"),
                                    )
                                    .child(Slider::new(&self.slider_state))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(colors.text_muted))
                                            .child(format!("Value: {}", slider_value)),
                                    ),
                            ),
                    ),
            )
    }

    fn render_indicators_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        v_flex()
            .gap_3()
            .w(px(320.))
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(colors.text))
                    .child("Indicators"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(colors.border_variant))
                    .bg(rgb(colors.surface))
                    .child(
                        v_flex()
                            .gap_5()
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(colors.text_muted))
                                            .child("Progress Bars"),
                                    )
                                    .child(Progress::new("progress-1").value(self.progress_value))
                                    .child(Progress::new("progress-2").value(35.0))
                                    .child(Progress::new("progress-3").value(100.0)),
                            )
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(colors.text_muted))
                                            .child("Spinner"),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_4()
                                            .child(Spinner::new().with_size(gpui_component::Size::Small))
                                            .child(Spinner::new())
                                            .child(Spinner::new().with_size(gpui_component::Size::Large)),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(colors.text_muted))
                                            .child("Badges"),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_4()
                                            .child(
                                                Badge::new()
                                                    .count(5)
                                                    .child(
                                                        div()
                                                            .px_3()
                                                            .py_1()
                                                            .rounded_md()
                                                            .bg(rgb(colors.element))
                                                            .text_sm()
                                                            .text_color(rgb(colors.text))
                                                            .child("Messages"),
                                                    ),
                                            )
                                            .child(
                                                Badge::new()
                                                    .count(99)
                                                    .child(
                                                        div()
                                                            .px_3()
                                                            .py_1()
                                                            .rounded_md()
                                                            .bg(rgb(colors.element))
                                                            .text_sm()
                                                            .text_color(rgb(colors.text))
                                                            .child("Notifications"),
                                                    ),
                                            )
                                            .child(
                                                Badge::new()
                                                    .dot()
                                                    .child(
                                                        div()
                                                            .px_3()
                                                            .py_1()
                                                            .rounded_md()
                                                            .bg(rgb(colors.element))
                                                            .text_sm()
                                                            .text_color(rgb(colors.text))
                                                            .child("Status"),
                                                    ),
                                            ),
                                    ),
                            ),
                    ),
            )
    }

    fn render_avatars_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        v_flex()
            .gap_3()
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(colors.text))
                    .child("Avatars"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(colors.border_variant))
                    .bg(rgb(colors.surface))
                    .child(
                        v_flex()
                            .gap_4()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(colors.text_muted))
                                    .child("With initials:"),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Avatar::new().name("John Doe").with_size(gpui_component::Size::Small))
                                    .child(Avatar::new().name("Jane Smith"))
                                    .child(Avatar::new().name("Bob Wilson").with_size(gpui_component::Size::Large))
                                    .child(Avatar::new().name("Alice"))
                                    .child(Avatar::new().name("Charlie Brown")),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(colors.text_muted))
                                    .child("Placeholder:"),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Avatar::new().with_size(gpui_component::Size::Small))
                                    .child(Avatar::new())
                                    .child(Avatar::new().with_size(gpui_component::Size::Large)),
                            ),
                    ),
            )
    }

    fn render_calendar_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        v_flex()
            .gap_3()
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(colors.text))
                    .child("Calendar"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(colors.border_variant))
                    .bg(rgb(colors.surface))
                    .child(Calendar::new(&self.calendar_state)),
            )
    }

    fn render_select_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        v_flex()
            .gap_3()
            .w(px(280.))
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(colors.text))
                    .child("Select / Dropdown"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(colors.border_variant))
                    .bg(rgb(colors.surface))
                    .child(
                        v_flex()
                            .gap_4()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(colors.text_muted))
                                    .child("Select a fruit:"),
                            )
                            .child(
                                Select::new(&self.select_state)
                                    .placeholder("Choose a fruit...")
                                    .cleanable(true),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(colors.text_muted))
                                    .child(format!(
                                        "Selected: {:?}",
                                        self.select_state.read(cx).selected_value()
                                    )),
                            ),
                    ),
            )
    }

    fn render_chart_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        v_flex()
            .gap_3()
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(colors.text))
                    .child("Line Chart"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(colors.border_variant))
                    .bg(rgb(colors.surface))
                    .child(
                        div()
                            .h(px(300.))
                            .w_full()
                            .child(
                                LineChart::new(self.sales_data.clone())
                                    .x(|d| d.month.clone())
                                    .y(|d| d.value)
                                    .dot(),
                            ),
                    ),
            )
    }

    fn render_custom_select_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        let selected_info = self.lot_select_state.read(cx).selected().map(|lot| {
            format!("Selected: {} | {} | {}", lot.lot_number, lot.job_number, lot.description)
        });

        v_flex()
            .gap_3()
            .w(px(550.))
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(colors.text))
                    .child("Custom Multi-Column Select"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(colors.border_variant))
                    .bg(rgb(colors.surface))
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(colors.text_muted))
                                    .child("Manufacturing Lot"),
                            )
                            .child(
                                MultiColumnSelect::new(self.lot_select_state.clone())
                                    .columns(vec![
                                        ColumnDef::new("Lot #").width(px(90.)),
                                        ColumnDef::new("Job #").width(px(130.)),
                                        ColumnDef::new("Description"),
                                    ])
                                    .placeholder("Select a lot...")
                                    .dropdown_width(px(550.))
                                    .searchable(true),
                            )
                            .when(selected_info.is_some(), |el: Div| {
                                el.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(colors.text_muted))
                                        .child(selected_info.unwrap()),
                                )
                            }),
                    ),
            )
    }
}
