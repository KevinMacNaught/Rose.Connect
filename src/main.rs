mod assets;
mod components;
mod components_test;
mod icons;
mod icons_page;
mod kanban;
mod layout;
mod postcommander;
mod settings;
mod settings_window;
mod theme;

use assets::Assets;
use gpui::*;
use gpui_component::Root;
use layout::MainLayout;
use settings::AppSettings;
use settings_window::open_settings_window;
use theme::{GlobalTheme, ThemeRegistry};

actions!(app, [Quit, OpenSettings]);

fn app_menus() -> Vec<Menu> {
    vec![Menu {
        name: "Agent Manager".into(),
        items: vec![
            MenuItem::action("Settings...", OpenSettings),
            MenuItem::separator(),
            MenuItem::os_submenu("Services", SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action("Quit Agent Manager", Quit),
        ],
    }]
}

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn open_settings(_: &OpenSettings, cx: &mut App) {
    open_settings_window(cx);
}

fn main() {
    Application::new()
        .with_assets(Assets::new())
        .run(|cx: &mut App| {
        gpui_component::init(cx);

        let settings = AppSettings::load();
        AppSettings::set_global(settings, cx);

        ThemeRegistry::set_global(cx);

        GlobalTheme::init(cx);

        cx.activate(true);
        cx.on_action(quit);
        cx.on_action(open_settings);
        cx.set_menus(app_menus());

        cx.bind_keys([
            KeyBinding::new("backspace", components::Backspace, Some("TextInput")),
            KeyBinding::new("delete", components::Delete, Some("TextInput")),
            KeyBinding::new("left", components::Left, Some("TextInput")),
            KeyBinding::new("right", components::Right, Some("TextInput")),
            KeyBinding::new("shift-left", components::SelectLeft, Some("TextInput")),
            KeyBinding::new("shift-right", components::SelectRight, Some("TextInput")),
            KeyBinding::new("cmd-a", components::SelectAll, Some("TextInput")),
            KeyBinding::new("cmd-v", components::Paste, Some("TextInput")),
            KeyBinding::new("cmd-c", components::Copy, Some("TextInput")),
            KeyBinding::new("cmd-x", components::Cut, Some("TextInput")),
            KeyBinding::new("home", components::Home, Some("TextInput")),
            KeyBinding::new("end", components::End, Some("TextInput")),
        ]);

        let window_bounds = AppSettings::get_global(cx)
            .window_bounds
            .as_ref()
            .map(|b| {
                WindowBounds::Windowed(Bounds::new(
                    point(px(b.x), px(b.y)),
                    size(px(b.width), px(b.height)),
                ))
            })
            .unwrap_or_else(|| {
                WindowBounds::Windowed(Bounds::centered(None, size(px(1200.), px(800.)), cx))
            });

        let options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Agent Manager".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(9.), px(9.))),
            }),
            window_bounds: Some(window_bounds),
            window_background: WindowBackgroundAppearance::Opaque,
            ..Default::default()
        };

        cx.on_window_closed(|cx| {
            let settings = AppSettings::get_global(cx).clone();
            settings.save();
        })
        .detach();

        cx.open_window(options, |window, cx| {
            let main_layout = cx.new(|cx| MainLayout::new(window, cx));
            cx.new(|cx| Root::new(main_layout, window, cx))
        })
        .unwrap();
    });
}
