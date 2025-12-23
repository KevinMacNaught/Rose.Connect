use gpui::{App, Bounds, BorrowAppContext, Global, Pixels};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowBoundsSettings {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConnectionSettings {
    pub host: String,
    pub port: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PostCommanderSettings {
    #[serde(default)]
    pub connection: Option<ConnectionSettings>,
    #[serde(default)]
    pub expanded_nodes: Option<Vec<String>>,
    #[serde(default)]
    pub sidebar_width: Option<f32>,
    #[serde(default)]
    pub editor_height: Option<f32>,
    #[serde(default)]
    pub structure_panel_width: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme_name: String,
    #[serde(default)]
    pub window_bounds: Option<WindowBoundsSettings>,
    #[serde(default)]
    pub postcommander: Option<PostCommanderSettings>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme_name: "One Dark".to_string(),
            window_bounds: None,
            postcommander: None,
        }
    }
}

impl WindowBoundsSettings {
    pub fn from_bounds(bounds: Bounds<Pixels>) -> Self {
        Self {
            x: f32::from(bounds.origin.x),
            y: f32::from(bounds.origin.y),
            width: f32::from(bounds.size.width),
            height: f32::from(bounds.size.height),
        }
    }
}

struct GlobalAppSettings(AppSettings);

impl Global for GlobalAppSettings {}

impl AppSettings {
    pub fn get_global(cx: &App) -> &AppSettings {
        &cx.global::<GlobalAppSettings>().0
    }

    pub fn set_global(settings: AppSettings, cx: &mut App) {
        cx.set_global(GlobalAppSettings(settings));
    }

    pub fn update_global(cx: &mut App, f: impl FnOnce(&mut AppSettings)) {
        cx.update_global::<GlobalAppSettings, _>(|global, _| {
            f(&mut global.0);
        });
    }

    fn settings_path() -> PathBuf {
        PathBuf::from("data/settings.json")
    }

    pub fn load() -> Self {
        let path = Self::settings_path();
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let path = Self::settings_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn postcommander(&self) -> &PostCommanderSettings {
        static DEFAULT: PostCommanderSettings = PostCommanderSettings {
            connection: None,
            expanded_nodes: None,
            sidebar_width: None,
            editor_height: None,
            structure_panel_width: None,
        };
        self.postcommander.as_ref().unwrap_or(&DEFAULT)
    }

    pub fn postcommander_mut(&mut self) -> &mut PostCommanderSettings {
        self.postcommander.get_or_insert_with(PostCommanderSettings::default)
    }
}
