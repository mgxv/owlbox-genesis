// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, LazyLock};

use regex::Regex;

const BUNDLE_ID: &str = "com.github.mgxv.owlbox";

static EMAIL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\w.+\-]+@[\w.\-]+\.[a-zA-Z]{2,}").unwrap());

fn scrub_emails(s: &str) -> String {
    EMAIL_RE.replace_all(s, "<redacted-email>").to_string()
}

fn main() -> anyhow::Result<()> {
    let _sentry_guard = init_sentry();
    owlbox_lib::run()
}

fn init_sentry() -> Option<sentry::ClientInitGuard> {
    let dsn = option_env!("SENTRY_DSN").filter(|s| !s.is_empty())?;
    if !crash_reporting_enabled() {
        return None;
    }
    Some(sentry::init((
        dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            send_default_pii: false,
            before_send: Some(Arc::new(|mut event| {
                event.user = None;
                event.request = None;
                if let Some(msg) = event.message.as_mut() {
                    *msg = scrub_emails(msg);
                }
                for ex in event.exception.values.iter_mut() {
                    if let Some(value) = ex.value.as_mut() {
                        *value = scrub_emails(value);
                    }
                }
                Some(event)
            })),
            ..Default::default()
        },
    )))
}

// Sentry must initialize before Tauri so it catches early panics; read
// prefs.json directly from the path tauri-plugin-store writes to.
fn crash_reporting_enabled() -> bool {
    let Some(data_dir) = dirs::data_dir() else {
        return false;
    };
    let path = data_dir.join(BUNDLE_ID).join("preferences.json");
    let Ok(content) = std::fs::read_to_string(&path) else {
        return false;
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) else {
        return false;
    };
    json.get("crashReporting")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}
