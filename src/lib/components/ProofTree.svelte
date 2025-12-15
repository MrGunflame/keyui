<script>
    let proofNodes = [
        { id: 0, label: "0: OPEN GOAL", status: "open" }
    ];
</script>

<style>
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
</style>

<div>
    <h3>Proof Tree</h3>
    {#each proofNodes as n}
        <div class="node {n.status}">
            {n.label}
        </div>
    {/each}
</div>
