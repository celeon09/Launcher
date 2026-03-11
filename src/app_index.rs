use gio::prelude::*;

/// Represents a single installed application.
#[derive(Clone, Debug)]
pub struct AppEntry {
    pub name: String,
    pub description: String,
    pub icon_name: String,
    pub app_info: gio::AppInfo,
}

/// Load all installed applications via GIO.
pub fn load_apps() -> Vec<AppEntry> {
    gio::AppInfo::all()
        .into_iter()
        .filter(|app| app.should_show())
        .filter_map(|app| {
            let name = app.name().to_string();
            if name.is_empty() {
                return None;
            }
            let description = app
                .description()
                .map(|d| d.to_string())
                .unwrap_or_default();
            let icon_name = app
                .icon()
                .and_then(|icon| {
                    icon.downcast::<gio::ThemedIcon>()
                        .ok()
                        .and_then(|ti| ti.names().into_iter().next())
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| "application-x-executable".to_string());

            Some(AppEntry {
                name,
                description,
                icon_name,
                app_info: app,
            })
        })
        .collect()
}
