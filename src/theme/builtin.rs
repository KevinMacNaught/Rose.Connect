use super::{Appearance, Theme, ThemeColors};

pub fn one_dark() -> Theme {
    Theme {
        id: "one-dark".to_string(),
        name: "One Dark".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x282c33,
            panel_background: 0x21252b,
            editor_background: 0x282c33,
            surface: 0x2f343e,
            elevated_surface: 0x363c46,

            element: 0x3b414d,
            element_hover: 0x464b57,
            element_selected: 0x4d5362,
            element_active: 0x4d5362,
            element_disabled: 0x32363d,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x464b57,

            border: 0x464b57,
            border_variant: 0x363c46,
            border_focused: 0x47679e,
            border_selected: 0x74ade8,
            border_disabled: 0x32363d,

            text: 0xdce0e5,
            text_muted: 0xa9afbc,
            text_accent: 0x74ade8,
            text_disabled: 0x5c6370,
            text_placeholder: 0x6b727f,

            icon: 0xdce0e5,
            icon_muted: 0xa9afbc,
            icon_disabled: 0x5c6370,
            icon_accent: 0x74ade8,

            status_success: 0xa1c181,
            status_warning: 0xdec184,
            status_error: 0xd07277,
            status_info: 0x74ade8,

            status_success_background: 0x2a3a2a,
            status_warning_background: 0x3a3528,
            status_error_background: 0x3a2a2a,
            status_info_background: 0x2a3340,

            status_success_border: 0x4a6a4a,
            status_warning_border: 0x6a5a3a,
            status_error_border: 0x6a4a4a,
            status_info_border: 0x4a5a6a,

            accent: 0x74ade8,
            accent_foreground: 0xffffff,
        },
    }
}

pub fn one_light() -> Theme {
    Theme {
        id: "one-light".to_string(),
        name: "One Light".into(),
        appearance: Appearance::Light,
        colors: ThemeColors {
            background: 0xfafafa,
            panel_background: 0xf0f0f0,
            editor_background: 0xfafafa,
            surface: 0xf0f0f0,
            elevated_surface: 0xffffff,

            element: 0xe5e5e5,
            element_hover: 0xd8d8d8,
            element_selected: 0xcbcbcb,
            element_active: 0xcbcbcb,
            element_disabled: 0xf0f0f0,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0xe5e5e5,

            border: 0xd0d0d0,
            border_variant: 0xe0e0e0,
            border_focused: 0x4078f2,
            border_selected: 0x4078f2,
            border_disabled: 0xe8e8e8,

            text: 0x383a42,
            text_muted: 0x696c77,
            text_accent: 0x4078f2,
            text_disabled: 0xa0a1a7,
            text_placeholder: 0x9d9d9f,

            icon: 0x383a42,
            icon_muted: 0x696c77,
            icon_disabled: 0xa0a1a7,
            icon_accent: 0x4078f2,

            status_success: 0x50a14f,
            status_warning: 0xc18401,
            status_error: 0xe45649,
            status_info: 0x4078f2,

            status_success_background: 0xe8f5e8,
            status_warning_background: 0xfff8e8,
            status_error_background: 0xfde8e8,
            status_info_background: 0xe8f0ff,

            status_success_border: 0xb8e0b8,
            status_warning_border: 0xf0d8a0,
            status_error_border: 0xf0b8b8,
            status_info_border: 0xb8d0f0,

            accent: 0x4078f2,
            accent_foreground: 0xffffff,
        },
    }
}

pub fn ayu_dark() -> Theme {
    Theme {
        id: "ayu-dark".to_string(),
        name: "Ayu Dark".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x0b0e14,
            panel_background: 0x0a0d12,
            editor_background: 0x0b0e14,
            surface: 0x0d1017,
            elevated_surface: 0x131721,

            element: 0x1a1f29,
            element_hover: 0x232834,
            element_selected: 0x2d333f,
            element_active: 0x2d333f,
            element_disabled: 0x12161c,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x1a1f29,

            border: 0x2d333f,
            border_variant: 0x1a1f29,
            border_focused: 0x39bae6,
            border_selected: 0x39bae6,
            border_disabled: 0x12161c,

            text: 0xbfbdb6,
            text_muted: 0x6c7380,
            text_accent: 0x39bae6,
            text_disabled: 0x454b55,
            text_placeholder: 0x565c68,

            icon: 0xbfbdb6,
            icon_muted: 0x6c7380,
            icon_disabled: 0x454b55,
            icon_accent: 0x39bae6,

            status_success: 0x7fd962,
            status_warning: 0xffb454,
            status_error: 0xf07178,
            status_info: 0x39bae6,

            status_success_background: 0x1a2a1a,
            status_warning_background: 0x2a2518,
            status_error_background: 0x2a1a1a,
            status_info_background: 0x1a2530,

            status_success_border: 0x3a5a3a,
            status_warning_border: 0x5a4a28,
            status_error_border: 0x5a3a3a,
            status_info_border: 0x3a4a5a,

            accent: 0x39bae6,
            accent_foreground: 0x0b0e14,
        },
    }
}

pub fn ayu_light() -> Theme {
    Theme {
        id: "ayu-light".to_string(),
        name: "Ayu Light".into(),
        appearance: Appearance::Light,
        colors: ThemeColors {
            background: 0xfafafa,
            panel_background: 0xf3f3f3,
            editor_background: 0xfafafa,
            surface: 0xf3f3f3,
            elevated_surface: 0xffffff,

            element: 0xe7e7e7,
            element_hover: 0xdcdcdc,
            element_selected: 0xd1d1d1,
            element_active: 0xd1d1d1,
            element_disabled: 0xf0f0f0,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0xe7e7e7,

            border: 0xd1d1d1,
            border_variant: 0xe7e7e7,
            border_focused: 0x399ee6,
            border_selected: 0x399ee6,
            border_disabled: 0xe8e8e8,

            text: 0x575f66,
            text_muted: 0x8a919a,
            text_accent: 0x399ee6,
            text_disabled: 0xb0b5bb,
            text_placeholder: 0xa0a6ad,

            icon: 0x575f66,
            icon_muted: 0x8a919a,
            icon_disabled: 0xb0b5bb,
            icon_accent: 0x399ee6,

            status_success: 0x86b300,
            status_warning: 0xf2ae49,
            status_error: 0xf51818,
            status_info: 0x399ee6,

            status_success_background: 0xeaf5e0,
            status_warning_background: 0xfff5e0,
            status_error_background: 0xffe8e8,
            status_info_background: 0xe8f4ff,

            status_success_border: 0xc0e0a0,
            status_warning_border: 0xf0d8a0,
            status_error_border: 0xf0b0b0,
            status_info_border: 0xb0d0f0,

            accent: 0x399ee6,
            accent_foreground: 0xffffff,
        },
    }
}

pub fn ayu_mirage() -> Theme {
    Theme {
        id: "ayu-mirage".to_string(),
        name: "Ayu Mirage".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x1f2430,
            panel_background: 0x1a1f29,
            editor_background: 0x1f2430,
            surface: 0x242936,
            elevated_surface: 0x2a303c,

            element: 0x323846,
            element_hover: 0x3b4252,
            element_selected: 0x454d5e,
            element_active: 0x454d5e,
            element_disabled: 0x272d38,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x323846,

            border: 0x3b4252,
            border_variant: 0x323846,
            border_focused: 0x5ccfe6,
            border_selected: 0x5ccfe6,
            border_disabled: 0x272d38,

            text: 0xcccac2,
            text_muted: 0x707a8c,
            text_accent: 0x5ccfe6,
            text_disabled: 0x505868,
            text_placeholder: 0x606878,

            icon: 0xcccac2,
            icon_muted: 0x707a8c,
            icon_disabled: 0x505868,
            icon_accent: 0x5ccfe6,

            status_success: 0x87d96c,
            status_warning: 0xffcc66,
            status_error: 0xff6666,
            status_info: 0x5ccfe6,

            status_success_background: 0x253025,
            status_warning_background: 0x352f20,
            status_error_background: 0x352525,
            status_info_background: 0x253035,

            status_success_border: 0x456545,
            status_warning_border: 0x655a40,
            status_error_border: 0x654545,
            status_info_border: 0x456065,

            accent: 0x5ccfe6,
            accent_foreground: 0x1f2430,
        },
    }
}

pub fn gruvbox_dark() -> Theme {
    Theme {
        id: "gruvbox-dark".to_string(),
        name: "Gruvbox Dark".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x282828,
            panel_background: 0x1d2021,
            editor_background: 0x282828,
            surface: 0x32302f,
            elevated_surface: 0x3c3836,

            element: 0x504945,
            element_hover: 0x665c54,
            element_selected: 0x7c6f64,
            element_active: 0x7c6f64,
            element_disabled: 0x3c3836,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x504945,

            border: 0x504945,
            border_variant: 0x3c3836,
            border_focused: 0x458588,
            border_selected: 0x83a598,
            border_disabled: 0x3c3836,

            text: 0xebdbb2,
            text_muted: 0xa89984,
            text_accent: 0x83a598,
            text_disabled: 0x665c54,
            text_placeholder: 0x7c6f64,

            icon: 0xebdbb2,
            icon_muted: 0xa89984,
            icon_disabled: 0x665c54,
            icon_accent: 0x83a598,

            status_success: 0xb8bb26,
            status_warning: 0xfabd2f,
            status_error: 0xfb4934,
            status_info: 0x83a598,

            status_success_background: 0x32361a,
            status_warning_background: 0x3a3520,
            status_error_background: 0x3a2520,
            status_info_background: 0x2a3530,

            status_success_border: 0x5a6030,
            status_warning_border: 0x6a5530,
            status_error_border: 0x6a4030,
            status_info_border: 0x4a6050,

            accent: 0x83a598,
            accent_foreground: 0x282828,
        },
    }
}

pub fn gruvbox_light() -> Theme {
    Theme {
        id: "gruvbox-light".to_string(),
        name: "Gruvbox Light".into(),
        appearance: Appearance::Light,
        colors: ThemeColors {
            background: 0xfbf1c7,
            panel_background: 0xf2e5bc,
            editor_background: 0xfbf1c7,
            surface: 0xf2e5bc,
            elevated_surface: 0xf9f5d7,

            element: 0xebdbb2,
            element_hover: 0xd5c4a1,
            element_selected: 0xbdae93,
            element_active: 0xbdae93,
            element_disabled: 0xf2e5bc,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0xebdbb2,

            border: 0xd5c4a1,
            border_variant: 0xebdbb2,
            border_focused: 0x458588,
            border_selected: 0x076678,
            border_disabled: 0xebdbb2,

            text: 0x3c3836,
            text_muted: 0x7c6f64,
            text_accent: 0x076678,
            text_disabled: 0xa89984,
            text_placeholder: 0x928374,

            icon: 0x3c3836,
            icon_muted: 0x7c6f64,
            icon_disabled: 0xa89984,
            icon_accent: 0x076678,

            status_success: 0x79740e,
            status_warning: 0xb57614,
            status_error: 0x9d0006,
            status_info: 0x076678,

            status_success_background: 0xf0f5d0,
            status_warning_background: 0xfff0d0,
            status_error_background: 0xffe0d0,
            status_info_background: 0xe0f0f5,

            status_success_border: 0xd0e0a0,
            status_warning_border: 0xf0d0a0,
            status_error_border: 0xf0b0a0,
            status_info_border: 0xa0d0e0,

            accent: 0x076678,
            accent_foreground: 0xfbf1c7,
        },
    }
}
