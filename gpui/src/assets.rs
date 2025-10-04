// Asset management - SVG icons, images, etc.

use gpui::{AssetSource, SharedString};

/// Asset loader for SVG icons and other resources
#[derive(Default)]
pub struct Assets {
    pub base: &'static str,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| {
                Some(SharedString::from(
                    entry.ok()?.path().to_string_lossy().into_owned(),
                ))
            })
            .collect::<Vec<_>>())
    }
}

/// Icon paths
pub mod icons {
    pub const PAPER_AIRPLANE: &str = "assets/icons/paper-airplane.svg";
    pub const PAPERCLIP: &str = "assets/icons/paperclip.svg";
}

