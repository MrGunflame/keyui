<script lang="ts">
    import TermTree from "$lib/components/TermTree.svelte";

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
        return seq;
    }

    $effect(() => {
        if (appState.proof == null || appState.active_node == null) {
            return;
        }

        fetchSequent(
            appState.client,
            appState.proof,
            appState.active_node,
        ).then((seq) => {
            sequent = seq;
        });
    });
</script>

<div class="sequent-container">
    <h3>Sequent</h3>
    <div class="sequent-content">
        <!-- NOTE: That all of this is on a single line is deliberate: the pre element is whitespace/tab sensitive. -->
        <pre><code
                >{#if sequent}{#key sequent}<TermTree {appState} {sequent} />
                    {/key}{:else}<span>{"<no sequent loaded>"}</span>
                {/if}</code
            ></pre>
    </div>
</div>

<style>
    .sequent-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        overflow: hidden;
    }

    .sequent-container h3 {
        margin: 0 0 10px 0;
        flex-shrink: 0;
    }

    .sequent-content {
        flex: 1;
        overflow: auto;
    }

    pre {
        margin: 0;
        white-space: pre-wrap;
        word-wrap: break-word;
    }

    code {
        display: block;
    }
</style>
