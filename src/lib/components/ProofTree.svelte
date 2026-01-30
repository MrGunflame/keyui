<script lang="ts">
  import type { TreeNodeDesc } from "../../routes/api";

  let { appState } = $props();

  type Node = {
    node: TreeNodeDesc;
    depth: number;
  };

  let nodes = $state<Node[]>([]);
  
  type CtxMenuState = {
    open: boolean;
    x: number;
    y: number;
    node: TreeNodeDesc | null;
  };

  let ctxMenu = $state<CtxMenuState>({
    open: false,
    x: 0,
    y: 0,
    node: null,
  });

  function openCtxMenu(e: MouseEvent, node: TreeNodeDesc) {
    e.preventDefault();
    ctxMenu = {
      open: true,
      x: e.clientX,
      y: e.clientY,
      node,
    };
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

<div>
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
        onclick={(e) => e.stopPropagation()}
      >
        <div class="ctx-title">Node {ctxMenu.node?.id.nodeId}</div>
        <button class="ctx-item" disabled>Action A</button>
        <button class="ctx-item" disabled> Action B</button>
        <button class="ctx-item" disabled> Action C</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .node-list {
    height: 100%;
    overflow-y: auto;
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .node {
    width: 100%;
    text-align: left;

    color: var(--c-text);
    border: 1px solid var(--c-border);
    padding: 8px 10px;
    margin: 6px 0;
    border-radius: 8px;

    background: var(--c-node);  
    font-weight: 600;
    cursor: pointer;
  }

  .node:hover {
    border-color: var(--c-border-hover);
  }


  .node.open{ background: var(--c-node-open);}
  .node.closed{ 
    background: var(--c-node-closed);
    opacity: 0.55;
  }
  .node.unknown{ background: var(--c-node-unknwon);}
  
  .ctx-backdrop {
    position: fixed;
    inset: 0;
    z-index: 999;
  }

  .ctx-menu {
    position: fixed;
    z-index: 1000;
    min-width: 220px;
    background: var(--c-panel-2);
    border: 1px solid var(--c-border);
    border-radius: 8px;
    padding: 8px;
    box-shadow: 0 10px 25px rgba(0,0,0,0.4);
  }

  .ctx-title {
    font-size: 12px;
    opacity: 0.8;
    padding: 6px 8px;
    border-bottom: 1px solid var(--c-border);
    margin-bottom: 6px;
  }

  .ctx-item {
    width: 100%;
    text-align: left;
    padding: 8px;
    border: 0;
    background: transparent;
    color: var(--c-text);
    cursor: not-allowed;
    border-radius: 6px;
    opacity: 0.7;
  }

  .ctx-item:hover {
    background:var(--c-hover-bg);
  }

  /* Active node = very visible */
  .node.active {
    outline: 2px solid var(--c-active-outline);
    outline-offset: 2px;
  }

  /* leaf vs internal */
  .node.leaf {
    border-left: 6px solid var(--c-border);
  }

  .node.internal {
    border-left: 6px solid var(--c-border-hover);
  }

  /* active + closed should still be readable */
  .node.closed.active {
    opacity: 0.9;
  }
</style>
