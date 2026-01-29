<script lang="ts">
    let { children } = $props();
</script>

<div class="panel">
    {@render children?.()}
</div>

<style>
    .panel {
        background: #252525;
        padding: 10px;
        border-radius: 6px;
        height: 100%;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }
</style>
