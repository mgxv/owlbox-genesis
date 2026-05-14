(function () {
    if (!window.__TAURI_INTERNALS__) return;
    if (window.__owlbox__) return;

    window.__owlbox__ = {
        EMAIL_RE: /[\w.+-]+@[\w.-]+\.[a-z]{2,}/i,

        onReady(fn) {
            if (document.readyState === "loading") {
                document.addEventListener("DOMContentLoaded", fn, {
                    once: true,
                });
            } else {
                fn();
            }
        },

        emit(name, payload) {
            window.__TAURI_INTERNALS__.invoke("plugin:event|emit", {
                event: name,
                payload: payload,
            });
        },
    };
})();
