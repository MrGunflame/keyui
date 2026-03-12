<script lang="ts">
    import { onMount } from "svelte";

    type Theme = "dark" | "light";
    let theme: Theme = "dark";

    /* =========================
       THEME
    ========================= */

    function applyTheme(t: Theme) {
        theme = t;
        document.documentElement.dataset.theme = t;
        localStorage.setItem("theme", t);
    }

    function toggle() {
        applyTheme(theme === "dark" ? "light" : "dark");
    }

    /* =========================
       FONT SCALING
    ========================= */

    const FONT_KEY = "keyui.fontScale";
    let fontScale: number = 1;

    function applyFontScale(scale: number) {
        document.documentElement.style.setProperty(
            "--font-scale",
            String(scale),
        );
    }

    function saveFontScale(scale: number) {
        fontScale = scale;
        localStorage.setItem(FONT_KEY, String(scale));
        applyFontScale(scale);
    }

    function loadFontScale(): number {
        const raw = localStorage.getItem(FONT_KEY);
        const n = raw ? Number(raw) : 1;
        return Number.isFinite(n) ? n : 1;
    }

    onMount(() => {
        // Load theme
        const saved = localStorage.getItem("theme") as Theme | null;
        if (saved === "dark" || saved === "light") {
            applyTheme(saved);
        } else {
            const prefersLight = window.matchMedia(
                "(prefers-color-scheme: light)",
            ).matches;
            applyTheme(prefersLight ? "light" : "dark");
        }

        // Load font scale
        fontScale = loadFontScale();
        applyFontScale(fontScale);
    });
</script>

<div class="settings-wrapper">
    <button on:click={toggle} class="theme-btn">
        {theme === "dark" ? "Light" : "Dark"}
    </button>

    <div class="font-slider">
        <label>
            Font size
            <input
                type="range"
                min="0.8"
                max="1.6"
                step="0.05"
                bind:value={fontScale}
                on:input={() => saveFontScale(fontScale)}
            />
            <span>{Math.round(fontScale * 100)}%</span>
        </label>
    </div>
</div>

<style>
    .settings-wrapper {
        display: flex;
    }

    .theme-btn {
        border: 1px solid var(--c-border);
        background: var(--c-panel);
        color: var(--c-text);
        padding: 6px 10px;
        border-radius: 8px;
        cursor: pointer;
    }

    .theme-btn:hover {
        border-color: var(--c-border-hover);
    }

    .font-slider {
        display: flex;
        align-items: center;
        gap: 8px;
        color: var(--c-text);
    }

    .font-slider input {
        cursor: pointer;
    }
</style>
