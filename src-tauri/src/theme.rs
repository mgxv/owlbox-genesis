use tauri::{AppHandle, Listener, Manager, Runtime, Theme};

use crate::{diag, settings};

fn parse(value: &str) -> Option<Theme> {
    match value {
        "light" => Some(Theme::Light),
        "dark" => Some(Theme::Dark),
        _ => None,
    }
}

pub fn apply<R: Runtime>(app: &AppHandle<R>) {
    let pref = settings::get_string(app, "theme", "system");
    let theme = parse(&pref);

    for (label, window) in app.webview_windows() {
        diag::check(
            window.set_theme(theme),
            &format!("[theme] set_theme {label}"),
        );
    }

    push_pref_to_gmail(app, &pref);
}

fn push_pref_to_gmail<R: Runtime>(app: &AppHandle<R>, pref: &str) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let Ok(literal) = serde_json::to_string(pref) else {
        return;
    };
    let js = format!("window.__owlbox__?.setThemePref?.({literal});");
    diag::check(window.eval(js), "[theme] push pref to webview");
}

pub fn register_handler<R: Runtime>(app: &AppHandle<R>) {
    let handle = app.clone();
    app.listen("theme-changed", move |_| apply(&handle));
}
