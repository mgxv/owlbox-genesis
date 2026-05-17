use serde::Deserialize;
use tauri::{AppHandle, Listener, Runtime};
use tauri_plugin_notification::NotificationExt;

use crate::diag;

#[derive(Deserialize)]
struct Payload {
    title: String,
    body: String,
}

pub fn request_permission<R: Runtime>(app: &AppHandle<R>) {
    if let Err(e) = app.notification().request_permission() {
        diag::warn(&format!("[notifications] permission request: {e}"));
    }
}

pub fn register_handler<R: Runtime>(app: &AppHandle<R>) {
    let handle = app.clone();
    app.listen("notification", move |event| {
        let payload = match serde_json::from_str::<Payload>(event.payload()) {
            Ok(p) => p,
            Err(e) => {
                diag::warn(&format!("[notifications] parse: {e}"));
                return;
            }
        };
        if let Err(e) = handle
            .notification()
            .builder()
            .title(&payload.title)
            .body(&payload.body)
            .show()
        {
            diag::warn(&format!("[notifications] show: {e}"));
        }
    });
}
