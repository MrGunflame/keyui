<script lang="ts">
  import type { TreeNodeDesc } from "../../routes/api";

  let { appState } = $props();

  // Demo nodes
  const demoNodes = [
    { label: "0: OPEN GOAL", status: "open", indent: 0, sequent: "⊢ (p → q) → q → p" },
    { label: "1: apply rule: impRight", status: "open", indent: 1, sequent: "p → q, q ⊢ p" },
    { label: "2: split", status: "open", indent: 2, sequent: null },
    { label: "3: CLOSED", status: "closed", indent: 3, sequent: "⊢ ⊤" },
    { label: "4: OPEN GOAL", status: "open", indent: 3, sequent: "q ⊢ p" },
  ];

  let realNodes = $state<TreeNodeDesc[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  function statusFromName(name: string) {
    const up = name.toUpperCase();
    if (up.includes("OPEN")) return "open";
    if (up.includes("CLOSED")) return "closed";
    return "unknown";
  }
  function isActive(n: TreeNodeDesc) {
  return appState.active_node?.nodeId === n.id.nodeId;
}

  async function loadReal() {
    error = null;
    realNodes = [];

    if (!appState.proof) return;

    loading = true;
    try {
      const root = await appState.client.proofTreeRoot(appState.proof);
      realNodes = [root];
      appState.active_node = root.id; // select root so Sequent panel updates
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    void loadReal();
  });
</script>

<div>
  <h3>Proof Tree</h3>

  {#if !appState.proof}
    {#each demoNodes as n}
      <div class="node {n.status}" style="margin-left:{n.indent * 14}px">
        <div class="title">{n.label}</div>
      </div>
    {/each}
  {:else}
    {#if loading}
      <p style="opacity:0.7;">Loading proof tree…</p>
    {:else if error}
      <p style="color:#ff6b6b;">Error: {error}</p>
    {:else if realNodes.length === 0}
      <p style="opacity:0.7;">No nodes returned.</p>
    {:else}
      {#each realNodes as n}
        <div
          class="node {statusFromName(n.name)} {isActive(n)?'active' : ''}"
          onclick={() => (appState.active_node = n.id)}
        >
          <div class="title">{n.id.nodeId}:{n.name}</div>
        </div>
      {/each}
    {/if}
  {/if}
</div>

<style>
  .node { padding: 8px; margin: 6px 0; border-radius: 6px; background: #2b2b2b; cursor: pointer; }
  .open { background: #662222; }
  .closed { background: #225522; }
  .unknown { background: #333; }
  .title { font-weight: 600; }
  .active { outline: 2px solid #ffffff33; }
</style>










