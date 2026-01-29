<script lang="ts">
  import type { TreeNodeDesc } from "../../routes/api";

  let { appState } = $props();

  type Node = {
    node: TreeNodeDesc;
    depth: number;
  };

  let nodes = $state<Node[]>([]);
  
  //State for the context menu
  type CtxMenuState = {
    open: boolean;
    x: number;
    y: number;
    node: TreeNodeDesc | null;
    appliedOn: string | null;
    loading: boolean;
    error: string | null;
  };

  let ctxMenu = $state<CtxMenuState>({
    open: false,
    x: 0,
    y: 0,
    node: null,
    appliedOn: null,
    loading: false,
    error: null,
  });

  //check if a node is a closed goal
  function isClosedGoal(node: TreeNodeDesc | null) {
    if (!node) return false;
    return node.name.toLowerCase().includes("closed goal");
  }

  //Fetches the sequent for a given node using the goal/print API
  async function fetchAppliedOn(nodeId:any) {
    const options={
      unicode: false,
      width: 120,
      indentation: 0,
      pure: false,
      termLabels: true,
    };

    //goalPrint takes a NodeId
    const res= await appState.client.goalPrint(nodeId,options);
    return res.result as string; 
  }

  //Opens the context menu when user right-clicks a node
  function openCtxMenu(e: MouseEvent, node: TreeNodeDesc) {
    e.preventDefault();

    // If it's a closed goal: don't fetch anything
    if (isClosedGoal(node)) {
      ctxMenu = {
        open: true,
        x: e.clientX,
        y: e.clientY,
        node,
        appliedOn: null,
        loading: false,
        error: null,
      };
      return;
    }

    //Normal nodes : fetch "AppliedOn" using goalPrint
    ctxMenu = {
      open: true,
      x: e.clientX,
      y: e.clientY,
      node,
      appliedOn: null,
      loading: true,
      error:null,
    };

    //Fetch the sequent in the background
    fetchAppliedOn(node.id).then(text=> {
      if(!ctxMenu.open || ctxMenu.node?.id.nodeId !== node.id.nodeId) return;

      ctxMenu.appliedOn = text;
      ctxMenu.loading = false;
    }).catch(err => {
      if (!ctxMenu.open || ctxMenu.node?.id.nodeId !==node.id.nodeId) return;

      ctxMenu.error = err?.toString?.() ?? "Unknown error";
      ctxMenu.loading = false;
    });

  }

  function closeCtxMenu() {
    ctxMenu.open = false;
  }

  function statusFromName(name: string) {
    const up = name.toUpperCase();
    if (up.includes("OPEN")) return "open";
    if (up.includes("CLOSED")) return "closed";
    return "unknown";
  }

  // Leaf vs internal (computed from list order + depth)
  function isLeaf(index: number) {
    const currentDepth = nodes[index]?.depth ?? 0;
    const nextDepth = nodes[index + 1]?.depth ?? -1;
    return nextDepth <= currentDepth;
  }

  function isActive(node: TreeNodeDesc) {
    return appState.active_node?.nodeId == node.id.nodeId;
  }

  // DEV ONLY: show a small fake tree when no proof is loaded (so we can work on UI)
  const DEMO_TREE = false;

  function makeDemoNodes(): Node[] {
    const fake = (nodeId: number, name: string) =>
      ({ id: { nodeId }, name } as unknown as TreeNodeDesc);

    return [
      { node: fake(0, "OPEN Root"), depth: 0 },
      { node: fake(1, "OPEN Internal A"), depth: 1 },
      { node: fake(2, "CLOSED Leaf A1"), depth: 2 },
      { node: fake(3, "OPEN Leaf A2"), depth: 2 },
      { node: fake(4, "CLOSED Internal B"), depth: 1 },
      { node: fake(5, "CLOSED Leaf B1"), depth: 2 },
    ];
  }

  async function loadTree(client:any , proof:any) {
    let nodes = [];
    let stack: { id: any; depth: number }[] = [];


    let root = await client.proofTreeRoot(proof);
    nodes.push({ node: root, depth: 0 });
    stack.push({ id: root.id, depth: 0 });

    while (stack.length != 0) {
      let { id, depth } = stack.pop()!;
      let elems = await client.proofTreeChildren(proof, id);

      for (const elem of elems) {
        nodes.push({ node: elem, depth: depth + 1 });
        stack.push({ id: elem.id, depth: depth + 1 });
      }
    }

    return nodes;
  }

  // Rerender whenever proof tree change signal is received.
  let waker = appState.proofTreeChanged.subscribe();

  $effect(() => {
    $waker;
    
    if (appState.proof == null) {
      if (DEMO_TREE) {
        nodes = makeDemoNodes();
      }
      return;
    }

    loadTree(appState.client, appState.proof).then(n => {
      nodes = n;
    });
  });
</script>

<div class="proof-tree-container">
  <h3>Proof Tree</h3>
  <ul class="node-list">
    {#each nodes as node, index}
      <li style="margin-left: {node.depth * 14}px;">
        <button
          class="node {statusFromName(node.node.name)} {isActive(node.node) ? "active" : ""} {isLeaf(index) ? "leaf" : "internal"}"
          onclick={() => (appState.active_node = node.node.id)}
        oncontextmenu={(e)=>openCtxMenu(e,node.node)}
        >
          {node.node.id.nodeId}: {node.node.name}
        </button>
      </li>
    {/each}
  </ul>
  {#if ctxMenu.open}
  <div class="ctx-backdrop" onclick={closeCtxMenu}>
    <div
      class="ctx-menu"
      style="left:{ctxMenu.x}px; top:{ctxMenu.y}px;"
      onclick={(e) => e.stopPropagation()}
    >
      {#if ctxMenu.node?.name?.toLowerCase() === "closed goal"}
        
        <div class="ctx-simple">A closed goal</div>
      {:else}
      
        <div class="ctx-title">Taclet info</div>

        <div class="ctx-content">
          <div class="ctx-row">
            <div class="ctx-label">Rule</div>
            <div class="ctx-value">{ctxMenu.node?.name ?? "-"}</div>
          </div>

          <div class="ctx-sep"></div>

          <div class="ctx-label">Applied on</div>

          {#if ctxMenu.loading}
            <div class="ctx-mono loading">Loading…</div>
          {:else if ctxMenu.error}
            <div class="ctx-mono error">{ctxMenu.error}</div>
          {:else}
            <div class="ctx-mono">{ctxMenu.appliedOn ?? "-"}</div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
{/if}

</div>

<style>
    .proof-tree-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }
  .proof-tree-container h3 {
    margin: 0 0 10px 0;
    flex-shrink: 0;
  }
  .node-list {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .node {
    width: 100%;
    text-align: left;
    word-wrap: break-word;
    white-space: normal;

    color: white;
    border: 1px solid rgba(255, 255, 255, 0.08);
    padding: 8px 10px;
    margin: 6px 0;
    border-radius: 8px;

    background: #2b2b2b;
    font-weight: 600;
    cursor: pointer;
    transition: border-color 120ms ease, transform 120ms ease;
  }

  .node:hover {
    border-color: rgba(255, 255, 255, 0.22);
    transform: translateY(-1px);
  }

  .open { background: #662222; }
  .closed { background: #225522; }
  .unknown { background: #333; }
  
  .ctx-backdrop {
    position: fixed;
    inset: 0;
    z-index: 999;
  }
  .ctx-simple {
  padding: 10px 12px;
  font-size: 13px;
  font-weight: 600;
  opacity: 0.95;
  white-space: nowrap;
}

  .ctx-menu {
    position: fixed;
    z-index: 1000;
    min-width: 260px;
    max-width: 420px;
    background:rgba(20, 20, 20, 0.92);
    border: 1px solid rgba(255, 255, 255, 0.10);
    border-radius: 12px;
    padding: 10px 12px;
    box-shadow: 0 18px 45px rgba(0,0,0,0.55);
    backdrop-filter: blur(10px);
    transform: translate(8px,8px);
  }

  .ctx-title {
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.2px;
    opacity: 0.9;
    padding: 6px 2px 10px 2px;
    border-bottom: 1px solid rgba(255,255,255,0.08);
    margin-bottom: 10px;
  }

  .ctx-content {
  display: grid;
  gap: 10px;
}

.ctx-row {
  display: grid;
  grid-template-columns: 92px 1fr;
  gap: 10px;
  align-items: baseline;
}

.ctx-label {
  font-size: 12px;
  opacity: 0.7;
}

.ctx-value {
  font-size: 13px;
  font-weight: 650;
}

.ctx-sep {
  height: 1px;
  background: rgba(255,255,255,0.08);
  margin: 2px 0;
}

.ctx-mono {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 12px;
  line-height: 1.35;

  white-space: pre-wrap;
  word-break: break-word;

  padding: 8px 10px;
  border-radius: 10px;
  background: rgba(255,255,255,0.06);
  border: 1px solid rgba(255,255,255,0.08);
}

.ctx-mono.loading {
  opacity: 0.75;
}

.ctx-mono.error {
  border-color: rgba(255, 120, 120, 0.35);
  background: rgba(255, 120, 120, 0.10);
}

  /* Active node = very visible */
  .node.active {
    outline: 2px solid rgba(80, 200, 120, 0.95);
    outline-offset: 2px;
  }

  /* open / closed / unknown */
  .node.open {
    background: #6a2525;
  }

  .node.closed {
    background: #1f4f2a;
    opacity: 0.55; /* closed nodes look processed */
  }

  .node.unknown {
    background: #333;
  }

  /* leaf vs internal */
  .node.leaf {
    border-left: 6px solid rgba(255, 255, 255, 0.16);
  }

  .node.internal {
    border-left: 6px solid rgba(255, 255, 255, 0.34);
  }

  /* active + closed should still be readable */
  .node.closed.active {
    opacity: 0.9;
  }
</style>