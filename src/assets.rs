// Asset management - SVG icons, images, etc.

use std::fs;
use std::path::PathBuf;
use gpui::{AssetSource, SharedString};

/// Asset loader for SVG icons and other resources
pub struct Assets {
    base: PathBuf,
}

impl Assets {
    /// Create a new Assets instance with the given base path
    pub fn new(base: PathBuf) -> Self {
        Self { base }
    }

    /// Create Assets from the project root (CARGO_MANIFEST_DIR)
    pub fn from_project_root() -> Self {
        Self::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
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

/// Icon paths
pub mod icons {
    pub const PAPER_AIRPLANE: &str = "assets/icons/paper-airplane.svg";
}

