<script lang="ts">
    import TermTree from "$lib/components/TermTree.svelte";
    import type {
        Client,
        NodeId,
        NodeTextDesc,
        PrintOptions,
        ProofId,
    } from "../api";

    let { appState } = $props();

    let sequent = $state<NodeTextDesc | null>(null);
    let width = $state(120);
    let contentEl: HTMLDivElement | null = $state(null);
    let charWidthEstimatorEl: HTMLSpanElement | null = $state(null);

    async function fetchSequent(
        client: Client,
        proof: ProofId,
        node: NodeId,
        width: number,
    ) {
        const options: PrintOptions = {
            unicode: false,
            width,
            indentation: 0,
            pure: false,
            termLabels: true,
        };

        const seq = await client.goalPrint(node, options);
        return seq;
    }

    $effect(() => {
        if (!contentEl || !charWidthEstimatorEl) return;

        const ro = new ResizeObserver((entries) => {
            const px = entries[0].contentRect.width;
            const charWidth = charWidthEstimatorEl?.offsetWidth!;
            const newWidth = Math.floor(px / charWidth);

            console.debug("resize to new width: " + newWidth);
            width = newWidth;
        });

        ro.observe(contentEl);
        return () => ro.disconnect();
    });

    $effect(() => {
        if (appState.proof == null || appState.active_node == null) {
            return;
        }

        fetchSequent(
            appState.client,
            appState.proof,
            appState.active_node,
            width,
        ).then((seq) => {
            sequent = seq;
        });
    });
</script>

<div class="sequent-container">
    <h3>Sequent</h3>
    <div class="sequent-content" bind:this={contentEl}>
        <!-- This element is invisible and only exists so that we can extract
        the character width, regardless of font size.
        The character that we use for estimation does not matter; we use a monospace font.
        -->
        <code class="character-width-estimator" bind:this={charWidthEstimatorEl}
            >x</code
        >

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

    .character-width-estimator {
        top: 0;
        left: 0;
        position: absolute;
        visibility: hidden;
    }
</style>
