<script>
    import Term from "./Term.svelte";

    let proofNodes = [
        { id: 0, label: "0: OPEN GOAL", status: "open" }
    ];

    let termTree = {
        op: "+",
        args: ["a", { op: "*", args: ["b", "c"] }]
    };

    /** @type {any[] | null} */
    let selectedPath = null;

    /** @param {any[]} path */
    function handleTermSelect(path) {

        selectedPath = path;
        console.log("Clicked path:", path);
    }
</script>


<style>
    .tree {
        background: #252525;
        padding: 10px;
        border-radius: 6px;
        height: 50%;
        overflow-y: auto;
    }
    .node {
        padding: 6px;
        margin: 4px 0;
        border-radius: 4px;
    }
    .open {
        background: #662222;
    }
    .closed {
        background: #225522;
    }
    .path-display {
        margin-top: 10px;
        padding: 8px;
        background: #333;
        border-radius: 4px;
        color: #ccc;
    }
</style>

<div class="tree">
    <h3>Proof Tree</h3>

    {#each proofNodes as n}
        <div class="node {n.status}">
            {n.label}
        </div>
    {/each}

    <h3>Term Tree</h3>

    <!-- clickable term rendering -->
    <Term term={termTree} path={[]} onSelect={handleTermSelect} />

    {#if selectedPath}
        <div class="path-display">
            Selected term path: [{selectedPath.join(", ")}]
        </div>
    {/if}
</div>
