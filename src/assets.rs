use anyhow::Result;
use gpui::{AssetSource, SharedString};
use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

pub struct Assets {
    base: PathBuf,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        }
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let full_path = self.base.join(path);
        if full_path.exists() {
            return fs::read(full_path)
                .map(|data| Some(Cow::Owned(data)))
                .map_err(|err| err.into());
        }

        // gpui_component looks for "icons/foo.svg", but we have "assets/icons/foo.svg"
        if path.starts_with("icons/") {
            let assets_path = self.base.join("assets").join(path);
            if assets_path.exists() {
                return fs::read(assets_path)
                    .map(|data| Some(Cow::Owned(data)))
                    .map_err(|err| err.into());
            }
        }

        Ok(None)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}
