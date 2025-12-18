<script>
    export let term;

    /** @type {any[]} */
    export let path = [];

   /** @type {(path: any[]) => void} */
   export let onSelect = () => {};


    /** @param {MouseEvent} event */
    function handleClick(event) {
        event.stopPropagation();
        onSelect(path);
    }
</script>


<span onclick={handleClick} style="cursor: pointer; padding: 2px;">
    {#if typeof term === "string"}
        {term}
    {:else}
        <!-- Binop -->
        (
        {#if term.args.length > 1}
            <svelte:self
                term={term.args[0]}
                path={[...path, 0]}
                onSelect={onSelect}
            />

            <span>{term.op}</span>

            <svelte:self
                term={term.args[1]}
                path={[...path, 1]}
                onSelect={onSelect}
            />
        <!-- Unop -->
        {:else}
            <span>{term.op}</span>

            <svelte:self
                term={term.args[0]}
                path={[...path, 0]}
                onSelect={onSelect}
            />
        {/if}
        )
    {/if}
</span>

<style>
    span:hover {
        background: #444;
        border-radius: 3px;
    }
</style>
