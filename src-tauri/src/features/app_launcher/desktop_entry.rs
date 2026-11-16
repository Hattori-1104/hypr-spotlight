use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::features::app_launcher::model::{DesktopEntry, DesktopEntryInfo};

/// `.desktop`ファイルが配置されているディレクトリのパスを列挙する
pub fn data_directories() -> Vec<PathBuf> {
    let home = env::var("HOME").unwrap_or_default();

    let data_home = env::var("XDG_DATA_HOME")
        .ok()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(home).join(".local/share"));

    let data_dirs = env::var("XDG_DATA_DIRS")
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "/usr/local/share:/usr/share".to_string());

    std::iter::once(data_home)
        .chain(env::split_paths(&data_dirs))
        .filter(|path| path.is_absolute())
        .map(|path| path.join("applications"))
        .collect()
}

/// ディレクトリ内の`.desktop`ファイルを列挙する
pub fn collect_desktop_files(dir: &Path, result: &mut Vec<DesktopEntry>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            collect_desktop_files(&path, result);
        } else if path.extension().is_some_and(|ext| ext == "desktop") {
            let info = read_desktop_info(&path);
            if let Some(info) = info {
                // println!("{:#?}", &info);
                if let Some(name) = info.name {
                    if info.entry_type.is_some_and(|t| t == "Application")
                        && info.no_display.is_none_or(|p| p == false)
                        && info.hidden.is_none_or(|p| p == false)
                    {
                        result.push(DesktopEntry {
                            name,
                            path: path.to_string_lossy().into_owned(),
                        });
                    }
                }
            }
        }
    }
}

/// `.desktop`ファイルの内容をパースする
fn read_desktop_info(path: &Path) -> Option<DesktopEntryInfo> {
    let content = fs::read_to_string(path).ok()?;
    let mut in_desktop_entry = false;

    let mut desktop_info = DesktopEntryInfo {
        name: None,
        entry_type: None,
        hidden: None,
        no_display: None,
        not_show_in: None,
        only_show_in: None,
    };

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("[") && line.ends_with("]") {
            if in_desktop_entry {
                break;
            }
            in_desktop_entry = line == "[Desktop Entry]";
            continue;
        }

        if in_desktop_entry {
            if let Some(name) = line.strip_prefix("Name=") {
                desktop_info.name = Some(name.trim().to_string());
            } else if let Some(entry_type) = line.strip_prefix("Type=") {
                desktop_info.entry_type = entry_type.trim().to_string().into();
            } else if let Some(no_display) = line.strip_prefix("NoDisplay=") {
                desktop_info.no_display = Some(no_display.trim() == "true");
            } else if let Some(hidden) = line.strip_prefix("Hidden=") {
                desktop_info.hidden = Some(hidden.trim() == "true");
            } else if let Some(only_show_in) = line.strip_prefix("OnlyShowIn=") {
                desktop_info.only_show_in = Some(
                    only_show_in
                        .trim()
                        .split(";")
                        .filter(|d| !d.is_empty())
                        .map(String::from)
                        .collect(),
                );
            } else if let Some(not_show_in) = line.strip_prefix("not_show_in") {
                desktop_info.not_show_in = Some(
                    not_show_in
                        .trim()
                        .split(";")
                        .filter(|d| !d.is_empty())
                        .map(|d| d.to_string())
                        .collect(),
                )
            }
        }
    }

    Some(desktop_info)
}

pub fn read_all() -> Vec<DesktopEntry> {
    let mut entries = Vec::new();
    for dir in data_directories() {
        collect_desktop_files(&dir, &mut entries);
    }

    entries.sort_by_key(|entry| entry.name.to_lowercase());
    entries
}
