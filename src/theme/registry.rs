use super::{builtin, Appearance, Theme};
use gpui::{App, Global, SharedString};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct ThemeMeta {
    pub name: SharedString,
    pub appearance: Appearance,
}

struct ThemeRegistryState {
    themes: HashMap<SharedString, Arc<Theme>>,
}

pub struct ThemeRegistry {
    state: ThemeRegistryState,
}

impl ThemeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            state: ThemeRegistryState {
                themes: HashMap::new(),
            },
        };
        registry.load_builtin_themes();
        registry
    }

    fn load_builtin_themes(&mut self) {
        self.insert_theme(builtin::one_dark());
        self.insert_theme(builtin::one_light());
        self.insert_theme(builtin::ayu_dark());
        self.insert_theme(builtin::ayu_light());
        self.insert_theme(builtin::ayu_mirage());
        self.insert_theme(builtin::gruvbox_dark());
        self.insert_theme(builtin::gruvbox_light());
    }

    pub fn insert_theme(&mut self, theme: Theme) {
        self.state
            .themes
            .insert(theme.name.clone(), Arc::new(theme));
    }

    pub fn get(&self, name: &str) -> Option<Arc<Theme>> {
        self.state.themes.get(name).cloned()
    }

    pub fn list(&self) -> Vec<ThemeMeta> {
        let mut themes: Vec<_> = self
            .state
            .themes
            .values()
            .map(|t| ThemeMeta {
                name: t.name.clone(),
                appearance: t.appearance,
            })
            .collect();
        themes.sort_by(|a, b| {
            a.appearance
                .is_light()
                .cmp(&b.appearance.is_light())
                .then(a.name.cmp(&b.name))
        });
        themes
    }
}

struct GlobalThemeRegistry(Arc<ThemeRegistry>);

impl Global for GlobalThemeRegistry {}

impl ThemeRegistry {
    pub fn global(cx: &App) -> Arc<Self> {
        cx.global::<GlobalThemeRegistry>().0.clone()
    }

    pub fn set_global(cx: &mut App) {
        cx.set_global(GlobalThemeRegistry(Arc::new(ThemeRegistry::new())));
    }
}
