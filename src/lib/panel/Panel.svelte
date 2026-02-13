<script lang="ts">
    let { children } = $props();
</script>

<div class="panel">
    {@render children?.()}
</div>

<style>
    .panel {
        background: var(--c-panel);
        padding: 10px;
        border-radius: 6px;
        height: 100%;
        color: var(--c-text);
        border: 1px solid var(--c-border);
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }
</style>
