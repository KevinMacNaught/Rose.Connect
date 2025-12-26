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

pub fn coffee() -> Theme {
    Theme {
        id: "coffee".to_string(),
        name: "Coffee".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x1e1714,
            panel_background: 0x191310,
            editor_background: 0x1e1714,
            surface: 0x252019,
            elevated_surface: 0x2c261e,

            element: 0x3a3028,
            element_hover: 0x4a3f34,
            element_selected: 0x5a4f42,
            element_active: 0x5a4f42,
            element_disabled: 0x2a2420,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x3a3028,

            border: 0x4a3f34,
            border_variant: 0x3a3028,
            border_focused: 0xd4a05a,
            border_selected: 0xd4a05a,
            border_disabled: 0x2a2420,

            text: 0xe8dcc8,
            text_muted: 0xa89888,
            text_accent: 0xd4a05a,
            text_disabled: 0x685850,
            text_placeholder: 0x786860,

            icon: 0xe8dcc8,
            icon_muted: 0xa89888,
            icon_disabled: 0x685850,
            icon_accent: 0xd4a05a,

            status_success: 0x98b878,
            status_warning: 0xe8b860,
            status_error: 0xd07058,
            status_info: 0x68a8b8,

            status_success_background: 0x252a20,
            status_warning_background: 0x2a2518,
            status_error_background: 0x2a201a,
            status_info_background: 0x1a2528,

            status_success_border: 0x4a5a40,
            status_warning_border: 0x5a4a30,
            status_error_border: 0x5a4038,
            status_info_border: 0x3a4a50,

            accent: 0xd4a05a,
            accent_foreground: 0x1e1714,
        },
    }
}

pub fn sky() -> Theme {
    Theme {
        id: "sky".to_string(),
        name: "Sky".into(),
        appearance: Appearance::Light,
        colors: ThemeColors {
            background: 0xf0f7fc,
            panel_background: 0xe8f2fa,
            editor_background: 0xf0f7fc,
            surface: 0xffffff,
            elevated_surface: 0xffffff,

            element: 0xddeaf5,
            element_hover: 0xc8ddef,
            element_selected: 0xb0d0e8,
            element_active: 0xb0d0e8,
            element_disabled: 0xe8f0f5,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0xddeaf5,

            border: 0xc0d5e8,
            border_variant: 0xd5e5f0,
            border_focused: 0x4a9fd8,
            border_selected: 0x4a9fd8,
            border_disabled: 0xe0eaf2,

            text: 0x2c4a60,
            text_muted: 0x5a7a90,
            text_accent: 0x2080c0,
            text_disabled: 0x9ab0c0,
            text_placeholder: 0x88a0b0,

            icon: 0x2c4a60,
            icon_muted: 0x5a7a90,
            icon_disabled: 0x9ab0c0,
            icon_accent: 0x2080c0,

            status_success: 0x38a060,
            status_warning: 0xd09020,
            status_error: 0xd04848,
            status_info: 0x3090d0,

            status_success_background: 0xe8f5ed,
            status_warning_background: 0xfcf4e0,
            status_error_background: 0xfce8e8,
            status_info_background: 0xe8f4fc,

            status_success_border: 0xb0dcc0,
            status_warning_border: 0xecd8a0,
            status_error_border: 0xecc0c0,
            status_info_border: 0xb0d8f0,

            accent: 0x4a9fd8,
            accent_foreground: 0xffffff,
        },
    }
}

pub fn rose_dark() -> Theme {
    Theme {
        id: "rose-dark".to_string(),
        name: "Rose Dark".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x141a24,
            panel_background: 0x101620,
            editor_background: 0x141a24,
            surface: 0x1a2230,
            elevated_surface: 0x202a3a,

            element: 0x283448,
            element_hover: 0x324058,
            element_selected: 0x3c4c68,
            element_active: 0x3c4c68,
            element_disabled: 0x1a2230,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x283448,

            border: 0x324058,
            border_variant: 0x283448,
            border_focused: 0xf10e00,
            border_selected: 0xf10e00,
            border_disabled: 0x1a2230,

            text: 0xe8ecf0,
            text_muted: 0x8898a8,
            text_accent: 0xf43020,
            text_disabled: 0x506070,
            text_placeholder: 0x607080,

            icon: 0xe8ecf0,
            icon_muted: 0x8898a8,
            icon_disabled: 0x506070,
            icon_accent: 0xf43020,

            status_success: 0x48b060,
            status_warning: 0xe8a030,
            status_error: 0xf10e00,
            status_info: 0x2457aa,

            status_success_background: 0x182820,
            status_warning_background: 0x282418,
            status_error_background: 0x281410,
            status_info_background: 0x141a28,

            status_success_border: 0x306840,
            status_warning_border: 0x685830,
            status_error_border: 0x682820,
            status_info_border: 0x2457aa,

            accent: 0xf10e00,
            accent_foreground: 0xffffff,
        },
    }
}

pub fn rose_light() -> Theme {
    Theme {
        id: "rose-light".to_string(),
        name: "Rose Light".into(),
        appearance: Appearance::Light,
        colors: ThemeColors {
            background: 0xf5f7fa,
            panel_background: 0xeef2f6,
            editor_background: 0xf5f7fa,
            surface: 0xffffff,
            elevated_surface: 0xffffff,

            element: 0xe0e6ec,
            element_hover: 0xd0d8e0,
            element_selected: 0xc0ccd8,
            element_active: 0xc0ccd8,
            element_disabled: 0xeef2f6,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0xe0e6ec,

            border: 0xc8d0d8,
            border_variant: 0xd8e0e8,
            border_focused: 0xf10e00,
            border_selected: 0xf10e00,
            border_disabled: 0xe8ecf0,

            text: 0x1f4c94,
            text_muted: 0x506888,
            text_accent: 0xf10e00,
            text_disabled: 0x98a8b8,
            text_placeholder: 0x8898a8,

            icon: 0x1f4c94,
            icon_muted: 0x506888,
            icon_disabled: 0x98a8b8,
            icon_accent: 0xf10e00,

            status_success: 0x28884a,
            status_warning: 0xc88020,
            status_error: 0xd00a00,
            status_info: 0x2457aa,

            status_success_background: 0xe8f5ec,
            status_warning_background: 0xfcf4e0,
            status_error_background: 0xfce8e6,
            status_info_background: 0xe8f0fc,

            status_success_border: 0xb0d8c0,
            status_warning_border: 0xecd8a0,
            status_error_border: 0xecc0b8,
            status_info_border: 0xb0c8e8,

            accent: 0xf10e00,
            accent_foreground: 0xffffff,
        },
    }
}

pub fn nord() -> Theme {
    Theme {
        id: "nord".to_string(),
        name: "Nord".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x2e3440,
            panel_background: 0x272c36,
            editor_background: 0x2e3440,
            surface: 0x3b4252,
            elevated_surface: 0x434c5e,

            element: 0x434c5e,
            element_hover: 0x4c566a,
            element_selected: 0x5e6779,
            element_active: 0x5e6779,
            element_disabled: 0x3b4252,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x434c5e,

            border: 0x4c566a,
            border_variant: 0x434c5e,
            border_focused: 0x88c0d0,
            border_selected: 0x88c0d0,
            border_disabled: 0x3b4252,

            text: 0xeceff4,
            text_muted: 0xd8dee9,
            text_accent: 0x88c0d0,
            text_disabled: 0x616e88,
            text_placeholder: 0x6d7a96,

            icon: 0xeceff4,
            icon_muted: 0xd8dee9,
            icon_disabled: 0x616e88,
            icon_accent: 0x88c0d0,

            status_success: 0xa3be8c,
            status_warning: 0xebcb8b,
            status_error: 0xbf616a,
            status_info: 0x81a1c1,

            status_success_background: 0x354035,
            status_warning_background: 0x3d3830,
            status_error_background: 0x3d3035,
            status_info_background: 0x303848,

            status_success_border: 0x5a7050,
            status_warning_border: 0x6a5a40,
            status_error_border: 0x6a4048,
            status_info_border: 0x4a5a70,

            accent: 0x88c0d0,
            accent_foreground: 0x2e3440,
        },
    }
}

pub fn dracula() -> Theme {
    Theme {
        id: "dracula".to_string(),
        name: "Dracula".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x282a36,
            panel_background: 0x21222c,
            editor_background: 0x282a36,
            surface: 0x343746,
            elevated_surface: 0x3e4156,

            element: 0x44475a,
            element_hover: 0x525569,
            element_selected: 0x606378,
            element_active: 0x606378,
            element_disabled: 0x343746,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x44475a,

            border: 0x525569,
            border_variant: 0x44475a,
            border_focused: 0xbd93f9,
            border_selected: 0xbd93f9,
            border_disabled: 0x343746,

            text: 0xf8f8f2,
            text_muted: 0xbfbfba,
            text_accent: 0xbd93f9,
            text_disabled: 0x6272a4,
            text_placeholder: 0x6e7a9e,

            icon: 0xf8f8f2,
            icon_muted: 0xbfbfba,
            icon_disabled: 0x6272a4,
            icon_accent: 0xbd93f9,

            status_success: 0x50fa7b,
            status_warning: 0xf1fa8c,
            status_error: 0xff5555,
            status_info: 0x8be9fd,

            status_success_background: 0x283830,
            status_warning_background: 0x383828,
            status_error_background: 0x382828,
            status_info_background: 0x283038,

            status_success_border: 0x406848,
            status_warning_border: 0x686840,
            status_error_border: 0x684040,
            status_info_border: 0x406068,

            accent: 0xbd93f9,
            accent_foreground: 0x282a36,
        },
    }
}

pub fn solarized_dark() -> Theme {
    Theme {
        id: "solarized-dark".to_string(),
        name: "Solarized Dark".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x002b36,
            panel_background: 0x00252f,
            editor_background: 0x002b36,
            surface: 0x073642,
            elevated_surface: 0x0a4050,

            element: 0x094050,
            element_hover: 0x0c4a5c,
            element_selected: 0x10566a,
            element_active: 0x10566a,
            element_disabled: 0x053540,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x094050,

            border: 0x0c4a5c,
            border_variant: 0x094050,
            border_focused: 0x268bd2,
            border_selected: 0x268bd2,
            border_disabled: 0x053540,

            text: 0x839496,
            text_muted: 0x657b83,
            text_accent: 0x268bd2,
            text_disabled: 0x4a6068,
            text_placeholder: 0x566a72,

            icon: 0x839496,
            icon_muted: 0x657b83,
            icon_disabled: 0x4a6068,
            icon_accent: 0x268bd2,

            status_success: 0x859900,
            status_warning: 0xb58900,
            status_error: 0xdc322f,
            status_info: 0x2aa198,

            status_success_background: 0x0a3020,
            status_warning_background: 0x1a2810,
            status_error_background: 0x1a1818,
            status_info_background: 0x083030,

            status_success_border: 0x2a5020,
            status_warning_border: 0x4a4010,
            status_error_border: 0x4a2020,
            status_info_border: 0x1a5050,

            accent: 0x268bd2,
            accent_foreground: 0x002b36,
        },
    }
}

pub fn solarized_light() -> Theme {
    Theme {
        id: "solarized-light".to_string(),
        name: "Solarized Light".into(),
        appearance: Appearance::Light,
        colors: ThemeColors {
            background: 0xfdf6e3,
            panel_background: 0xf5eedb,
            editor_background: 0xfdf6e3,
            surface: 0xeee8d5,
            elevated_surface: 0xfdf6e3,

            element: 0xe6dfc8,
            element_hover: 0xddd6bc,
            element_selected: 0xd0c9b0,
            element_active: 0xd0c9b0,
            element_disabled: 0xf0eadc,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0xe6dfc8,

            border: 0xd5ceb8,
            border_variant: 0xe0d9c5,
            border_focused: 0x268bd2,
            border_selected: 0x268bd2,
            border_disabled: 0xe8e2d0,

            text: 0x657b83,
            text_muted: 0x839496,
            text_accent: 0x268bd2,
            text_disabled: 0xa8b0a8,
            text_placeholder: 0x93a1a1,

            icon: 0x657b83,
            icon_muted: 0x839496,
            icon_disabled: 0xa8b0a8,
            icon_accent: 0x268bd2,

            status_success: 0x859900,
            status_warning: 0xb58900,
            status_error: 0xdc322f,
            status_info: 0x2aa198,

            status_success_background: 0xf0f8e0,
            status_warning_background: 0xfcf4d8,
            status_error_background: 0xfce8e0,
            status_info_background: 0xe8f8f8,

            status_success_border: 0xc8e0a0,
            status_warning_border: 0xe8d898,
            status_error_border: 0xe8c0b8,
            status_info_border: 0xa8e0d8,

            accent: 0x268bd2,
            accent_foreground: 0xfdf6e3,
        },
    }
}

pub fn tokyo_night() -> Theme {
    Theme {
        id: "tokyo-night".to_string(),
        name: "Tokyo Night".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x1a1b26,
            panel_background: 0x16161e,
            editor_background: 0x1a1b26,
            surface: 0x24283b,
            elevated_surface: 0x2a2e45,

            element: 0x2f3452,
            element_hover: 0x3b4062,
            element_selected: 0x484e6e,
            element_active: 0x484e6e,
            element_disabled: 0x24283b,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x2f3452,

            border: 0x3b4062,
            border_variant: 0x2f3452,
            border_focused: 0x7aa2f7,
            border_selected: 0x7aa2f7,
            border_disabled: 0x24283b,

            text: 0xc0caf5,
            text_muted: 0x9aa5ce,
            text_accent: 0x7aa2f7,
            text_disabled: 0x565f89,
            text_placeholder: 0x636c95,

            icon: 0xc0caf5,
            icon_muted: 0x9aa5ce,
            icon_disabled: 0x565f89,
            icon_accent: 0x7aa2f7,

            status_success: 0x9ece6a,
            status_warning: 0xe0af68,
            status_error: 0xf7768e,
            status_info: 0x7dcfff,

            status_success_background: 0x202820,
            status_warning_background: 0x282420,
            status_error_background: 0x281c20,
            status_info_background: 0x182028,

            status_success_border: 0x4a6040,
            status_warning_border: 0x605040,
            status_error_border: 0x604048,
            status_info_border: 0x405868,

            accent: 0x7aa2f7,
            accent_foreground: 0x1a1b26,
        },
    }
}

pub fn everforest() -> Theme {
    Theme {
        id: "everforest".to_string(),
        name: "Everforest".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x2d353b,
            panel_background: 0x272e33,
            editor_background: 0x2d353b,
            surface: 0x343f44,
            elevated_surface: 0x3d484d,

            element: 0x3d484d,
            element_hover: 0x475258,
            element_selected: 0x525c62,
            element_active: 0x525c62,
            element_disabled: 0x343f44,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x3d484d,

            border: 0x475258,
            border_variant: 0x3d484d,
            border_focused: 0xa7c080,
            border_selected: 0xa7c080,
            border_disabled: 0x343f44,

            text: 0xd3c6aa,
            text_muted: 0x9da9a0,
            text_accent: 0xa7c080,
            text_disabled: 0x68756e,
            text_placeholder: 0x7a8780,

            icon: 0xd3c6aa,
            icon_muted: 0x9da9a0,
            icon_disabled: 0x68756e,
            icon_accent: 0xa7c080,

            status_success: 0xa7c080,
            status_warning: 0xdbbc7f,
            status_error: 0xe67e80,
            status_info: 0x7fbbb3,

            status_success_background: 0x2a3028,
            status_warning_background: 0x302a20,
            status_error_background: 0x302424,
            status_info_background: 0x242a2c,

            status_success_border: 0x506048,
            status_warning_border: 0x605840,
            status_error_border: 0x604848,
            status_info_border: 0x486058,

            accent: 0xa7c080,
            accent_foreground: 0x2d353b,
        },
    }
}

pub fn midnight() -> Theme {
    Theme {
        id: "midnight".to_string(),
        name: "Midnight".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x000000,
            panel_background: 0x000000,
            editor_background: 0x000000,
            surface: 0x0a0a0a,
            elevated_surface: 0x141414,

            element: 0x1a1a1a,
            element_hover: 0x252525,
            element_selected: 0x303030,
            element_active: 0x303030,
            element_disabled: 0x101010,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x1a1a1a,

            border: 0x2a2a2a,
            border_variant: 0x1e1e1e,
            border_focused: 0x0080ff,
            border_selected: 0x0080ff,
            border_disabled: 0x141414,

            text: 0xe8e8e8,
            text_muted: 0x909090,
            text_accent: 0x40a0ff,
            text_disabled: 0x505050,
            text_placeholder: 0x606060,

            icon: 0xe8e8e8,
            icon_muted: 0x909090,
            icon_disabled: 0x505050,
            icon_accent: 0x40a0ff,

            status_success: 0x40c060,
            status_warning: 0xe0a020,
            status_error: 0xf04040,
            status_info: 0x40a0ff,

            status_success_background: 0x082010,
            status_warning_background: 0x181408,
            status_error_background: 0x180808,
            status_info_background: 0x081018,

            status_success_border: 0x206030,
            status_warning_border: 0x504020,
            status_error_border: 0x502020,
            status_info_border: 0x204060,

            accent: 0x0080ff,
            accent_foreground: 0xffffff,
        },
    }
}

pub fn high_contrast() -> Theme {
    Theme {
        id: "high-contrast".to_string(),
        name: "High Contrast".into(),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: 0x000000,
            panel_background: 0x000000,
            editor_background: 0x000000,
            surface: 0x000000,
            elevated_surface: 0x101010,

            element: 0x202020,
            element_hover: 0x303030,
            element_selected: 0x404040,
            element_active: 0x404040,
            element_disabled: 0x101010,
            ghost_element_background: 0x00000000,
            ghost_element_hover: 0x202020,

            border: 0xffffff,
            border_variant: 0x808080,
            border_focused: 0xffff00,
            border_selected: 0xffff00,
            border_disabled: 0x404040,

            text: 0xffffff,
            text_muted: 0xd0d0d0,
            text_accent: 0xffff00,
            text_disabled: 0x707070,
            text_placeholder: 0x808080,

            icon: 0xffffff,
            icon_muted: 0xd0d0d0,
            icon_disabled: 0x707070,
            icon_accent: 0xffff00,

            status_success: 0x00ff00,
            status_warning: 0xffff00,
            status_error: 0xff0000,
            status_info: 0x00ffff,

            status_success_background: 0x002000,
            status_warning_background: 0x202000,
            status_error_background: 0x200000,
            status_info_background: 0x002020,

            status_success_border: 0x00ff00,
            status_warning_border: 0xffff00,
            status_error_border: 0xff0000,
            status_info_border: 0x00ffff,

            accent: 0xffff00,
            accent_foreground: 0x000000,
        },
    }
}
