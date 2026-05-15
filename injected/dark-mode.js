(function () {
    try {
        if (!window.__owlbox__) return;
        const owl = window.__owlbox__;

        const STYLE_ID = "owlbox-dark-mode";
        const DARK_CSS = "%%DARK_CSS%%";

        let pref = window.__OWLBOX_INITIAL_THEME__ || "system";
        const media = window.matchMedia("(prefers-color-scheme: dark)");

        function isEffectiveDark() {
            if (pref === "dark") return true;
            if (pref === "light") return false;
            return media.matches;
        }

        function attach() {
            if (document.getElementById(STYLE_ID)) return;
            const el = document.createElement("style");
            el.id = STYLE_ID;
            el.textContent = DARK_CSS;
            (document.head || document.documentElement).appendChild(el);
        }

        function detach() {
            const el = document.getElementById(STYLE_ID);
            if (el) el.remove();
        }

        function apply() {
            if (isEffectiveDark()) attach();
            else detach();
        }

        owl.setThemePref = function (next) {
            pref = next;
            apply();
        };

        media.addEventListener("change", apply);
        apply();
    } catch (e) {
        try {
            window.__TAURI_INTERNALS__?.invoke("plugin:event|emit", {
                event: "injected-script-error",
                payload: { script: "dark-mode", error: String(e) },
            });
        } catch {}
    }
})();
