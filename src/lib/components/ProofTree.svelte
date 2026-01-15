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

  function isActive(node: TreeNodeDesc) {
    return appState.active_node?.nodeId == node.id.nodeId;
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

  $effect(() => {
    if (appState.proof == null) {
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

    <li style="margin-left: {node.depth * 10}px;">
      <button
        class="node {statusFromName(node.node.name)} { isActive(node.node) ? "active" : ""}"
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
    display: block;
    color: white;
    background: transparent;
    border: 0;
    padding: 8px;
    margin: 6px 0;
    border-radius: 6px;
    background: #2b2b2b;
    font-weight: 600;
  }

  .active {
    border: 2px solid green;
  }

  .open { background: #662222; }
  .closed { background: #225522; }
  .unknown { background: #333; }
  
  .ctx-backdrop {
    position: fixed;
    inset: 0;
    z-index: 999;
  }

  .ctx-menu {
    position: fixed;
    z-index: 1000;
    min-width: 220px;
    background: #1f1f1f;
    border: 1px solid #444;
    border-radius: 8px;
    padding: 8px;
    box-shadow: 0 10px 25px rgba(0,0,0,0.4);
  }

  .ctx-title {
    font-size: 12px;
    opacity: 0.8;
    padding: 6px 8px;
    border-bottom: 1px solid #333;
    margin-bottom: 6px;
  }

  .ctx-item {
    width: 100%;
    text-align: left;
    padding: 8px;
    border: 0;
    background: transparent;
    color: white;
    cursor: not-allowed;
    border-radius: 6px;
    opacity: 0.7;
  }

  .ctx-item:hover {
    background: #2b2b2b;
  }
</style>
