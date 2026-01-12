<script lang="ts">
  import type { TreeNodeDesc } from "../../routes/api";

  let { appState } = $props();

  type Node = {
    node: TreeNodeDesc;
    depth: number;
  };

  let nodes = $state<Node[]>([]);

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
  // Set to false before final PR if your team prefers no demo data.
  const DEMO_TREE = true;

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

  async function loadTree(client, proof) {
    let nodes = [];
    let stack = [];

    let root = await client.proofTreeRoot(proof);
    nodes.push({ node: root, depth: 0 });
    stack.push({ id: root.id, depth: 0 });

    while (stack.length != 0) {
      let { id, depth } = stack.pop();
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
        >
          {node.node.id.nodeId}: {node.node.name}
        </button>
      </li>
    {/each}
  </ul>
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
