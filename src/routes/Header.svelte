<script lang="ts">
    import Menu from './Menu.svelte';
    import FilePicker from './FilePicker.svelte';

    let file_menu_active = $state(false);

    function toggleMenu() {
        file_menu_active ^= true;
    }
</script>

<div class="header">
    <ul>
        <li>
            <button onclick={toggleMenu}>File</button>
            {#if file_menu_active}
            <Menu>
                <FilePicker>Open</FilePicker>
            </Menu>
            {/if}
        </li>
        <li><button>About</button></li>
    </ul>
</div>

<style>

.header {
    padding: 3px;
}

.header ul {
    display: flex;
    list-style-type: none;
    margin: 0;
    padding: 0;
}

.header li {
    display: inline-block;
}

.header button {
}

</style>
