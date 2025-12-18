<script>
    import Term from "./Term.svelte";
    import { parseFormula } from "$lib/parseFormula.js";

    export let sequent = "==> (p -> q) -> !q -> !p";

    // Extract formula part after ==>
    let parts = sequent.split("==>");
    let formula = parts.length > 1 ? parts[1].trim() : sequent.trim();
 

    // Parse the formula into a term tree
    let termTree = parseFormula(formula);

    /** @type {any[] | null} */
    let selectedPath = null;

    /** @param {any[]} path */
    function handleTermSelect(path) {
        selectedPath = path;
    }
</script>                                           

<style>
    .goal {
        background: #252525;
        padding: 10px;
        border-radius: 6px;
        height: 50%;
    }
    .path-display {
        margin-top: 10px;
        padding: 8px;
        background: #333;
        border-radius: 4px;
        color: #ccc;
    }
</style>

<div class="goal">
    <h3>Current Goal</h3>

    <!-- CLICKABLE TERM TREE -->
    <Term term={termTree} path={[]} onSelect={handleTermSelect} />

    {#if selectedPath}
        <div class="path-display">
            Selected path: [{selectedPath.join(", ")}]
        </div>
    {/if}
</div>
