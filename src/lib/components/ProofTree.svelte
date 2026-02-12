<script lang="ts">
  import type { TreeNodeDesc } from "../../routes/api";

  let { appState } = $props();

  type Node = {
    node: TreeNodeDesc;
    depth: number;
  };

  let nodes = $state<Node[]>([]);
  let searchQuery = $state("");
  let collapsedNodes = $state<Set<string>>(new Set());
  
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

  function toggleCollapse(nodeId: string) {
    if (collapsedNodes.has(nodeId)) {
      collapsedNodes.delete(nodeId);
    } else {
      collapsedNodes.add(nodeId);
    }
    collapsedNodes = new Set(collapsedNodes); 
  }

  function isCollapsed(nodeId: string) {
    return collapsedNodes.has(nodeId);
  }

  function isHiddenByCollapse(index: number): boolean {
 const currentNode = nodes[index];
  if (!currentNode) return false;

  
  for (let i = index - 1; i >= 0; i--) {
    const ancestor = nodes[i];
    
    
    if (ancestor.depth < currentNode.depth) {
      const ancestorNodeId = ancestor.node.id.nodeId;
      if (collapsedNodes.has(ancestorNodeId)) {
        return true;
      }
    }
  }
  
  return false;
  }

  function matchesSearch(node: TreeNodeDesc): boolean {
    if (!searchQuery.trim()) return true;
    
    const query = searchQuery.toLowerCase();
    const nodeName = node.name.toLowerCase();
    const nodeId = node.id.nodeId.toString();
    
    return nodeName.includes(query) || nodeId.includes(query);
  }

  function hasMatchingDescendant(index: number): boolean {
    if (!searchQuery.trim()) return false;
    
    const currentDepth = nodes[index]?.depth ?? 0;
    
    for (let i = index + 1; i < nodes.length; i++) {
      if (nodes[i].depth <= currentDepth) break;
      if (matchesSearch(nodes[i].node)) return true;
    }
    return false;
  }

  function shouldShowNode(index: number): boolean {
    const node = nodes[index];
    if (!node) return false;

    
    if (isHiddenByCollapse(index)) return false;

    
    if (!searchQuery.trim()) return true;

    
    return matchesSearch(node.node) || hasMatchingDescendant(index);
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

  <div class="search-container">
    <input
      type="text"
      class="search-input"
      placeholder="Suche nach Name oder ID..."
      bind:value={searchQuery}
    />
    {#if searchQuery}
      <button class="clear-btn" onclick={() => searchQuery = ""}>✕</button>
    {/if}
  </div>

  <ul class="node-list">
    {#each nodes as node, index}
    {#if shouldShowNode(index)}
        <li style="margin-left: {node.depth * 14}px;">
          <button
            class="node {statusFromName(node.node.name)} {isActive(node.node) ? 'active' : ''} {isLeaf(index) ? 'leaf' : 'internal'}"
            onclick={() => (appState.active_node = node.node.id)}
            oncontextmenu={(e) => openCtxMenu(e, node.node)}
          >
          {#if !isLeaf(index)}
              <span 
                class="collapse-icon"
                onclick={(e) => {
                  e.stopPropagation();
                  toggleCollapse(node.node.id.nodeId);
                }}
              >
                {isCollapsed(node.node.id.nodeId) ? '▶' : '▼'}
              </span>
            {/if}
            <span class="node-content">
              {node.node.id.nodeId}: {node.node.name}
            </span>
          </button>
        </li>
      {/if}
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
<<<<<<< Updated upstream
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
=======
  .proof-tree-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 0 10px;
  }
  .search-container {
    position: relative;
    margin: 10px 0;
  }
  .search-input {
    width: 100%;
    padding: 10px 35px 10px 12px;
    background: #2b2b2b;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    color: white;
    font-size: 14px;
    box-sizing: border-box;
  }
  .search-input:focus {
    outline: none;
    border-color: rgba(80, 200, 120, 0.5);
  }

  .search-input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }

  .clear-btn {
    position: absolute;
    right: 8px;
    top: 50%;
    transform: translateY(-50%);
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.5);
    cursor: pointer;
    padding: 4px 8px;
    font-size: 16px;
  }

  .clear-btn:hover {
    color: white;
  }



>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
    word-wrap: break-word;
    white-space: normal;
=======
    display: flex;
    align-items: center;
    gap: 8px;

>>>>>>> Stashed changes

    color: white;
    border: 1px solid rgba(255, 255, 255, 0.08);
    padding: 8px 10px;
    margin: 6px 0;
    border-radius: 8px;

    background: #2b2b2b;
    font-weight: 600;
    cursor: pointer;
  }

  .node:hover {
    border-color: rgba(255, 255, 255, 0.22);
  }

  .collapse-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    font-size: 10px;
    opacity: 0.7;
    transition: opacity 0.2s;
  }

  .collapse-icon:hover {
    opacity: 1;
  }

   .node-content {
    flex: 1;
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
