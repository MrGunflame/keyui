<script lang="ts">
    import ThemeToggle from "$lib/components/ThemeToggle.svelte";
    import Menu from './Menu.svelte';
    import FilePicker from './FilePicker.svelte';
    import { open } from '@tauri-apps/plugin-fs';

    let { appState, onError } = $props();

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
        const id = await appState.client.load({
            problemFile: path,
        });

        appState.proof = id;
        const root = await appState.client.proofTreeRoot(id);
        appState.active_node = root.id;
    }

    function tryOpenKeyFile(path: string) {
        openKeyFile(path).catch(err => {
            onError(err.toString());
        });
    }

    async function autoProof() {
        if (!appState.proof) {
            return;
        }

        const options = {
            method: null,
            dep: null,
            query: null,
            nonLinArith: null,
            maxSteps: 1000,
        };

        const status = await appState.client.proofAuto(appState.proof, options);
        console.log(status);

        appState.proofTreeChanged.notify();
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
                        <FilePicker action={tryOpenKeyFile}>Open</FilePicker>
                    </li>
                </ul>
            </Menu>
            {/if}
        </li>
        <li>
            <button onclick={toggleBigButton}>Proof</button>
            {#if big_button}
            <Menu>
                <ul class="submenu">
                    <li>
                        <button onclick={autoProof}>Auto</button>
                    </li>
                </ul>
            </Menu>
            {/if}
        </li>
        <li><button>About</button></li>
    </ul>
    <ThemeToggle />
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
    background-color: var(--c-panel);
    color: var(--c-text);
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
