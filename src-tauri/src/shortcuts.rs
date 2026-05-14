use anyhow::Context;
use tauri::menu::{
    Menu, MenuBuilder, MenuEvent, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder,
};
use tauri::{AppHandle, Manager, Runtime};

pub fn build_menu<R: Runtime>(app: &AppHandle<R>) -> anyhow::Result<Menu<R>> {
    let app_menu = SubmenuBuilder::new(app, "Owlbox")
        .item(&PredefinedMenuItem::about(app, None, None)?)
        .separator()
        .item(
            &MenuItemBuilder::with_id("preferences", "Preferences…")
                .accelerator("CmdOrCtrl+,")
                .build(app)?,
        )
        .separator()
        .item(&PredefinedMenuItem::hide(app, None)?)
        .item(&PredefinedMenuItem::hide_others(app, None)?)
        .item(&PredefinedMenuItem::show_all(app, None)?)
        .separator()
        .item(&PredefinedMenuItem::quit(app, None)?)
        .build()?;

    let file_menu = SubmenuBuilder::new(app, "File")
        .item(
            &MenuItemBuilder::with_id("compose", "New Message")
                .accelerator("Ctrl+Cmd+N")
                .build(app)?,
        )
        .build()?;

    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .item(&PredefinedMenuItem::undo(app, None)?)
        .item(&PredefinedMenuItem::redo(app, None)?)
        .separator()
        .item(&PredefinedMenuItem::cut(app, None)?)
        .item(&PredefinedMenuItem::copy(app, None)?)
        .item(&PredefinedMenuItem::paste(app, None)?)
        .item(&PredefinedMenuItem::select_all(app, None)?)
        .build()?;

    let view_menu = SubmenuBuilder::new(app, "View")
        .item(
            &MenuItemBuilder::with_id("reload", "Reload")
                .accelerator("CmdOrCtrl+R")
                .build(app)?,
        )
        .item(
            &MenuItemBuilder::with_id("focus_search", "Find")
                .accelerator("CmdOrCtrl+F")
                .build(app)?,
        )
        .build()?;

    let window_menu = SubmenuBuilder::new(app, "Window")
        .item(&PredefinedMenuItem::minimize(app, None)?)
        .item(&PredefinedMenuItem::close_window(app, None)?)
        .build()?;

    MenuBuilder::new(app)
        .items(&[&app_menu, &file_menu, &edit_menu, &view_menu, &window_menu])
        .build()
        .context("build app menu")
}

pub fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    match event.id().as_ref() {
        "preferences" => {
            let _ = super::preferences::toggle(app);
        }
        "compose" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval(
                    r#"(() => {
                        const btn = document.querySelector('div[role="button"][gh="cm"], .T-I.T-I-KE.L3');
                        if (btn) btn.click();
                    })();"#,
                );
            }
        }
        "reload" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("location.reload();");
            }
        }
        "focus_search" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval(
                    r#"(() => {
                        const el = document.querySelector('input[aria-label*="Search" i]');
                        if (el) { el.focus(); el.select && el.select(); }
                    })();"#,
                );
            }
        }
        _ => {}
    }
}
