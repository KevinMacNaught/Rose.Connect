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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum QueryHistoryStatus {
    Success,
    Error(String),
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryHistoryEntry {
    pub sql: String,
    pub timestamp: String,
    pub execution_ms: Option<u64>,
    pub status: QueryHistoryStatus,
    pub database: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryHistorySettings {
    pub entries: Vec<QueryHistoryEntry>,
    pub max_entries: usize,
}

impl Default for QueryHistorySettings {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 100,
        }
    }
}

impl QueryHistorySettings {
    pub fn add_entry(&mut self, entry: QueryHistoryEntry) {
        self.entries.insert(0, entry);
        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn search(&self, query: &str) -> Vec<&QueryHistoryEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|entry| entry.sql.to_lowercase().contains(&query_lower))
            .collect()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedQueryEntry {
    pub id: String,
    pub name: String,
    pub sql: String,
    pub folder: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub last_used: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedQueriesSettings {
    pub entries: Vec<SavedQueryEntry>,
    pub max_entries: usize,
}

impl Default for SavedQueriesSettings {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 500,
        }
    }
}

impl SavedQueriesSettings {
    pub fn add_entry(&mut self, entry: SavedQueryEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn remove_entry(&mut self, id: &str) -> Option<SavedQueryEntry> {
        if let Some(pos) = self.entries.iter().position(|e| e.id == id) {
            Some(self.entries.remove(pos))
        } else {
            None
        }
    }

    pub fn update_entry(&mut self, id: &str, f: impl FnOnce(&mut SavedQueryEntry)) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            f(entry);
            true
        } else {
            false
        }
    }

    pub fn get_entry(&self, id: &str) -> Option<&SavedQueryEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn search(&self, query: &str) -> Vec<&SavedQueryEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.name.to_lowercase().contains(&query_lower)
                    || e.sql.to_lowercase().contains(&query_lower)
                    || e.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .collect()
    }

    pub fn get_folders(&self) -> Vec<String> {
        let mut folders: Vec<String> = self
            .entries
            .iter()
            .filter_map(|e| e.folder.clone())
            .collect();
        folders.sort();
        folders.dedup();
        folders
    }
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
    #[serde(default)]
    pub query_history: Option<QueryHistorySettings>,
    #[serde(default)]
    pub saved_queries: Option<SavedQueriesSettings>,
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
            query_history: None,
            saved_queries: None,
        };
        self.postcommander.as_ref().unwrap_or(&DEFAULT)
    }

    pub fn postcommander_mut(&mut self) -> &mut PostCommanderSettings {
        self.postcommander.get_or_insert_with(PostCommanderSettings::default)
    }
}
