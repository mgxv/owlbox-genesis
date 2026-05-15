use crate::diag;

pub fn stays_inside(url: &tauri::Url) -> bool {
    matches!(
        url.host_str().unwrap_or(""),
        "mail.google.com" | "accounts.google.com"
    )
}

// Don't swap for tauri-plugin-opener: its window.open hijack swallows
// Gmail's link clicks and its detached spawn crashes AppKit on fork.
pub fn open(url: &str) {
    #[cfg(target_os = "macos")]
    {
        diag::check(
            std::process::Command::new("open").arg(url).spawn(),
            "[external] open",
        );
    }
    #[cfg(target_os = "windows")]
    {
        diag::check(
            std::process::Command::new("cmd")
                .args(["/c", "start", "", url])
                .spawn(),
            "[external] open",
        );
    }
    #[cfg(target_os = "linux")]
    {
        diag::check(
            std::process::Command::new("xdg-open").arg(url).spawn(),
            "[external] open",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::Url;

    fn url(s: &str) -> Url {
        Url::parse(s).unwrap()
    }

    #[test]
    fn allows_gmail() {
        assert!(stays_inside(&url(
            "https://mail.google.com/mail/u/0/#inbox"
        )));
    }

    #[test]
    fn allows_accounts() {
        assert!(stays_inside(&url(
            "https://accounts.google.com/ServiceLogin?service=mail"
        )));
    }

    #[test]
    fn rejects_unrelated_host() {
        assert!(!stays_inside(&url("https://example.com/page")));
    }

    #[test]
    fn rejects_google_marketing() {
        // INITIAL_URL exists to bypass workspace.google.com; if Gmail ever
        // redirects there we want the system browser, not the webview.
        assert!(!stays_inside(&url("https://workspace.google.com/")));
    }

    #[test]
    fn rejects_other_google_subdomains() {
        assert!(!stays_inside(&url("https://myaccount.google.com/security")));
        assert!(!stays_inside(&url("https://docs.google.com/document/d/x")));
    }

    #[test]
    fn rejects_missing_host() {
        assert!(!stays_inside(&url("file:///etc/passwd")));
    }
}
