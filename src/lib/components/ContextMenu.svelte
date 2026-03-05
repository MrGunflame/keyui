<script lang="ts">
    type Props = {
        onClose: () => void;
        children: any;
        open: any;
        positionX: number;
        positionY: number;
    };

    const { onClose, children, open, positionX, positionY }: Props = $props();

    function onKeyDown(event: KeyboardEvent) {
        if (event.key == "Escape") {
            onClose();
        }
    }
</script>

{#if open}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="ctx-backdrop" onclick={() => onClose()}>
        <div
            class="ctx-menu"
            style="top: {positionY}px; left: {positionX}px;"
            onclick={(e) => e.stopPropagation()}
        >
            {@render children?.()}
        </div>
    </div>
{/if}

<svelte:window onkeydown={onKeyDown} />

<style>
    .ctx-backdrop {
        position: fixed;
        inset: 0;
        z-index: 999;
    }

    .ctx-menu {
        position: absolute;
        background: var(--c-panel-2);
        padding: 5px;
        border: 1px solid #444;
        border-radius: 8px;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.4);
        z-index: 1000;
        backdrop-filter: blur(10px);
    }
</style>
