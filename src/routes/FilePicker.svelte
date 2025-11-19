<script lang="ts">
    import { open } from '@tauri-apps/plugin-dialog';

    async function openPicker() {
        const path = await open({
            multiple: false,
            directory: false,
            filters: [
                {
                    name: "Rust",
                    extensions: ["rs"],
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
    }

    let { children } = $props();
</script>

<button onclick={openPicker}>{@render children?.()}</button>
