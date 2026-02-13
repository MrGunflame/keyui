<script lang="ts">
    import Menu from "./Menu.svelte";
    import FilePicker from "./FilePicker.svelte";

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
        padding: 0;
        background-color: #333;
        border: 1px solid #444;
    }
</style>
