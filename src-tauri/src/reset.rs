use tauri::{AppHandle, Manager, Runtime};

use crate::{diag, paths};

#[tauri::command]
pub async fn reset_app<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(paths::WINDOW_MAIN) {
        diag::check(
            window.clear_all_browsing_data(),
            "[reset] clear_all_browsing_data",
        );
    }

    if let Some(path) = paths::prefs_path() {
        if path.exists() {
            diag::check(std::fs::remove_file(&path), "[reset] remove preferences");
        }
    }

    Ok(())
}
