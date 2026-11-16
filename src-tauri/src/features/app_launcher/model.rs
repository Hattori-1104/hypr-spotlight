use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DesktopEntry {
    pub path: String,
    pub name: String,
}

pub struct DesktopEntryInfo {
    pub name: Option<String>,
    pub entry_type: Option<String>,
    pub no_display: Option<bool>,
    pub hidden: Option<bool>,
    pub only_show_in: Option<Vec<String>>,
    pub not_show_in: Option<Vec<String>>,
}
