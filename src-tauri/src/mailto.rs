use anyhow::Context;
use tauri::{AppHandle, Manager, Runtime, Url};

pub fn dispatch<R: Runtime>(app: &AppHandle<R>, mailto: &Url) -> anyhow::Result<()> {
    let mut gmail_url = Url::parse("https://mail.google.com/mail/")?;
    gmail_url
        .query_pairs_mut()
        .append_pair("extsrc", "mailto")
        .append_pair("url", mailto.as_str());

    let Some(window) = app.get_webview_window("main") else {
        return Ok(());
    };
    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_focus();

    window
        .navigate(gmail_url)
        .context("navigate to gmail compose")?;
    Ok(())
}
