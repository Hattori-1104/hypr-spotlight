use crate::features::app_launcher::{desktop_entry::read_all, model::DesktopEntry};

pub struct LauncherState {
    pub desktop_entries: Vec<DesktopEntry>,
}

impl LauncherState {
    pub fn load() -> Self {
        Self {
            desktop_entries: read_all(),
        }
    }
}
