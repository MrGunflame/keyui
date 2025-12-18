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
        <!-- operator -->
        <span>{term.op}</span>
        (

        {#each term.args as arg, i}
            <term
                term={arg}
                path={[...path, i]}
                onSelect={onSelect}
            />
            {#if i < term.args.length - 1}, {/if}
        {/each}

        )
    {/if}
</span>

<style>
    span:hover {
        background: #444;
        border-radius: 3px;
    }
</style>
