<script lang="ts">
  import { onMount } from "svelte";

  type Theme = "dark" | "light";
  let theme: Theme = "dark";

  function applyTheme(t: Theme) {
    theme = t;
    document.documentElement.dataset.theme = t;
    localStorage.setItem("theme", t);
  }

  function toggle() {
    applyTheme(theme === "dark" ? "light" : "dark");
  }

  onMount(() => {
    const saved = localStorage.getItem("theme") as Theme | null;
    if (saved === "dark" || saved === "light") {
      applyTheme(saved);
      return;
    }

    const prefersLight = window.matchMedia("(prefers-color-scheme: light)").matches;
    applyTheme(prefersLight ? "light" : "dark");
  });
</script>

<button on:click={toggle} class="theme-btn">
  {theme === "dark" ? " Light" : "Dark"}
</button>

<style>
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
</style>
