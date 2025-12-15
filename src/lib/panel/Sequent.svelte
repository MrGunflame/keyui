<script lang="ts">
    let { appState } = $props();
    
    let sequent = $state("<no sequent loaded>");

    async function fetchSequent(client, proof, node) {
        const options = {
            unicode: false,
            width: 120,
            indentation: 0,
            pure: false,
            termLabels: true,
        };

        const seq = await client.goalPrint(node, options);
        return seq.result;
    }

    $effect(() => {
        if (appState.proof == null || appState.active_node == null) {
            return;
        }

        fetchSequent(appState.client, appState.proof, appState.active_node).then(seq => {
            sequent = seq;
        });
    });
</script>

<div>
    <h3>Sequent</h3>
    <pre>
        <code>
            {sequent}
        </code>
    </pre>
</div>

<style>
    pre {
        white-space: pre-line;
    }
</style>
