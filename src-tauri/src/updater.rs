use tauri::{AppHandle, Runtime};
use tauri_plugin_updater::UpdaterExt;

use crate::diag;

pub fn check_in_background<R: Runtime>(app: &AppHandle<R>) {
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        let updater = match handle.updater() {
            Ok(u) => u,
            Err(e) => {
                diag::warn(&format!("[updater] init: {e}"));
                return;
            }
        };
        match updater.check().await {
            Ok(Some(update)) => {
                log::info!("[updater] update available: {}", update.version);
                if let Err(e) = update.download_and_install(|_, _| {}, || {}).await {
                    diag::warn(&format!("[updater] install: {e}"));
                } else {
                    log::info!("[updater] installed; takes effect on next launch");
                }
            }
            Ok(None) => log::info!("[updater] up to date"),
            Err(e) => diag::warn(&format!("[updater] check: {e}")),
        }
    });
}
