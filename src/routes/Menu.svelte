<script lang="ts">
    let { children } = $props();
</script>

<div class="menu">
    {@render children?.()}
</div>

<style>
    .menu {
        position: absolute;
        z-index: 100;
    }
</style>
