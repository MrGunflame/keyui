<script lang="ts">
    import Term from "./Term.svelte";
    import { parseFormula } from "$lib/parseFormula.js";

    export let sequent: string = "==> (p -> q) -> !q -> !p";

    // FIXED extraction
    let parts = sequent.split("==>");
    let formula = parts.length > 1 ? parts[1].trim() : sequent.trim();
    console.log("TERM TREE FORMULA:", formula);

    let termTree = parseFormula(formula);

    let selectedPath: number[] | null = null;

    function handleSelect(path: number[]) {
        selectedPath = path;
        console.log("Clicked path:", path);
    }
</script>




<div class="tree">
    <h3>Term Tree</h3>

    <Term term={termTree} path={[]} onSelect={handleSelect} />

    {#if selectedPath}
        <div class="display">
            Selected path: [{selectedPath.join(", ")}]
        </div>
    {/if}
</div>

<style>
    .tree {
        background: #252525;
        padding: 10px;
        border-radius: 6px;
        height: 100%;
    }
    .display {
        margin-top: 8px;
        background: #333;
        padding: 6px;
        border-radius: 4px;
    }
</style>
