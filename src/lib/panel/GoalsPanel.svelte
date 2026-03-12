<script lang="ts">
    import type { ProofId, TreeNodeDesc, NodeId } from "$lib/api";

    let { appState } = $props();

    let openGoals = $state<TreeNodeDesc[]>([]);
    let loading = $state(false);

    // Rerun when proof tree changes (load, auto proof, etc.)
    const proofTreeWaker = appState.proofTreeChanged.subscribe();

    async function collectOpenGoalNodesFromTree(
        client: any,
        proof: ProofId,
    ): Promise<TreeNodeDesc[]> {
        const result: TreeNodeDesc[] = [];

        const root: TreeNodeDesc = await client.proofTreeRoot(proof);

        // BFS/DFS over the proof tree using proofTree/children
        const stack: TreeNodeDesc[] = [root];
        const visited = new Set<string>();

        while (stack.length > 0) {
            const node = stack.pop()!;
            const key = node.id.nodeId;
            if (visited.has(key)) continue;
            visited.add(key);

            const name = (node.name ?? "").toLowerCase();
            if (name.includes("open goal")) {
                result.push(node);
            }

            // fetch children and continue
            const children = await client.proofTreeChildren(proof, node.id);
            for (const c of children) stack.push(c);
        }

        return result;
    }

    async function reload() {
        if (!appState.proof) {
            openGoals = [];
            return;
        }

        loading = true;
        try {
            openGoals = await collectOpenGoalNodesFromTree(
                appState.client,
                appState.proof,
            );
        } finally {
            loading = false;
        }
    }

    $effect(() => {
        $proofTreeWaker; // track signal
        reload();
    });

    function selectNode(id: NodeId) {
        // clicking an open goal selects it in the app
        appState.active_node = id;
    }
</script>

<h3>Goals</h3>

{#if loading}
    <div>Loading…</div>
{:else}
    <div>Open goals: {openGoals.length}</div>

    {#if openGoals.length === 0}
        <div class="empty">No open goals found.</div>
    {:else}
        <ul>
            {#each openGoals as g}
                <li class="goal" on:click={() => selectNode(g.id)}>
                    {g.name}
                </li>
            {/each}
        </ul>
    {/if}
{/if}

<style>
    .panel h3 {
        margin-top: 0;
    }

    .panel ul {
        margin: 0;
        padding-left: 18px;
    }

    .goal {
        margin: 4px 0;
        word-break: break-word;
        cursor: pointer;
    }

    .goal:hover {
        text-decoration: underline;
    }

    .empty {
        opacity: 0.8;
        font-style: italic;
        margin-top: 6px;
    }
</style>
