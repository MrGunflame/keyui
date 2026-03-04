<script lang="ts">
    import TermTree from "$lib/components/TermTree.svelte";

    let { appState } = $props();

    let sequent = $state(null);
    let width = $state(120);
    let contentEl: HTMLDivElement | null = $state(null);

    async function fetchSequent(client, proof, node, w) {
        const options = {
            unicode: false,
            width: w,
            indentation: 0,
            pure: false,
            termLabels: true,
        };

        const seq = await client.goalPrint(node, options);
        return seq;
    }
    $effect(() => {
     if(!contentEl) return;

    const ro = new ResizeObserver((entries) => {
        const px = entries[0].contentRect.width;
        width = Math.floor(px / 8);
    });

    ro.observe(contentEl);
    return () => ro.disconnect();
});

    $effect(() => {
        if (appState.proof == null || appState.active_node == null) {
            return;
        }

       const w = width;

    fetchSequent(
        appState.client,
        appState.proof,
        appState.active_node,
        w
    ).then((seq) => {
            sequent = seq;
        });
    });
</script>

<div class="sequent-container">
    <h3>Sequent</h3>
    <div class="sequent-content" bind:this={contentEl}>
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
        white-space: pre;
       
    }

    code {
        display: block;
    }
</style>
