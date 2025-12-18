<script lang="ts">
    import { open } from '@tauri-apps/plugin-dialog';

    let { action, children } = $props();

    async function openPicker() {
        const path = await open({
            multiple: false,
            directory: false,
            filters: [
                {
                    name: ".key, .proof, .rs, Cargo.toml",
                    extensions: ["key", "proof", "rs", "Cargo.toml"],
                },
                // This filter allows any file with any extension to
                // be selected.
                {
                    name: "All Files",
                    extensions: ["*"],
                },
            ],
        });

        // path is null if the user aborted.
        if (path == null) {
            return;
        }

        console.log(path);
        action(path);
    }

</script>

<button class="btn" onclick={openPicker}>{@render children?.()}</button>

<style>

/* FIXME: This currently is a clone of the button style in
`Header.svelte` which we need to style the local button.
It would be better if the caller can decide on the style instead. */

.btn {
    box-shadow: none;

    padding: 4px 10px;
    background-color: transparent;
    border: none;
    color: white;
    cursor: pointer;

    padding: 7px;
    border-radius: 0;
}

.btn:hover {
    background-color: #555;
}

</style>
