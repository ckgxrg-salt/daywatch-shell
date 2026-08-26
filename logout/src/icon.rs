//! XDG icons locator

use iced::widget::{image, svg};
use std::path::PathBuf;

pub enum XdgIcon {
    Image(image::Handle),
    Svg(svg::Handle),
}

impl Default for XdgIcon {
    fn default() -> Self {
        // TODO: properly handle missing icon
        Self::Image("nope".into())
    }
}

impl From<PathBuf> for XdgIcon {
    fn from(value: PathBuf) -> Self {
        if value.extension().is_some_and(|ext| ext == "svg") {
            XdgIcon::Svg(value.into())
        } else {
            XdgIcon::Image(value.into())
        }
    }
}

pub struct IconManager {
    theme: String,
}

impl IconManager {
    pub fn new() -> Self {
        Self {
            theme: linicon_theme::get_icon_theme().unwrap_or(String::from("hicolor")),
        }
    }

    pub fn lookup(&self, name: &str) -> Option<XdgIcon> {
        freedesktop_icons::lookup(name)
            .with_theme(&self.theme)
            .with_size(0)
            .with_cache()
            .find()
            .map(|p| p.into())
    }
}
