use tauri::State;

use crate::features::app_launcher::{model::DesktopEntry, state::LauncherState};

#[tauri::command]
pub fn list_cached(state: State<'_, LauncherState>) -> Vec<DesktopEntry> {
    state.desktop_entries.clone()
}
