<script lang="ts">
    import ThemeToggle from "$lib/components/ThemeToggle.svelte";
    import Menu from "./Menu.svelte";
    import FilePicker from "./FilePicker.svelte";
    import { open } from "@tauri-apps/plugin-fs";

    let { appState, onError } = $props();

    let file_menu_active = $state(false);

    function toggleMenu() {
        file_menu_active ^= true;
    }

    async function openKeyFile(path: string) {
        const id = await appState.client.load({ problemFile: path });
        appState.proof = id;

        const root = await appState.client.proofTreeRoot(id);
        appState.active_node = root.id;
    }

    function tryOpenKeyFile(path: string) {
        openKeyFile(path).catch((err) => onError(err.toString()));
    }
</script>

<div class="header">
    <ul class="top-menu">
        <li>
            <button onclick={toggleMenu}>File</button>
            {#if file_menu_active}
                <Menu>
                    <ul class="submenu">
                        <li>
                            <FilePicker action={tryOpenKeyFile}>Open</FilePicker
                            >
                        </li>
                    </ul>
                </Menu>
            {/if}
        </li>

        <li>
            <button>About</button>
        </li>
    </ul>
    <ThemeToggle />
</div>

<style>
    .header button {
        box-shadow: none;
    }

    .header button:hover {
        background-color: var(--c-hover-bg);
        border-radius: 0;
    }

    .header {
        padding: 8px 15px;
        background-color: var(--c-panel);
        border-bottom: 2px solid var(--c-border);

        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .top-menu {
        display: flex;
        list-style-type: none;
        margin: 0;
        padding: 0;
    }

    .top-menu li {
        display: inline-block;
    }

    .top-menu button {
        padding: 4px 10px;
        background-color: transparent;
        border: none;
        cursor: pointer;
    }

    .submenu {
        display: flex;
        align-items: flex-start;
        flex-direction: column;
        padding: 0;
        background-color: var(--c-panel-2);
        border: 1px solid var(--c-border);
    }

    .top-menu {
        display: flex;
        list-style-type: none;
        margin: 0;
        padding: 0;
    }

    .top-menu li {
        display: inline-block;
    }

    .submenu {
        display: flex;
        align-items: flex-start;
        flex-direction: column;
        padding: 0;
        background-color: var(--c-bg);
        border: 1px solid #444;
    }
</style>
