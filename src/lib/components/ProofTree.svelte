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

  function isActive(node: TreeNodeDesc) {
    return appState.active_node?.nodeId == node.id.nodeId;
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

    <!-- Close current subtree if depth decreased -->

    <li style="margin-left: {node.depth * 10}px;">
      <button
        class="node {statusFromName(node.node.name)} { isActive(node.node) ? "active" : ""}"
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
</style>
