use tauri::{AppHandle, Listener, Runtime};
use tauri_plugin_autostart::ManagerExt;

use crate::settings;

pub fn apply<R: Runtime>(app: &AppHandle<R>) {
    let want = settings::get_bool(app, "launchAtStartup", false);
    let manager = app.autolaunch();
    let is = manager.is_enabled().unwrap_or(false);
    if want && !is {
        let _ = manager.enable();
    } else if !want && is {
        let _ = manager.disable();
    }
}

pub fn register_handler<R: Runtime>(app: &AppHandle<R>) {
    let handle = app.clone();
    app.listen("launch-at-startup-changed", move |_| apply(&handle));
}
