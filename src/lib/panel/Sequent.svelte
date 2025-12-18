<script lang="ts">
    import TermTree from '$lib/components/TermTree.svelte';
    
    let { appState } = $props();
    
    let sequent = $state(null);

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
            {#if sequent}
                <TermTree {sequent} />
            {:else}
                <span>{"<no sequent loaded>"}</span>
            {/if}
        </code>
    </pre>
</div>

<style>
    pre {
        white-space: pre-line;
    }
</style>
