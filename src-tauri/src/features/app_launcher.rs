use tauri::Manager;

pub mod commands;
mod desktop_entry;
mod model;
mod state;

pub fn init(app: &tauri::App) {
    app.manage(state::LauncherState::load());
}
