<script lang="ts">
    import Menu from './Menu.svelte';
    import FilePicker from './FilePicker.svelte';
    import { open } from '@tauri-apps/plugin-fs';

    let { appState } = $props();

    let file_menu_active = $state(false);
    let big_button = $state(false);

    function toggleMenu() {
        file_menu_active ^= true;
        big_button = false;
    }

    function toggleBigButton() {
        big_button ^= true;
        file_menu_active = false;
    }

    async function openKeyFile(path: string) {
        const file = await open(path, {
            read: true,
        });

        const stat = await file.stat();
        const buf = new Uint8Array(stat.size);
        await file.read(buf);

        const text = new TextDecoder().decode(buf);
        await file.close();

        const id = await appState.client.loadKey(text);
        appState.proof = id;
        const root = await appState.client.proofTreeRoot(id);
        appState.active_node = root.id;
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
                        <FilePicker action={openKeyFile}>Open</FilePicker>
                    </li>
                </ul>
            </Menu>
            {/if}
        </li>
        <li>
            <button onclick={toggleBigButton}>Big Button</button>
            {#if big_button}
            <Menu>
                <ul class="submenu">
                    <li>
                        <button>Button A</button>
                    </li>
                    <li>
                        <button>Button B</button>
                    </li>
                    <li>
                        <button>Button C</button>
                    </li>
                </ul>
            </Menu>
            {/if}
        </li>
        <li><button>About</button></li>
    </ul>
</div>

<style>

.header button {
    box-shadow: none;
}

.header button:hover {
    background-color: #555;
    border-radius: 0;
}

.header {
    padding: 8px 15px;
    background-color: #222;
    color: white;
    border-bottom: 2px solid gray;
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
    color: white;
    cursor: pointer;
}

.submenu {
    display: flex;
    align-items: flex-start;
    flex-direction: column;
    /* No padding so that it aligns with the top menu button. */
    padding: 0;
    background-color: #333;
    border: 1px solid #444;
}

.submenu button {
    padding: 7px;
}

.submenu button:hover {
    background-color: #555;
}

</style>
