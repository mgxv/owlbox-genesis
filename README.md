# Owlbox

A minimal macOS application that hosts Gmail's web client in a native window, built with [Tauri 2](https://tauri.app) and React. Similar in spirit to [Boxy Suite 2](https://www.boxysuite.com/) — Owlbox does not reimplement Gmail; it embeds the real Gmail UI inside a native WKWebView and adds the OS integrations a browser tab can't. Because Tauri uses the system WebView instead of bundling Chromium, the installed footprint is a few megabytes rather than the ~150 MB of an equivalent Electron build.

## Features

- Dock icon badge with unread count, parsed from Gmail's tab title
- `mailto:` handler registration (Owlbox can be set as the system default mail handler)
- Compose opens in its own native window (`Cmd+Shift+N`)
- External links open in the system default browser
- Native preferences pane with General, Appearance, and Advanced tabs
- Native keyboard shortcuts (see [Shortcuts](#shortcuts))
- Window state persistence across restarts
- Launch at login
- Native macOS notifications for new emails
- Auto-updates — checks and installs silently on startup, takes effect on next launch; or check manually from Preferences → General
- Opt-in crash reporting via Sentry (off by default; no email content is ever sent)

## Install

Download the `.dmg` for your Mac from the [latest release](https://github.com/mgxv/owlbox/releases/latest):

- **Apple Silicon (M1 and later)** — `Owlbox_*_aarch64.dmg`
- **Intel** — `Owlbox_*_x64.dmg`

Drag **Owlbox** into `/Applications`.

On first launch macOS will block the app because it isn't signed with an Apple Developer account:

1. Double-click **Owlbox** — macOS shows an alert saying it cannot be opened. Click **Done**.
2. Open **System Settings → Privacy & Security** and scroll to the Security section.
3. Click **Open Anyway** next to the Owlbox message.
4. Click **Open Anyway** in the confirmation dialog.
5. Enter your password or confirm with Touch ID if prompted.

Alternatively, run this in Terminal to remove the quarantine flag without going through the UI:

```bash
xattr -dr com.apple.quarantine /Applications/Owlbox.app
```

## Notifications

Owlbox shows native macOS notifications for new emails via Gmail's web notification API. macOS will prompt for notification permission on first launch — click **Allow**.

> **Important:** Gmail's own desktop notification setting must also be enabled. In Gmail, go to **Settings (gear icon) → See all settings → General → Desktop Notifications** and set it to **"New mail notifications on"**. Without this, Gmail will not fire notifications regardless of macOS permission.

## Non-goals

Owlbox is intentionally narrow in scope. It does not introduce custom compose or reply surfaces, bridge additional mail or messaging services, manage contacts, or duplicate functionality already provided by Gmail's web client. All mail-related behavior — UI, search, keyboard shortcuts, account switching, and add-ons — is delegated to Google's official client. The only visual modification is the optional Gmail dark theme, which is applied via a bundled Dark Reader injection.

## Requirements

- macOS 15 (Sequoia) or newer
- [mise](https://mise.jdx.dev/) for toolchain management

## Setup

```bash
brew install mise
echo 'eval "$(mise activate zsh)"' >> ~/.zshrc && exec zsh

mise install
pnpm install
```

## Develop

```bash
pnpm tauri dev
```

<details>
<summary>

## Verify

</summary>

CI runs every check below on each PR. Run them locally first to catch issues before pushing.

Frontend (from repo root):

```bash
pnpm lint
pnpm format
pnpm format:check
pnpm exec tsc --noEmit
pnpm test
pnpm build
```

`pnpm format` auto-applies Prettier.

Rust (from `src-tauri/`):

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --locked
cargo audit
```

`cargo fmt --all` auto-applies rustfmt.

Other:

```bash
bash .github/scripts/check-audit-expirations.sh
```

A full bundle build (`pnpm tauri build`) takes 5–15 minutes and isn't strictly required before pushing, but it's the fastest way to catch release-only issues like LTO link failures.

</details>

<details>
<summary>

## Release

</summary>

Releases are produced by `.github/workflows/release.yml` when a `v*.*.*` tag is pushed. The workflow builds separate native binaries for Apple Silicon (`aarch64-apple-darwin`) and Intel (`x86_64-apple-darwin`), signs the updater artifacts with the Tauri signing keys (`TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` repo secrets), and creates a **draft** GitHub Release containing:

- `Owlbox_*_aarch64.dmg` and `Owlbox_*_x86_64.dmg`
- `Owlbox_aarch64.app.tar.gz` and `Owlbox_x64.app.tar.gz`
- `latest.json` — the updater manifest with per-arch signatures and download URLs

To cut a release:

```bash
# Make sure version is in sync across package.json, src-tauri/Cargo.toml, and src-tauri/tauri.conf.json
git tag v0.1.0
git push origin v0.1.0
```

After the workflow finishes (~20–30 min), review the draft at <https://github.com/mgxv/owlbox/releases> and click **Publish release**. The auto-updater endpoint only resolves to published (non-draft) releases — leaving it as a draft means installed apps won't see the update.

For a local bundle build (unsigned, no upload):

```bash
pnpm tauri build
```

Outputs:

- `src-tauri/target/release/bundle/macos/Owlbox.app`
- `src-tauri/target/release/bundle/dmg/Owlbox_<version>_<arch>.dmg`

To wipe all build artifacts and caches, run `./clean.sh`.

</details>

## Shortcuts

| Shortcut      | Action                |
| ------------- | --------------------- |
| `Cmd+,`       | Preferences           |
| `Cmd+Shift+N` | Compose new message   |
| `Cmd+R`       | Reload                |
| `Cmd+F`       | Find / focus search   |
| `Cmd+Shift+V` | Paste and match style |
| `Cmd+=`       | Zoom in               |
| `Cmd+-`       | Zoom out              |
| `Cmd+0`       | Reset to default zoom |

Standard macOS edit and window commands (`Cmd+C`/`V`/`X`/`Z`/`A`, `Cmd+M`, `Cmd+W`, `Cmd+Q`, etc.) are also wired up.

## Preferences

| Key               | Type                                                  | Default    |
| ----------------- | ----------------------------------------------------- | ---------- |
| `theme`           | `"light" \| "dark" \| "system"`                       | `"system"` |
| `gmailTheme`      | `"light" \| "dark"`                                   | `"light"`  |
| `defaultZoom`     | `70 \| 80 \| 90 \| 100 \| 110 \| 120 \| 130` (number) | `100`      |
| `showDockBadge`   | bool                                                  | `true`     |
| `launchAtStartup` | bool                                                  | `false`    |
| `crashReporting`  | bool                                                  | `false`    |

`Cmd+=` / `Cmd+-` step zoom in 10% increments and clamp to 50–150%. `Cmd+0` resets to `defaultZoom`.

`gmailTheme` themes the Gmail web client itself via a bundled [Dark Reader](https://darkreader.org/) build injected as a user script — `theme` only controls Owlbox's own window chrome. Changing `gmailTheme` requires a restart to take effect.

**Reset app** (Advanced tab) clears all WebView data — cookies, session, localStorage, and cache — and resets all preferences to defaults. The next launch will prompt for Gmail sign-in.

Crash reporting requires a `SENTRY_DSN` baked in at build time; without it, the toggle is a no-op.
