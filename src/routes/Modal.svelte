<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";

    export let open: boolean = false;
    export let closeOnBackdrop: boolean = true;

    const dispatch = createEventDispatcher();

    function close() {
        dispatch("close");
    }

    function onBackdropClick() {
        if (closeOnBackdrop) close();
    }

    function onKeydown(event: KeyboardEvent) {
        if (event.key === "Escape") close();
    }

    onMount(() => {
        window.addEventListener("keydown", onKeydown);
        return () => window.removeEventListener("keydown", onKeydown);
    });
</script>

{#if open}
    <div class="backdrop" on:click={onBackdropClick}>
        <div class="modal" on:click|stopPropagation>
            <button class="close" on:click={close}>×</button>

            <!-- Modal content -->
            <slot />
        </div>
    </div>
{/if}

<style>
    @keyframes scaleIn {
        from {
            opacity: 0;
            transform: scale(0.95);
        }
        to {
            opacity: 1;
            transform: scale(1);
        }

        .backdrop {
            position: fixed;
            inset: 0;
            background: rgba(0, 0, 0, 0.5);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 1000;
        }

        .modal {
            background: var(--c-panel);
            color: var(--c-text);
            border: 1px solid var(--c-border);
            border-radius: 8px;
            padding: 1.5rem;
            min-width: 300px;
            max-width: 90%;
            position: relative;
            animation: scaleIn 0.2s ease-out;
        }

        .close {
            position: absolute;
            top: 0.5rem;
            right: 0.75rem;
            background: none;
            border: none;
            font-size: 1.5rem;
            cursor: pointer;
            color: var(--c-text);
        }
        .close:hover {
            background: var(--c-hover-bg);
            border-radius: 4px;
        }
    }
</style>
