use gio::prelude::*;

use crate::app_index::AppEntry;

/// Launch the given application.
pub fn launch(app: &AppEntry) -> Result<(), glib::Error> {
    app.app_info.launch(&[], gio::AppLaunchContext::NONE)?;
    Ok(())
}
